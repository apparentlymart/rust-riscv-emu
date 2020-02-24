#![no_std]

mod cpu;
mod data;
mod exception;
mod hart;
mod instruction;
mod memory;
mod raw_instruction;
mod register;

pub use cpu::CPU;
pub use data::Byte;
pub use data::{Float, Int};
pub use data::{HalfwordSigned, LongwordSigned, QuadwordSigned, WordSigned};
pub use data::{HalfwordUnsigned, LongwordUnsigned, QuadwordUnsigned, WordUnsigned};
pub use exception::{Cause, ExceptionCause, InterruptCause};
pub use hart::Hart;
pub use instruction::{Instruction, Operation};
pub use memory::{AddressConverter, Bus, Memory};
pub use raw_instruction::RawInstruction;
pub use register::{ControlStatusRegister, FloatRegister, IntRegister, Register};

pub mod ops {
    pub use crate::instruction::OperationRV32 as RV32;
}
