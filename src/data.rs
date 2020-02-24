use core::mem::transmute;

pub type WordUnsigned = u32;
pub type HalfwordUnsigned = u16;
pub type LongwordUnsigned = u64;
pub type QuadwordUnsigned = u128;
pub type WordSigned = i32;
pub type HalfwordSigned = i16;
pub type LongwordSigned = i64;
pub type QuadwordSigned = i128;
pub type Byte = u8;

/// Represents the raw storage of an integer that can be interpreted either as
/// some specified size or as a RISC-V word (32-bit), signed or unsigned.
///
/// The `WordUnsigned` (`u32`) implementation uses RV32I behaviors, while the
/// `LongwordUnsigned` (`u64`) implementation uses RV64I behaviors.
pub trait Int
where
    Self: Copy,
{
    /// The native-sized signed integer type.
    type Signed;
    /// The native-sized unsigned integer type.
    type Unsigned;

    /// Returns the value zero.
    fn zero() -> Self;

    /// Populates an `Int` from a signed value of this type's native size.
    fn from_signed(v: Self::Signed) -> Self;
    /// Populates an `Int` from an unsigned value of this type's native size.
    fn from_unsigned(v: Self::Unsigned) -> Self;
    /// Populates an `Int` from a signed value of word size, sign-extending to
    /// the native size if it's larger.
    fn from_signed_word(v: WordSigned) -> Self;
    /// Populates an `Int` from an unsigned value of word size, sign-extending
    /// to the native size if it's larger.
    fn from_unsigned_word(v: WordUnsigned) -> Self;

    /// Interpretes the `Int` as a signed value of this type's native size.
    fn to_signed(self) -> Self::Signed;
    /// Interpretes the `Int` as an unsigned value of this type's native size.
    fn to_unsigned(self) -> Self::Unsigned;
    /// Interpretes the `Int` as a signed value of word size, truncating
    /// higher-order bits if the native size is larger.
    fn to_signed_word(self) -> WordSigned;
    /// Interpretes the `Int` as an unsigned value of word size, truncating
    /// higher-order bits if the native size is larger.
    fn to_unsigned_word(self) -> WordUnsigned;
}

/// RV32I implementation of `Int` using `WordUnsigned` (`u32`) as backing storage.
///
/// Because this implementation uses a Word as its native type, the `_word`
/// methods are just aliases of the corresponding native methods.
impl Int for WordUnsigned {
    type Signed = WordSigned;
    type Unsigned = WordUnsigned;

    // The "Word" implementation of Int is relatively straightforward because
    // everything is 32-bit and so our _word functions are just clones of
    // the non-_word versions.

    fn from_signed(v: WordSigned) -> Self {
        unsafe { transmute(v) }
    }

    fn from_unsigned(v: WordUnsigned) -> Self {
        v
    }

    fn from_signed_word(v: WordSigned) -> Self {
        unsafe { transmute(v) }
    }

    fn from_unsigned_word(v: WordUnsigned) -> Self {
        v
    }

    fn to_signed(self) -> WordSigned {
        unsafe { transmute(self) }
    }

    fn to_unsigned(self) -> WordUnsigned {
        self
    }

    fn to_signed_word(self) -> WordSigned {
        unsafe { transmute(self) }
    }

    fn to_unsigned_word(self) -> WordUnsigned {
        self
    }

    fn zero() -> Self {
        0
    }
}

/// RV64I implementation of `Int` using `LongwordUnsigned` (`u64`) as backing storage.
impl Int for LongwordUnsigned {
    type Signed = LongwordSigned;
    type Unsigned = LongwordUnsigned;

    fn from_signed(v: LongwordSigned) -> Self {
        unsafe { transmute(v) }
    }

    fn from_unsigned(v: LongwordUnsigned) -> Self {
        v
    }

    fn from_signed_word(v: WordSigned) -> Self {
        let nv = v as LongwordSigned; // automatic sign extension
        unsafe { transmute(nv) }
    }

    fn from_unsigned_word(v: WordUnsigned) -> Self {
        // Even though the input is unsigned, we must still treat it as
        // signed when we write it so that a subsequent re-interpretation
        // as a doubleword signed would see a sign-extended result.
        let signed32: WordSigned = unsafe { transmute(v) };
        let signed64: LongwordSigned = signed32 as i64;
        unsafe { transmute(signed64) } // i64 to u64
    }

    fn to_signed(self) -> LongwordSigned {
        unsafe { transmute(self) }
    }

    fn to_unsigned(self) -> LongwordUnsigned {
        self
    }

    fn to_signed_word(self) -> WordSigned {
        let nv = self as WordUnsigned; // just truncate existing bits
        unsafe { transmute(nv) }
    }

    fn to_unsigned_word(self) -> WordUnsigned {
        let nv = self as WordUnsigned; // just truncate existing bits
        unsafe { transmute(nv) }
    }

    fn zero() -> Self {
        0
    }
}

/// Represents the raw storage of a float that can be interpreted either as a
/// single- or double-precision float, and can also be interpreted as a raw
/// set of bits (either word-size or longword-size) in order to interact with
/// memory.
///
/// This type is intended to be used by an ISA simulator implementation that
/// supports both the "F" and the "D" extensions against a single set of
/// registers, and so its storage representation of single-precision floats
/// is consistent with the NaN-boxing requirements described in the "D"
/// extension for such implementations. A simulator implementing only the
/// "F" extension can safely use just the "single" and "word" methods,
/// ignoring the NaN-boxing behaviors since they are visible only via the
/// "double" and "longword" methods.
pub trait Float
where
    Self: Copy,
{
    type Single;
    type Double;

    /// Returns a representation of the double-precision float zero value.
    fn zero() -> Self;

    /// Populates a `Float` from a single-precision floating point value,
    /// storing it in a double-precision "NaN box" as required by the
    /// RISC-V double-precision floating point specification when storage
    /// is shared between both single-precision and double-precision floats.
    fn from_single(v: Self::Single) -> Self;

    /// Populates a `Float` from a double-precision floating point value.
    fn from_double(v: Self::Double) -> Self;

    /// Populates a `Float` using the bits from the given word directly.
    ///
    /// This is intended for interpreting words from memory as single-precision
    /// floats. The stored result is a double-precision "NaN box" as described
    /// in the documentation for `from_single`.
    fn from_word_bitwise(v: WordUnsigned) -> Self;

    /// Populates a `Float` using the bits from the given longword directly.
    ///
    /// This is intended for interpreting words from memory as double-precision
    /// floats.
    fn from_longword_bitwise(v: LongwordUnsigned) -> Self;

    /// Interprets the stored value as a NaN-boxed single-precision float.
    ///
    /// If the value was not created by either `from_single` or
    /// `from_word_bitwise` then the result will be garbage: the low-order
    /// bits of a double-precision float reinterpreted as a single-precision
    /// float.
    fn to_single(self) -> Self::Single;

    /// Interprets the stored value as a double-precision float.
    fn to_double(self) -> Self::Double;

    /// Returns the raw low-order bits from the stored value.
    ///
    /// This is intended for preparing a value to write into memory as a word.
    /// The caveats of the validity of the result are the same as for
    /// `to_single`.
    fn to_word_bitwise(self) -> WordUnsigned;

    /// Returns the bits from the stored value.
    ///
    /// This is intended for preparing a value to write into memory as a
    /// longword.
    fn to_longword_bitwise(self) -> LongwordUnsigned;
}

/// Implementation of `Float` in terms of a `f64` value, with single-precision
/// floats stored in "NaN boxes".
///
/// Because this implementation internally works with and exposes the raw bits
/// of an `f64` value, it can potentially produce non-canonical or invalid
/// `f64` values.
impl Float for f64 {
    type Single = f32;
    type Double = f64;

    fn zero() -> Self {
        0.0
    }

    fn from_double(v: f64) -> Self {
        v
    }

    fn to_double(self) -> f64 {
        self
    }

    fn from_single(v: f32) -> Self {
        // RISC-V uses "NaN boxing" to pack a 32-bit float into a 64-bit one,
        // which means we need to do some unsafe bit twiddling to expand this
        // to 64 bits while filling the top half with ones to make it appear
        // as a NaN when interpreted as float64.
        let bits32: u32 = unsafe { transmute(v) };
        let bits64: u64 = (bits32 as u64) | 0xffffffff00000000;
        unsafe { transmute(bits64) } // u64 -> float64, bitwise
    }

    fn to_single(self) -> f32 {
        // When retrieving a 32-bit float from a 64-bit register, RISC-V
        // calls for us to discard the high 32 bits and just take the low
        // 32 bits as-is. This will behave correctly if the value was
        // originally created by "NaN boxing" as implemented in from_single.
        let bits64: u64 = unsafe { transmute(self) };
        let bits32: u32 = bits64 as u32;
        unsafe { transmute(bits32) } // u32 -> float32, bitwise
    }

    fn from_longword_bitwise(v: LongwordUnsigned) -> Self {
        unsafe { transmute(v) } // u64 to f64, bit-for-bit
    }

    fn to_longword_bitwise(self) -> LongwordUnsigned {
        unsafe { transmute(self) } // f64 to u64, bit-for-bit
    }

    fn from_word_bitwise(v: WordUnsigned) -> Self {
        let f: f32 = unsafe { transmute(v) };
        Self::from_single(f)
    }

    fn to_word_bitwise(self) -> WordUnsigned {
        let f: f32 = self.to_single();
        unsafe { transmute(f) } // f32 to u32, bit-for-bit
    }
}

#[cfg(test)]
mod tests {
    use super::{Float, Int};

    #[test]
    fn int_32() {
        assert_eq!(u32::zero(), 0 as u32);
        assert_eq!(u32::from_unsigned(0xffffffff), 0xffffffff);
        assert_eq!(u32::from_signed(-1), 0xffffffff);
        assert_eq!(u32::from_unsigned_word(0xffffffff), 0xffffffff);
        assert_eq!(u32::from_signed_word(-1), 0xffffffff);
        assert_eq!(u32::from_signed(-1).to_unsigned(), 0xffffffff);
        assert_eq!(u32::from_unsigned(0xffffffff).to_signed(), -1);
        assert_eq!(u32::from_signed_word(-1).to_unsigned(), 0xffffffff);
        assert_eq!(u32::from_unsigned_word(0xffffffff).to_signed(), -1);
    }

    #[test]
    fn int_64() {
        assert_eq!(u64::zero(), 0 as u64);
        assert_eq!(u64::from_unsigned(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(u64::from_signed(-1), 0xffffffffffffffff);
        assert_eq!(u64::from_unsigned_word(0xffffffff), 0xffffffffffffffff); // sign-extended even though not signed
        assert_eq!(u64::from_signed_word(-1), 0xffffffffffffffff);
        assert_eq!(u64::from_signed(-1).to_unsigned(), 0xffffffffffffffff);
        assert_eq!(u64::from_unsigned(0xffffffffffffffff).to_signed(), -1);
        assert_eq!(u64::from_signed_word(-1).to_unsigned(), 0xffffffffffffffff);
        assert_eq!(u64::from_unsigned_word(0xffffffff).to_signed(), -1);
    }

    #[test]
    fn float_64() {
        assert_eq!(f64::zero(), 0.0 as f64);

        assert_eq!(f64::from_double(1.2), 1.2 as f64);
        assert_eq!(f64::from_double(1.2).to_double(), 1.2 as f64); // No-op

        assert!(f64::from_single(1.2).is_nan()); // 32-bit float is "NaN boxed"
        assert_eq!(f64::from_single(1.2).to_single(), 1.2 as f32); // NaN box is round-trippable

        // Reinterpreting a float64 as a float32 is allowed, but it produces garbage.
        assert_eq!(f64::from_double(1.2).to_single(), 0.00000004172325 as f32);

        assert_eq!(
            f64::from_double(1.2).to_longword_bitwise(),
            4608083138725491507 as u64 // the double float reinterpreted as a u64
        );
        assert_eq!(
            f64::from_single(1.2).to_longword_bitwise(),
            18446744070481615258 as u64 // the "boxed NaN" reinterpreted as a u64
        );
        assert_eq!(
            f64::from_double(1.2).to_word_bitwise(),
            858993459 as u32 // low half of the float reinterpreted as u64
        );
        assert_eq!(
            f64::from_single(1.2).to_word_bitwise(),
            1067030938 as u32 // the original f32 reinterpreted as a u64
        );

        assert!(f64::from_word_bitwise(1067030938).is_nan()); // 32-bit float is "NaN boxed"
        assert_eq!(f64::from_longword_bitwise(4608083138725491507), 1.2 as f64);
        assert_eq!(f64::from_word_bitwise(1067030938).to_single(), 1.2 as f32);
    }
}
