#![no_std]

mod cpu;
mod data;
mod exception;
mod hart;
mod instruction;
mod isa;
mod memory;
mod raw_instruction;
mod register;

pub use cpu::CPU;
pub use data::Byte;
pub use data::{Float, Int, Zero};
pub use data::{HalfwordSigned, LongwordSigned, QuadwordSigned, WordSigned};
pub use data::{HalfwordUnsigned, LongwordUnsigned, QuadwordUnsigned, WordUnsigned};
pub use exception::{Cause, ExceptionCause, InterruptCause};
pub use hart::{Hart, SingleThreadUserHart};
pub use instruction::{Instruction, Operation};
pub use isa::BaseISA;
pub use memory::{AddressConverter, AddressTransformer, Bus, Memory};
pub use raw_instruction::RawInstruction;
pub use register::{ControlStatusRegister, FloatRegister, IntRegister, Register};

/// Instruction execution engines.
pub mod exec;

/// Contains the instruction enum types for each base ISA. (Implementations of `Operation`.)
pub mod ops {
    pub use crate::instruction::OperationRV32 as RV32;
    pub use crate::instruction::OperationRV64 as RV64;
}

/// Contains the marker types representing the base ISAs. (Implementations of `BaseISA`.)
pub mod isas {
    pub use crate::isa::{RV32, RV64};
}
