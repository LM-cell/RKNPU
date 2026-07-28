pub const CCLK_EMMC_DIV_SHIFT: u32 = 8;
pub const CCLK_EMMC_DIV_MASK: u32 = 0x3f << CCLK_EMMC_DIV_SHIFT;
pub const CCLK_EMMC_SEL_SHIFT: u32 = 14;
pub const CCLK_EMMC_SEL_MASK: u32 = 3 << CCLK_EMMC_SEL_SHIFT;

pub const CCLK_EMMC_SEL_GPLL: u32 = 0;
pub const CCLK_EMMC_SEL_CPLL: u32 = 1;

pub const SCLK_SFC_SEL_CPLL: u32 = 0;
pub const SCLK_SFC_SEL_GPLL: u32 = 1;
pub const SCLK_SFC_SEL_24M: u32 = 2;

// npu
pub const ACLK_NPU1: u32 = 290;
pub const HCLK_NPU1: u32 = 291;
pub const ACLK_NPU2: u32 = 292;
pub const HCLK_NPU2: u32 = 293;
pub const HCLK_NPU_CM0_ROOT: u32 = 294;
pub const FCLK_NPU_CM0_CORE: u32 = 295;
pub const CLK_NPU_CM0_RTC: u32 = 296;
pub const PCLK_NPU_PVTM: u32 = 297;
pub const PCLK_NPU_GRF: u32 = 298;
pub const CLK_NPU_PVTM: u32 = 299;
pub const CLK_CORE_NPU_PVTM: u32 = 300;
pub const ACLK_NPU0: u32 = 301;
pub const HCLK_NPU0: u32 = 302;
pub const HCLK_NPU_ROOT: u32 = 303;
pub const CLK_NPU_DSU0: u32 = 304;
pub const PCLK_NPU_ROOT: u32 = 305;
pub const PCLK_NPU_TIMER: u32 = 306;
pub const CLK_NPUTIMER_ROOT: u32 = 307;
pub const CLK_NPUTIMER0: u32 = 308;
pub const CLK_NPUTIMER1: u32 = 309;
pub const PCLK_NPU_WDT: u32 = 310;
pub const TCLK_NPU_WDT: u32 = 311;

// sd/mmmc
pub const DCLK_DECOM: u32 = 119;
pub const CCLK_EMMC: u32 = 314;
pub const BCLK_EMMC: u32 = 315;
pub const SCLK_SFC: u32 = 317;
pub const CCLK_SRC_SDIO: u32 = 410;
