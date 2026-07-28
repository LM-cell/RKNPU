//! NS16550/16450 UART 驱动模块
//!
//! 提供两种访问方式：
//! - IO Port 版本（x86_64 架构）
//! - MMIO 版本（通用嵌入式平台）

// 公共寄存器定义
mod registers;

use bitflags::Flags;
use heapless::Deque;
use rdif_serial::{
    Config, ConfigError, DataBits, InterruptMask, LineStatus, Parity, Register, Serial, StopBits,
    TransferError,
};
use registers::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod pio;
// MMIO 版本（通用）
mod mmio;

pub use mmio::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use pio::*;

pub trait Kind: Send + Sync + 'static {
    fn read_reg(&self, reg: u8) -> u8;
    fn write_reg(&mut self, reg: u8, val: u8);
    fn get_base(&self) -> usize;
    fn set_base(&mut self, base: usize);
}

#[derive(Clone, Debug)]
#[repr(align(64))]
pub struct Ns16550<T: Kind> {
    rcv_fifo: Deque<u8, 64>,
    base: T,
    clock_freq: u32,
    is_tx_empty_int_enabled: bool,
    err: Option<TransferError>,
}

impl<T: Kind> Ns16550<T> {
    fn new(base: T, clock_freq: u32) -> Serial<Self> {
        Serial::new(Self {
            rcv_fifo: Deque::new(),
            base,
            clock_freq,
            is_tx_empty_int_enabled: false,
            err: None,
        })
    }

    // 基础 u8 寄存器访问（用于除数寄存器等特殊场景）
    fn read_reg_u8(&self, reg: u8) -> u8 {
        self.base.read_reg(reg)
    }

    fn write_reg_u8(&mut self, reg: u8, val: u8) {
        self.base.write_reg(reg, val);
    }

    // 类型安全的 bitflags 寄存器访问
    fn read_flags<F: Flags<Bits = u8>>(&self, reg: u8) -> F {
        F::from_bits_retain(self.base.read_reg(reg))
    }

    fn write_flags<F: Flags<Bits = u8>>(&mut self, reg: u8, val: F) {
        self.base.write_reg(reg, val.bits());
    }

    /// 检查是否为 16550+（支持 FIFO）
    pub fn is_16550_plus(&self) -> bool {
        // 通过读取 IIR 寄存器的 FIFO 位来判断
        // IIR 的位7-6在 16550+ 中会显示 FIFO 启用状态
        let fifo: InterruptIdentificationFlags = self.read_flags(UART_IIR);
        fifo.contains(InterruptIdentificationFlags::FIFO_ENABLE_MASK)
    }

    /// 设置波特率
    fn set_baudrate_internal(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        if baudrate == 0 || self.clock_freq == 0 {
            return Err(ConfigError::InvalidBaudrate);
        }

        let divisor = self.clock_freq / (16 * baudrate);
        if divisor == 0 || divisor > 0xFFFF {
            return Err(ConfigError::InvalidBaudrate);
        }

        // 保存原始 LCR
        let mut lcr: LineControlFlags = self.read_flags(UART_LCR);

        // 设置 DLAB 以访问波特率除数寄存器
        lcr.insert(LineControlFlags::DIVISOR_LATCH_ACCESS);
        self.write_flags(UART_LCR, lcr);

        // 设置除数（使用 u8 方法，因为这是原始数据写入）
        self.write_reg_u8(UART_DLL, (divisor & 0xFF) as u8);
        self.write_reg_u8(UART_DLH, ((divisor >> 8) & 0xFF) as u8);

        // 清除 DLAB 位，恢复正常访问
        lcr.remove(LineControlFlags::DIVISOR_LATCH_ACCESS);
        self.write_flags(UART_LCR, lcr);

        Ok(())
    }

    /// 设置数据位
    fn set_data_bits_internal(&mut self, bits: DataBits) -> Result<(), ConfigError> {
        let wlen = match bits {
            DataBits::Five => LineControlFlags::WORD_LENGTH_5,
            DataBits::Six => LineControlFlags::WORD_LENGTH_6,
            DataBits::Seven => LineControlFlags::WORD_LENGTH_7,
            DataBits::Eight => LineControlFlags::WORD_LENGTH_8,
        };

        let mut lcr: LineControlFlags = self.read_flags(UART_LCR);
        // 清除旧的数据位设置，然后设置新的
        lcr.remove(LineControlFlags::WORD_LENGTH_MASK);
        lcr.insert(wlen);
        self.write_flags(UART_LCR, lcr);

        Ok(())
    }

    /// 设置停止位
    fn set_stop_bits_internal(&mut self, bits: StopBits) -> Result<(), ConfigError> {
        let mut lcr: LineControlFlags = self.read_flags(UART_LCR);
        match bits {
            StopBits::One => lcr.remove(LineControlFlags::STOP_BITS),
            StopBits::Two => lcr.insert(LineControlFlags::STOP_BITS),
        }
        self.write_flags(UART_LCR, lcr);
        Ok(())
    }

    /// 设置奇偶校验
    fn set_parity_internal(&mut self, parity: Parity) -> Result<(), ConfigError> {
        let mut lcr: LineControlFlags = self.read_flags(UART_LCR);

        // 先清除所有校验相关位
        lcr.remove(
            LineControlFlags::PARITY_ENABLE
                | LineControlFlags::EVEN_PARITY
                | LineControlFlags::STICK_PARITY,
        );

        // 根据校验类型设置相应位
        match parity {
            Parity::None => {
                // 已经清除，无需额外操作
            }
            Parity::Odd => {
                lcr.insert(LineControlFlags::PARITY_ENABLE);
            }
            Parity::Even => {
                lcr.insert(LineControlFlags::PARITY_ENABLE | LineControlFlags::EVEN_PARITY);
            }
            Parity::Mark => {
                lcr.insert(LineControlFlags::PARITY_ENABLE | LineControlFlags::STICK_PARITY);
            }
            Parity::Space => {
                lcr.insert(
                    LineControlFlags::PARITY_ENABLE
                        | LineControlFlags::EVEN_PARITY
                        | LineControlFlags::STICK_PARITY,
                );
            }
        }

        self.write_flags(UART_LCR, lcr);
        Ok(())
    }

    /// 启用或禁用 FIFO
    pub fn enable_fifo(&mut self, enable: bool) {
        if enable && self.is_16550_plus() {
            let mut fcr = FifoControlFlags::ENABLE_FIFO;
            fcr.insert(FifoControlFlags::CLEAR_RECEIVER_FIFO);
            fcr.insert(FifoControlFlags::CLEAR_TRANSMITTER_FIFO);
            fcr.insert(FifoControlFlags::TRIGGER_1_BYTE);
            self.write_flags(UART_FCR, fcr);
        } else {
            self.write_flags(UART_FCR, FifoControlFlags::empty());
        }
    }

    /// 设置 FIFO 触发级别
    pub fn set_fifo_trigger_level(&mut self, level: u8) {
        if !self.is_16550_plus() {
            return;
        }

        let trigger_value = match level {
            0..=3 => FifoControlFlags::TRIGGER_1_BYTE,
            4..=7 => FifoControlFlags::TRIGGER_4_BYTES,
            8..=11 => FifoControlFlags::TRIGGER_8_BYTES,
            _ => FifoControlFlags::TRIGGER_14_BYTES,
        };

        // 读取当前 FCR 设置，清除触发级别位，然后设置新的触发级别
        let mut fcr: FifoControlFlags = self.read_flags(UART_FCR);
        fcr.remove(FifoControlFlags::TRIGGER_LEVEL_MASK);
        fcr.insert(trigger_value);
        self.write_flags(UART_FCR, fcr);
    }

    /// 初始化 UART
    fn init(&mut self) {
        // 禁用所有中断
        self.write_flags(UART_IER, InterruptEnableFlags::empty());

        // 确保传输器启用（设置 DTR 和 RTS）
        let mut mcr: ModemControlFlags = self.read_flags(UART_MCR);
        mcr.insert(ModemControlFlags::DATA_TERMINAL_READY | ModemControlFlags::REQUEST_TO_SEND);
        self.write_flags(UART_MCR, mcr);
    }

    /// 清空接收 FIFO
    pub fn clear_receive_fifo(&mut self) {
        if self.is_16550_plus() {
            let mut fcr = FifoControlFlags::ENABLE_FIFO;
            fcr.insert(FifoControlFlags::CLEAR_RECEIVER_FIFO);
            self.write_flags(UART_FCR, fcr);
        }
        self.rcv_fifo.clear();
    }

    /// 清空发送 FIFO
    pub fn clear_transmit_fifo(&mut self) {
        if self.is_16550_plus() {
            let mut fcr = FifoControlFlags::ENABLE_FIFO;
            fcr.insert(FifoControlFlags::CLEAR_TRANSMITTER_FIFO);
            self.write_flags(UART_FCR, fcr);
        }
    }

    /// 检查 FIFO 是否启用
    pub fn is_fifo_enabled(&self) -> bool {
        if !self.is_16550_plus() {
            return false;
        }
        // 通过检查 IIR 的 FIFO 位来判断
        let iir: InterruptIdentificationFlags = self.read_flags(UART_IIR);
        iir.contains(InterruptIdentificationFlags::FIFO_ENABLE_MASK)
    }
}

impl<T: Kind> Register for Ns16550<T> {
    fn write_byte(&mut self, byte: u8) {
        self.write_reg_u8(UART_THR, byte);
        // 如果启用了发送中断模式，在写入数据后重新启用 THRE 中断
        // 这样当 THR 的数据被转移到 TSR 后会再次触发中断
        if self.is_tx_empty_int_enabled {
            let mut ier: InterruptEnableFlags = self.read_flags(UART_IER);
            ier.insert(InterruptEnableFlags::TRANSMITTER_HOLDING_EMPTY);
            self.write_flags(UART_IER, ier);
        }
    }

    fn read_byte(&mut self) -> Result<u8, TransferError> {
        if let Some(e) = self.err.take() {
            return Err(e);
        }
        Ok(self
            .rcv_fifo
            .pop_front()
            .expect("should check line status first"))
    }

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        // 配置波特率
        if let Some(baudrate) = config.baudrate {
            self.set_baudrate_internal(baudrate)?;
        }

        // 配置数据位
        if let Some(data_bits) = config.data_bits {
            self.set_data_bits_internal(data_bits)?;
        }

        // 配置停止位
        if let Some(stop_bits) = config.stop_bits {
            self.set_stop_bits_internal(stop_bits)?;
        }

        // 配置奇偶校验
        if let Some(parity) = config.parity {
            self.set_parity_internal(parity)?;
        }

        Ok(())
    }

    fn baudrate(&self) -> u32 {
        // 只读方式获取波特率，通过读取 DLL 和 DLH
        // 注意：如果 DLAB 未设置，读取的可能不是除数值
        let dll = self.read_reg_u8(UART_DLL) as u16;
        let dlh = self.read_reg_u8(UART_DLH) as u16;
        let divisor = dll | (dlh << 8);

        if divisor == 0 {
            return 0;
        }

        self.clock_freq / (16 * divisor as u32)
    }

    fn data_bits(&self) -> DataBits {
        let lcr: LineControlFlags = self.read_flags(UART_LCR);
        let wlen = lcr & LineControlFlags::WORD_LENGTH_MASK;
        if wlen == LineControlFlags::WORD_LENGTH_5 {
            DataBits::Five
        } else if wlen == LineControlFlags::WORD_LENGTH_6 {
            DataBits::Six
        } else if wlen == LineControlFlags::WORD_LENGTH_7 {
            DataBits::Seven
        } else {
            DataBits::Eight // 默认值
        }
    }

    fn stop_bits(&self) -> StopBits {
        let lcr: LineControlFlags = self.read_flags(UART_LCR);
        if lcr.contains(LineControlFlags::STOP_BITS) {
            StopBits::Two
        } else {
            StopBits::One
        }
    }

    fn parity(&self) -> Parity {
        let lcr: LineControlFlags = self.read_flags(UART_LCR);

        if !lcr.contains(LineControlFlags::PARITY_ENABLE) {
            Parity::None
        } else if lcr.contains(LineControlFlags::STICK_PARITY) {
            // Stick parity
            if lcr.contains(LineControlFlags::EVEN_PARITY) {
                Parity::Space
            } else {
                Parity::Mark
            }
        } else {
            // Normal parity
            if lcr.contains(LineControlFlags::EVEN_PARITY) {
                Parity::Even
            } else {
                Parity::Odd
            }
        }
    }

    fn open(&mut self) {
        self.init();
    }

    fn close(&mut self) {
        // 禁用所有中断
        self.write_flags(UART_IER, InterruptEnableFlags::empty());

        // 禁用 DTR 和 RTS
        let mut mcr: ModemControlFlags = self.read_flags(UART_MCR);
        mcr.remove(ModemControlFlags::DATA_TERMINAL_READY | ModemControlFlags::REQUEST_TO_SEND);
        self.write_flags(UART_MCR, mcr);
    }

    fn clean_interrupt_status(&mut self) -> InterruptMask {
        let iir: InterruptIdentificationFlags = self.read_flags(UART_IIR);
        let mut mask = InterruptMask::empty();

        // 检查是否有中断挂起
        if iir.contains(InterruptIdentificationFlags::NO_INTERRUPT_PENDING) {
            return mask;
        }

        // 获取中断ID（需要提取bit 1-3）
        let interrupt_id = iir & InterruptIdentificationFlags::INTERRUPT_ID_MASK;

        // 使用精确匹配而不是 contains
        if interrupt_id == InterruptIdentificationFlags::RECEIVER_LINE_STATUS {
            // 接收线路状态错误中断
            let lsr: LineStatusFlags = self.read_flags(UART_LSR);

            // 读取 RBR 以清除错误状态（即使有错误也需要读取）
            let d = self.read_reg_u8(UART_RBR);

            // 按优先级检查错误（从高到低）
            if lsr.contains(LineStatusFlags::OVERRUN_ERROR) {
                self.err = Some(TransferError::Overrun(d));
                mask |= InterruptMask::RX_AVAILABLE;
            } else if lsr.contains(LineStatusFlags::PARITY_ERROR) {
                self.err = Some(TransferError::Parity);
                mask |= InterruptMask::RX_AVAILABLE;
            } else if lsr.contains(LineStatusFlags::FRAMING_ERROR) {
                self.err = Some(TransferError::Framing);
                mask |= InterruptMask::RX_AVAILABLE;
            } else if lsr.contains(LineStatusFlags::BREAK_INTERRUPT) {
                self.err = Some(TransferError::Break);
                mask |= InterruptMask::RX_AVAILABLE;
            } else if lsr.contains(LineStatusFlags::DATA_READY) {
                // 没有错误，保存数据到 FIFO
                if self.rcv_fifo.push_back(d).is_err() {
                    self.err = Some(TransferError::Overrun(d));
                }
                mask |= InterruptMask::RX_AVAILABLE;
            }
        } else if interrupt_id == InterruptIdentificationFlags::RECEIVED_DATA_AVAILABLE
            || interrupt_id == InterruptIdentificationFlags::CHARACTER_TIMEOUT
        {
            // 接收数据可用中断或字符超时中断
            mask |= InterruptMask::RX_AVAILABLE;

            // 读取所有可用数据
            while self
                .read_flags::<LineStatusFlags>(UART_LSR)
                .contains(LineStatusFlags::DATA_READY)
            {
                let d = self.read_reg_u8(UART_RBR);
                if self.rcv_fifo.push_back(d).is_err() {
                    self.err = Some(TransferError::Overrun(d));
                    break;
                }
            }
        } else if interrupt_id == InterruptIdentificationFlags::TRANSMITTER_HOLDING_EMPTY {
            // 发送保持寄存器空中断
            // 关闭 THRI 使能位，避免持续触发中断
            // 用户在 write_byte 时会重新启用
            let mut ier: InterruptEnableFlags = self.read_flags(UART_IER);
            ier.remove(InterruptEnableFlags::TRANSMITTER_HOLDING_EMPTY);
            self.write_flags(UART_IER, ier);
            if self.is_tx_empty_int_enabled {
                mask |= InterruptMask::TX_EMPTY;
            }
        } else if interrupt_id == InterruptIdentificationFlags::MODEM_STATUS {
            // Modem 状态中断，读取 MSR 清除
            let _ = self.read_flags::<ModemStatusFlags>(UART_MSR);
        }

        mask
    }

    fn line_status(&mut self) -> LineStatus {
        let lsr: LineStatusFlags = self.read_flags(UART_LSR);
        let mut status = LineStatus::empty();
        if self.err.is_some() {
            status |= LineStatus::DATA_READY;
        }

        if lsr.contains(LineStatusFlags::TRANSMITTER_HOLDING_EMPTY) {
            status |= LineStatus::TX_HOLDING_EMPTY;
        }

        if self
            .read_flags::<InterruptEnableFlags>(UART_IER)
            .contains(InterruptEnableFlags::RECEIVED_DATA_AVAILABLE)
        {
            // 检查 FIFO 中是否有数据
            if !self.rcv_fifo.is_empty() {
                status |= LineStatus::DATA_READY;
            }
        } else {
            // 如果未启用接收中断，则直接检查 LSR 的 DATA_READY 位
            if lsr.contains(LineStatusFlags::DATA_READY) {
                let b = self.read_reg_u8(UART_RBR);
                if self.rcv_fifo.push_back(b).is_err() {
                    self.err = Some(TransferError::Overrun(b));
                }
                status |= LineStatus::DATA_READY;
            }
        }

        status
    }

    fn read_reg(&self, offset: usize) -> u32 {
        self.read_reg_u8(offset as u8) as u32
    }

    fn write_reg(&mut self, offset: usize, value: u32) {
        self.write_reg_u8(offset as u8, value as u8);
    }

    fn get_base(&self) -> usize {
        self.base.get_base()
    }

    fn set_base(&mut self, base: usize) {
        self.base.set_base(base);
    }

    fn clock_freq(&self) -> u32 {
        self.clock_freq
    }

    fn enable_loopback(&mut self) {
        let mut mcr: ModemControlFlags = self.read_flags(UART_MCR);
        mcr.insert(ModemControlFlags::LOOPBACK_ENABLE);
        self.write_flags(UART_MCR, mcr);
    }

    fn disable_loopback(&mut self) {
        let mut mcr: ModemControlFlags = self.read_flags(UART_MCR);
        mcr.remove(ModemControlFlags::LOOPBACK_ENABLE);
        self.write_flags(UART_MCR, mcr);
    }

    fn is_loopback_enabled(&self) -> bool {
        let mcr: ModemControlFlags = self.read_flags(UART_MCR);
        mcr.contains(ModemControlFlags::LOOPBACK_ENABLE)
    }

    fn set_irq_mask(&mut self, mask: InterruptMask) {
        let mut ier = InterruptEnableFlags::empty();
        self.is_tx_empty_int_enabled = false;

        if mask.contains(InterruptMask::RX_AVAILABLE) {
            ier.insert(InterruptEnableFlags::RECEIVED_DATA_AVAILABLE);
            ier.insert(InterruptEnableFlags::RECEIVER_LINE_STATUS);
        }
        if mask.contains(InterruptMask::TX_EMPTY) {
            ier.insert(InterruptEnableFlags::TRANSMITTER_HOLDING_EMPTY);
            self.is_tx_empty_int_enabled = true;
        }

        self.write_flags(UART_IER, ier);
    }

    fn get_irq_mask(&self) -> InterruptMask {
        let ier: InterruptEnableFlags = self.read_flags(UART_IER);
        let mut mask = InterruptMask::empty();

        if ier.contains(InterruptEnableFlags::RECEIVED_DATA_AVAILABLE) {
            mask |= InterruptMask::RX_AVAILABLE;
        }
        if self.is_tx_empty_int_enabled {
            mask |= InterruptMask::TX_EMPTY;
        }
        // 错误中断暂不映射到 InterruptMask
        // 用户需要通过状态寄存器检查错误

        mask
    }
}
