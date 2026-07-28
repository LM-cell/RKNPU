/// Software-side description of DPU register values for one task.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NpuDpuDesc {
    pub burst_len: u8,        // 0x400C - AXI burst length for output DMA
    pub conv_mode: u8,        // 0x400C - must match the CNA `conv_mode`
    pub output_mode: u8,      // 0x400C - `0x2` means "write to memory"
    pub flying_mode: u8,      // 0x400C - `0`: from MAC, `1`: from RDMA
    pub out_precision: u8,    // 0x4010 - output data precision
    pub in_precision: u8,     // 0x4010 - input precision from MAC
    pub proc_precision: u8,   // 0x4010 - processing precision
    pub dst_base_addr: u32,   // 0x4020 - DMA address of the output tensor
    pub dst_surf_stride: u32, // 0x4024 - bytes between output channel surfaces
    pub width: u16,           // 0x4030 - output width, zero-based
    pub height: u16,          // 0x4034 - output height, zero-based
    pub channel: u16,         // 0x403C - output channel count, zero-based
    pub bs_bypass: u8,        // 0x4040 - bypass the bias/scale block
    pub bs_alu_bypass: u8,    // 0x4040 - bypass the BS ALU sub-unit
    pub bs_mul_bypass: u8,    // 0x4040 - bypass the BS multiplier
    pub bs_relu_bypass: u8,   // 0x4040 - bypass BS ReLU activation
    pub od_bypass: u8,        // 0x4050 - bypass output-data reordering
    pub size_e_2: u8,         // 0x4050 - OW size exponent 2
    pub size_e_1: u8,         // 0x4050 - OW size exponent 1
    pub size_e_0: u8,         // 0x4050 - OW size exponent 0
    pub channel_wdma: u16,    // 0x4058 - WDMA output channels, zero-based
    pub height_wdma: u16,     // 0x405C - WDMA output height, zero-based
    pub width_wdma: u16,      // 0x405C - WDMA output width, zero-based
    pub bn_relu_bypass: u8,   // 0x4060 - bypass BN ReLU
    pub bn_mul_bypass: u8,    // 0x4060 - bypass BN multiplier
    pub bn_alu_bypass: u8,    // 0x4060 - bypass BN ALU
    pub bn_bypass: u8,        // 0x4060 - bypass the full batch-norm block
    pub ew_bypass: u8,        // 0x4070 - bypass the full elementwise block
    pub ew_op_bypass: u8,     // 0x4070 - bypass the elementwise op
    pub ew_lut_bypass: u8,    // 0x4070 - bypass the elementwise LUT
    pub ew_op_cvt_bypass: u8, // 0x4070 - bypass elementwise operand conversion
    pub ew_relu_bypass: u8,   // 0x4070 - bypass elementwise ReLU
    pub fp32tofp16_en: u8,    // 0x4084 - enable FP32-to-FP16 output conversion
    pub out_cvt_scale: u16,   // 0x4084 - output-conversion scale factor
    pub surf_add: u32,        // 0x40C0 - surface-adder value
}
