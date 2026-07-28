//! Abstractions for high-level NPU operations.
//!
//! Each operation variant maps a neural-network operator such as matmul or
//! convolution into a sequence of 64-bit register commands consumed by the PC
//! block and then applied to the CNA, MAC, and DPU pipeline.
//!
//! To add a new operator:
//!
//! 1. Create a new file under `op/`.
//! 2. Implement [`OperationTrait::fill_regcmd`].
//! 3. Add a matching variant to [`Operation`].

use crate::op::matmul::MatMul;

pub mod matmul;

/// Data-precision modes supported by the hardware.
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum Precision {
    /// 8-bit signed integer, the common path for quantized inference.
    Int8 = 0x0,
    /// IEEE 754 half precision.
    Float16 = 0x2,
    /// 32-bit signed integer, often used for accumulators.
    Int32 = 0x4,
    /// IEEE 754 single precision.
    Float32 = 0x5,
}

/// Concrete NPU operation together with its DMA-backed tensors.
pub enum Operation {
    /// Matrix multiplication: `A(m x k, i8) * B(k x n, i8) -> C(m x n, i32)`.
    MatMulu8(MatMul<i8, i32>),
}

impl Operation {
    /// Number of 64-bit register commands required by this operation.
    pub fn reg_amount(&self) -> u32 {
        112
    }

    /// Fill a preallocated regcmd buffer with this operation's hardware command
    /// stream.
    pub fn fill_regcmd(&self, regcmd: &mut [u64]) {
        match self {
            Operation::MatMulu8(op) => {
                op.fill_regcmd(regcmd);
            }
        }
    }
}

/// Trait implemented by every concrete NPU operation.
pub trait OperationTrait {
    /// Write the operation's register-command sequence into `regcmd`.
    fn fill_regcmd(&self, regcmd: &mut [u64]);
}
