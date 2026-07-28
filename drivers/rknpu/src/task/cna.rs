/// Software-side CNA descriptor for one hardware task.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NpuCnaDesc {
    pub enable: u8,
    pub conv_mode: u8,                // 0x100C - convolution mode, direct or Winograd
    pub in_precision: u8,             // 0x100C - input precision, such as Int8 or FP16
    pub proc_precision: u8,           // 0x100C - processing precision
    pub kernel_groups: u8,            // 0x1010 - number of kernel groups
    pub feature_grains: u16,          // 0x1010 - feature-data grain size
    pub conv_y_stride: u8,            // 0x1014 - vertical stride
    pub conv_x_stride: u8,            // 0x1014 - horizontal stride
    pub datain_width: u16,            // 0x1020 - input feature width
    pub datain_height: u16,           // 0x1020 - input feature height
    pub datain_channel: u16,          // 0x1024 - input feature channels
    pub dataout_width: u16,           // 0x1028 - output feature width
    pub dataout_atomics: u32,         // 0x102C - output atomic count, usually W x H
    pub weight_bytes: u32,            // 0x1030 - total weight size in bytes
    pub weight_bytes_per_kernel: u32, // 0x1034 - weight bytes per output channel
    pub weight_width: u8,             // 0x1038 - kernel width
    pub weight_height: u8,            // 0x1038 - kernel height
    pub weight_kernels: u16,          // 0x1038 - number of output channels
    pub weight_bank: u8,              // 0x1040 - CBUF bank allocation for weights
    pub data_bank: u8,                // 0x1040 - CBUF bank allocation for features
    pub data_entries: u16,            // 0x1044 - number of feature entries in CBUF
    pub data_sign: u8,                // 0x104C - signed or unsigned input flag
    pub cvt_type: u8,                 // 0x104C - conversion type
    pub cvt_bypass: u8,               // 0x104C - bypass input conversion
    pub cvt_scale0: u16,              // 0x1050 - scale for channel group 0
    pub cvt_scale1: u16,              // 0x1054 - scale for channel group 1
    pub cvt_scale2: u16,              // 0x1058 - scale for channel group 2
    pub cvt_scale3: u16,              // 0x105C - scale for channel group 3
    pub fc_skip_en: u8,               // 0x1060 - fully connected skip enable
    pub data_offset: u16,             // 0x1064 - start offset in input data
    pub pad_left: u8,                 // 0x1068 - left zero-padding
    pub pad_top: u8,                  // 0x1068 - top zero-padding
    pub feature_base_addr: u32,       // 0x1070 - DMA address of input features
    pub weight_offset: u16,           // 0x1074 - starting offset for weights
    pub weight_burst_len: u8,         // 0x1078 - AXI burst length for weight DMA
    pub data_burst_len: u8,           // 0x1078 - AXI burst length for feature DMA
    pub line_stride: u32,             // 0x107C - bytes between consecutive lines
    pub surf_stride: i32,             // 0x1080 - bytes between channel surfaces
    pub dma_width: u16,               // 0x1084 - DMA transfer width
    pub dma_height: u16,              // 0x1084 - DMA transfer height
    pub dma_channel: u16,             // 0x1088 - DMA transfer channels
    pub decompress_addr0: u32,        // 0x1110 - DMA address of compressed weights
    pub dataout_height: u16,
}

/// MAC core descriptor.
///
/// The MAC unit performs the actual multiply-accumulate work. Its descriptor is
/// fairly small because the CNA block handles most of the input routing.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NpuCoreDesc {
    pub proc_precision: u8,   // 0x3010 - compute precision
    pub qd_en: u8,            // 0x3010 - quantize/dequantize enable
    pub dataout_height: u16,  // 0x3014 - output height, stored zero-based
    pub dataout_width: u16,   // 0x3014 - output width, stored zero-based
    pub dataout_channel: u16, // 0x3018 - output channels, stored zero-based
}

/// PC descriptor, rarely filled by hand.
///
/// In most flows the register layer programs the PC block directly through
/// `submit_pc()` rather than materializing this structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NpuPcDesc {
    pub pc_source_addr: u32, // 0x0010 - regcmd buffer base address
    pub pc_data_amount: u32, // 0x0014 - regcmd words per task
}

/// Raw 64-bit register-command array for one CNA+MAC task.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NpuCnaCoreTask {
    pub ops: [u64; 112],
}
