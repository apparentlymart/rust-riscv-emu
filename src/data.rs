use core::mem::{size_of, transmute};
use core::ops;

// Trait implemented by types that can support the operations needed to
// implement a base integer instruction set of some specific XLEN.
//
// This includes both signed and unsigned types, with behavior depending on
// signedness. The trait `IntBits` represents a set of bits of a particular
// XLEN that can be interpreted as either signed or unsigned, producing
// a suitable `IntValue` of that same number of bits.
//
// See `IntBits` for information on how specific unsigned integer types select
// a particular base ISA for a CPU, which then in turn selects both the signed
// and unsigned IntValue types for that CPU.
pub trait IntValue
where
    Self: Copy
        + PartialEq
        + ops::Add
        + ops::Sub
        + ops::Mul
        + ops::Div
        + ops::BitAnd
        + ops::BitOr
        + ops::BitXor
        + ops::Shl<usize>
        + ops::Shr<usize>,
{
    fn zero() -> Self;
}

// Represents the "XLEN" of a particular CPU, which depends on which of the
// RISC-V base ISAs it implements.
//
// The valid types for `Value` are:
//
// | Base ISA | `IntBits` type |
// | -------- | -------------- |
// | RV32I    | `u32`          |
// | RV64I    | `u64`          |
// | RV128I   | `u128`         |
//
// The `IntBits` for a CPU is the storage type of its general-purpose registers,
// its program counter, and of the address values on its external bus. The
// CPU implementation will interpret those bits as either signed or unsigned
// depending on the definition of the operation it's implementing.
//
// At the time of writing, the RV128I base ISA is reserved for future expansion
// and not fully specified, so its implementation in this library may not be
// fully compliant with the subsequent specification, once written.
pub trait IntBits: IntValue {
    type Address: IntValue;
    type Unsigned: IntValue;
    type Signed: IntValue;

    fn to_address(self) -> Self::Address;
    fn to_signed(self) -> Self::Signed;
    fn to_unsigned(self) -> Self::Unsigned;
    fn from_signed(v: Self::Signed) -> Self;
    fn from_unsigned(v: Self::Unsigned) -> Self;
    fn from_raw_sign_ext(v: u32, sign_bit: usize) -> Self;
}

macro_rules! int_value_impl {
    ($unsigned:ty, $signed:ty) => {
        impl IntValue for $unsigned {
            fn zero() -> Self {
                return 0;
            }
        }

        impl IntValue for $signed {
            fn zero() -> Self {
                return 0;
            }
        }

        impl IntBits for $unsigned {

            type Address = $unsigned;
            type Signed = $signed;
            type Unsigned = $unsigned;

            fn to_address(self) -> Self::Address {
                unsafe { transmute::<Self, Self::Address>(self) }
            }

            fn to_signed(self) -> Self::Signed {
                self as Self::Signed
            }

            fn to_unsigned(self) -> Self::Unsigned {
                self as Self::Unsigned
            }

            fn from_signed(v: Self::Signed) -> Self {
                unsafe { transmute::<Self::Signed, Self>(v) }
            }

            fn from_unsigned(v: Self::Unsigned) -> Self {
                unsafe { transmute::<Self::Unsigned, Self>(v) }
            }

            fn from_raw_sign_ext(v: u32, specified_bits: usize) -> Self {
                // Our methodology here is to do a shift left followed by a shift right
                // while interpreting the value as signed, and thus having the shift right
                // do the necessary sign extension.
                let shift = ((size_of::<u32>() * 8) - specified_bits) as usize;
                let sv = unsafe { transmute::<u32, i32>(v) };
                let shifted = sv << shift;
                let unshifted = shifted >> shift;
                return Self::from_signed(unshifted as $signed);
            }

        }
    };
}

int_value_impl!(u32, i32);
int_value_impl!(u64, i64);
int_value_impl!(u128, i128);
