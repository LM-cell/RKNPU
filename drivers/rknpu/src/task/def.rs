//! Low-level register offsets and PC opcode constants used to build regcmd
//! streams for the RKNPU pipeline.

#![allow(dead_code)]

/// Direct-convolution mode value used by the CNA block.
pub const DIRECT_CONVOLUTION: u8 = 0x0;
/// Size in bytes of one CBUF bank.
pub const NPU_CBUF_BANK_SIZE: u16 = 32768;
/// Number of CBUF banks available on the NPU.
pub const NPU_CBUF_BANKS: u16 = 12;

/// Pack one PC register command into the 64-bit hardware format.
pub const fn npu_op(op: u32, value: u32, reg: u32) -> u64 {
    ((op as u64 & 0xFFFF) << 48) | ((value as u64 & 0xFFFF_FFFF) << 16) | reg as u64
}

// Register names taken from the TRM v1.0 (2022-03-09). Some descriptions are
// still tentative where the manual is sparse.
pub const PC_OPERATION_ENABLE: u32 = 0x0008; // operation enable
pub const PC_BASE_ADDRESS: u32 = 0x0010; // PC base-address register
pub const PC_REGISTER_AMOUNTS: u32 = 0x0014; // register count per task

pub const CNA_S_POINTER: u32 = 0x1004; // single-register-group pointer
pub const CNA_CONV_CON1: u32 = 0x100C; // convolution control register 1
pub const CNA_CONV_CON2: u32 = 0x1010; // convolution control register 2
pub const CNA_CONV_CON3: u32 = 0x1014; // convolution control register 3
pub const CNA_DATA_SIZE0: u32 = 0x1020; // feature-data size control 0
pub const CNA_DATA_SIZE1: u32 = 0x1024; // feature-data size control 1
pub const CNA_DATA_SIZE2: u32 = 0x1028; // feature-data size control 2
pub const CNA_DATA_SIZE3: u32 = 0x102C; // feature-data size control 3
pub const CNA_WEIGHT_SIZE0: u32 = 0x1030; // weight-size control 0
pub const CNA_WEIGHT_SIZE1: u32 = 0x1034; // weight-size control 1
pub const CNA_WEIGHT_SIZE2: u32 = 0x1038; // weight-size control 2
pub const CNA_CBUF_CON0: u32 = 0x1040; // CBUF control register 0
pub const CNA_CBUF_CON1: u32 = 0x1044; // CBUF control register 1
pub const CNA_CVT_CON0: u32 = 0x104C; // input-conversion control 0
pub const CNA_CVT_CON1: u32 = 0x1050; // input-conversion control 1
pub const CNA_CVT_CON2: u32 = 0x1054; // input-conversion control 2
pub const CNA_CVT_CON3: u32 = 0x1058; // input-conversion control 3
pub const CNA_CVT_CON4: u32 = 0x105C; // input-conversion control 4
pub const CNA_FC_CON0: u32 = 0x1060; // fully connected control 0
pub const CNA_FC_CON1: u32 = 0x1064; // fully connected control 1
pub const CNA_PAD_CON0: u32 = 0x1068; // padding control 0
pub const CNA_FEATURE_DATA_ADDR: u32 = 0x1070; // input feature-data base
pub const CNA_FC_CON2: u32 = 0x1074; // fully connected control 2
pub const CNA_DMA_CON0: u32 = 0x1078; // AXI control register 0
pub const CNA_DMA_CON1: u32 = 0x107C; // AXI control register 1
pub const CNA_DMA_CON2: u32 = 0x1080; // AXI control register 2
pub const CNA_FC_DATA_SIZE0: u32 = 0x1084; // FC data-size control 0
pub const CNA_FC_DATA_SIZE1: u32 = 0x1088; // FC data-size control 1
pub const CNA_DCOMP_CTRL: u32 = 0x1100; // weight decompression control
pub const CNA_DCOMP_REGNUM: u32 = 0x1104; // decompression register count
pub const CNA_DCOMP_ADDR0: u32 = 0x1110; // weight base address
pub const CNA_DCOMP_AMOUNT: u32 = 0x1140; // decompression amount 0
pub const CNA_DCOMP_AMOUNT1: u32 = 0x1144; // decompression amount 1
pub const CNA_DCOMP_AMOUNT2: u32 = 0x1148; // decompression amount 2
pub const CNA_DCOMP_AMOUNT3: u32 = 0x114C; // decompression amount 3
pub const CNA_DCOMP_AMOUNT4: u32 = 0x1150; // decompression amount 4
pub const CNA_DCOMP_AMOUNT5: u32 = 0x1154; // decompression amount 5
pub const CNA_DCOMP_AMOUNT6: u32 = 0x1158; // decompression amount 6
pub const CNA_DCOMP_AMOUNT7: u32 = 0x115C; // decompression amount 7
pub const CNA_DCOMP_AMOUNT8: u32 = 0x1160; // decompression amount 8
pub const CNA_DCOMP_AMOUNT9: u32 = 0x1164; // decompression amount 9
pub const CNA_DCOMP_AMOUNT10: u32 = 0x1168; // decompression amount 10
pub const CNA_DCOMP_AMOUNT11: u32 = 0x116C; // decompression amount 11
pub const CNA_DCOMP_AMOUNT12: u32 = 0x1170; // decompression amount 12
pub const CNA_DCOMP_AMOUNT13: u32 = 0x1174; // decompression amount 13
pub const CNA_DCOMP_AMOUNT14: u32 = 0x1178; // decompression amount 14
pub const CNA_DCOMP_AMOUNT15: u32 = 0x117C; // decompression amount 15
pub const CNA_CVT_CON5: u32 = 0x1180; // input-conversion control 5
pub const CNA_PAD_CON1: u32 = 0x1184; // padding control 1

pub const CORE_S_POINTER: u32 = 0x3004; // single-register-group pointer
pub const CORE_MISC_CFG: u32 = 0x3010; // miscellaneous configuration
pub const CORE_DATAOUT_SIZE_0: u32 = 0x3014; // output feature size register 0
pub const CORE_DATAOUT_SIZE_1: u32 = 0x3018; // output feature size register 1
pub const CORE_CLIP_TRUNCATE: u32 = 0x301C; // shift-value register
pub const CORE_3030: u32 = 0x3030; // undocumented in the TRM so far

pub const DPU_S_POINTER: u32 = 0x4004; // single-register-group pointer
pub const DPU_FEATURE_MODE_CFG: u32 = 0x400C; // feature-mode configuration
pub const DPU_DATA_FORMAT: u32 = 0x4010; // data-format configuration
pub const DPU_OFFSET_PEND: u32 = 0x4014; // offset-pending value
pub const DPU_DST_BASE_ADD: u32 = 0x4020; // destination base address
pub const DPU_DST_SURF_STRIDE: u32 = 0x4024; // destination surface stride
pub const DPU_DATA_CUBE_WIDTH: u32 = 0x4030; // input cube width
pub const DPU_DATA_CUBE_HEIGHT: u32 = 0x4034; // input cube height
pub const DPU_DATA_CUBE_NOTCH_ADDR: u32 = 0x4038; // input cube notch signal
pub const DPU_DATA_CUBE_CHANNEL: u32 = 0x403C; // input cube channel count
pub const DPU_BS_CFG: u32 = 0x4040; // bias/scale configuration
pub const DPU_BS_ALU_CFG: u32 = 0x4044; // BS ALU configuration
pub const DPU_BS_MUL_CFG: u32 = 0x4048; // BS MUL configuration
pub const DPU_BS_RELUX_CMP_VALUE: u32 = 0x404C; // ReLUX compare value
pub const DPU_BS_OW_CFG: u32 = 0x4050; // BS output-write configuration
pub const DPU_BS_OW_OP: u32 = 0x4054; // BS output-write operation
pub const DPU_WDMA_SIZE_0: u32 = 0x4058; // WDMA size register 0
pub const DPU_WDMA_SIZE_1: u32 = 0x405C; // WDMA size register 1
pub const DPU_BN_CFG: u32 = 0x4060; // batch-normalization configuration
pub const DPU_BN_ALU_CFG: u32 = 0x4064; // BN ALU configuration
pub const DPU_BN_MUL_CFG: u32 = 0x4068; // BN MUL configuration
pub const DPU_BN_RELUX_CMP_VALUE: u32 = 0x406C; // ReLUX compare value
pub const DPU_EW_CFG: u32 = 0x4070; // elementwise configuration
pub const DPU_EW_CVT_OFFSET_VALUE: u32 = 0x4074; // EW input-conversion offset
pub const DPU_EW_CVT_SCALE_VALUE: u32 = 0x4078; // EW input-conversion scale
pub const DPU_EW_RELUX_CMP_VALUE: u32 = 0x407C; // ReLUX compare value
pub const DPU_OUT_CVT_OFFSET: u32 = 0x4080; // output-converter offset
pub const DPU_OUT_CVT_SCALE: u32 = 0x4084; // output-converter scale
pub const DPU_OUT_CVT_SHIFT: u32 = 0x4088; // output-converter shift
pub const DPU_EW_OP_VALUE_0: u32 = 0x4090; // EW operand value 0
pub const DPU_EW_OP_VALUE_1: u32 = 0x4094; // EW operand value 1
pub const DPU_EW_OP_VALUE_2: u32 = 0x4098; // EW operand value 2
pub const DPU_EW_OP_VALUE_3: u32 = 0x409C; // EW operand value 3
pub const DPU_EW_OP_VALUE_4: u32 = 0x40A0; // EW operand value 4
pub const DPU_EW_OP_VALUE_5: u32 = 0x40A4; // EW operand value 5
pub const DPU_EW_OP_VALUE_6: u32 = 0x40A8; // EW operand value 6
pub const DPU_EW_OP_VALUE_7: u32 = 0x40AC; // EW operand value 7
pub const DPU_SURFACE_ADD: u32 = 0x40C0; // surface-adder value
pub const DPU_40C4: u32 = 0x40C4; // undocumented
pub const DPU_LUT_ACCESS_CFG: u32 = 0x4100; // LUT access address and type
pub const DPU_LUT_ACCESS_DATA: u32 = 0x4104; // LUT access data
pub const DPU_LUT_CFG: u32 = 0x4108; // LUT configuration
pub const DPU_LUT_INFO: u32 = 0x410C; // LUT info register
pub const DPU_LUT_LE_START: u32 = 0x4110; // LE LUT start point
pub const DPU_LUT_LE_END: u32 = 0x4114; // LE LUT end point
pub const DPU_LUT_LO_START: u32 = 0x4118; // LO LUT start point
pub const DPU_LUT_LO_END: u32 = 0x411C; // LO LUT end point
pub const DPU_LUT_LE_SLOPE_SCALE: u32 = 0x4120; // LE LUT slope scale
pub const DPU_LUT_LE_SLOPE_SHIFT: u32 = 0x4124; // LE LUT slope shift
pub const DPU_LUT_LO_SLOPE_SCALE: u32 = 0x4128; // LO LUT slope scale
pub const DPU_LUT_LO_SLOPE_SHIFT: u32 = 0x412C; // LO LUT slope shift

// Hardware block bits exposed by the NPU pipeline.
pub const BLOCK_PC: u32 = 0x0100;
pub const BLOCK_CNA: u32 = 0x0200;
pub const BLOCK_CORE: u32 = 0x0800;
pub const BLOCK_DPU: u32 = 0x1000;
pub const BLOCK_DPU_RDMA: u32 = 0x2000;
pub const BLOCK_PPU: u32 = 0x4000;
pub const BLOCK_PPU_RDMA: u32 = 0x8000;

pub const PC_OP_01: u32 = 0x01; // register op?
pub const PC_OP_40: u32 = 0x40; // sync op?
pub const PC_OP_ENABLE: u32 = 0x80; // enable block

pub const OP_REG_PC: u32 = BLOCK_PC | PC_OP_01; // PC register write
pub const OP_REG_CNA: u32 = BLOCK_CNA | PC_OP_01; // CNA register write
pub const OP_REG_CORE: u32 = BLOCK_CORE | PC_OP_01; // core register write
pub const OP_REG_DPU: u32 = BLOCK_DPU | PC_OP_01; // DPU register write

pub const OP_40: u32 = PC_OP_40 | PC_OP_01; // sync / barrier op
pub const OP_ENABLE: u32 = PC_OP_ENABLE | PC_OP_01; // enable op
pub const OP_NONE: u32 = 0x0; // no-op

pub const PC_ENABLE: u32 = 0x01; // enable this task
pub const PC_ENABLE_CNA: u32 = 0x04; // CNA completion bit?
pub const PC_ENABLE_DPU: u32 = 0x08; // DPU completion bit?
pub const PC_ENABLE_PPU: u32 = 0x10; // PPU completion bit?
