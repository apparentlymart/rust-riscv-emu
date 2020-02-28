use crate::data::{Float, Int, LongwordUnsigned, WordUnsigned, Zero};
use crate::instruction::{Operation, OperationRV32, OperationRV64};

/// Represents a RISC-V base ISA, collecting its integer data type and
/// operation type together as associated types for more convenient
/// use as a single type parameter on types that are generic over entire base
/// ISAs, rather than individual aspects thereof.
pub trait BaseISA {
    type Int: Int;
    type Float: Float;
    type Operation: Operation;
}

/// A compile-time-only type that represents the RISC-V 32-bit base ISA in
/// type parameters that require a `BaseISA` implementation.
pub enum RV32 {}

impl BaseISA for RV32 {
    type Int = WordUnsigned;
    type Float = f64;
    type Operation = OperationRV32;
}

/// A compile-time-only type that represents the RISC-V 64-bit base ISA in
/// type parameters that require a `BaseISA` implementation.
pub enum RV64 {}

impl BaseISA for RV64 {
    type Int = LongwordUnsigned;
    type Float = f64;
    type Operation = OperationRV64;
}
