use crate::register::{FloatRegister, IntRegister};
use core::mem::size_of;
use core::mem::transmute;

/// Represents a raw RISC-V instruction word that is yet to be decoded.
///
/// It can represent both standard-length and compressed instructions, the
/// latter of which are supported by ignoring the higher-order parcel.
#[derive(Debug, PartialEq)]
pub struct RawInstruction(u32);

impl RawInstruction {
    pub fn new(word: u32) -> Self {
        return Self(word);
    }

    pub fn opcode(&self) -> u8 {
        match self.length() {
            2 => (self.0 & 0b0000000000000011) as u8,
            4 => (self.0 & 0b0000000001111111) as u8,
            _ => self.0 as u8,
        }
    }

    pub fn matches(&self, mask: u32, want: u32) -> bool {
        (self.0 & mask) == want
    }

    pub fn length(&self) -> usize {
        instruction_length(self.0 as u16)
    }

    pub fn aq(&self) -> bool {
        return (self.0 & 0b00000100000000000000000000000000) != 0;
    }

    pub fn cfrd(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return FloatRegister::num(raw as usize);
    }

    pub fn cfrdq(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000000000011100) >> 2;
        return IntRegister::num(raw as usize);
    }

    pub fn cfrs2(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return FloatRegister::num(raw as usize);
    }

    pub fn cfrs2q(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000000000011100) >> 2;
        return IntRegister::num(raw as usize);
    }

    pub fn cimm16sp(&self) -> i32 {
        let width = 10;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000001000000000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 3;
        // Fill 0b00000000000000000000000000010000
        raw |= (self.0 & 0b00000000000000000000000001000000) >> 2;
        // Fill 0b00000000000000000000000001000000
        raw |= (self.0 & 0b00000000000000000000000000100000) << 1;
        // Fill 0b00000000000000000000000110000000
        raw |= (self.0 & 0b00000000000000000000000000011000) << 4;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000000000000000100) << 3;
        return sign_extend(raw, width);
    }

    pub fn cimm4spn(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000110000
        raw |= (self.0 & 0b00000000000000000001100000000000) >> 7;
        // Fill 0b00000000000000000000001111000000
        raw |= (self.0 & 0b00000000000000000000011110000000) >> 1;
        // Fill 0b00000000000000000000000000000100
        raw |= (self.0 & 0b00000000000000000000000001000000) >> 4;
        // Fill 0b00000000000000000000000000001000
        raw |= (self.0 & 0b00000000000000000000000000100000) >> 2;
        return raw;
    }

    pub fn cimmb(&self) -> i32 {
        let width = 9;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000100000000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 4;
        // Fill 0b00000000000000000000000000011000
        raw |= (self.0 & 0b00000000000000000000110000000000) >> 7;
        // Fill 0b00000000000000000000000011000000
        raw |= (self.0 & 0b00000000000000000000000001100000) << 1;
        // Fill 0b00000000000000000000000000000110
        raw |= (self.0 & 0b00000000000000000000000000011000) >> 2;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000000000000000100) << 3;
        return sign_extend(raw, width);
    }

    pub fn cimmd(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000111000
        raw |= (self.0 & 0b00000000000000000001110000000000) >> 7;
        // Fill 0b00000000000000000000000011000000
        raw |= (self.0 & 0b00000000000000000000000001100000) << 1;
        return raw;
    }

    pub fn cimmi(&self) -> i32 {
        let width = 6;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return sign_extend(raw, width);
    }

    pub fn cimmj(&self) -> i32 {
        let width = 12;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000100000000000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 1;
        // Fill 0b00000000000000000000000000010000
        raw |= (self.0 & 0b00000000000000000000100000000000) >> 7;
        // Fill 0b00000000000000000000001100000000
        raw |= (self.0 & 0b00000000000000000000011000000000) >> 1;
        // Fill 0b00000000000000000000010000000000
        raw |= (self.0 & 0b00000000000000000000000100000000) << 2;
        // Fill 0b00000000000000000000000001000000
        raw |= (self.0 & 0b00000000000000000000000010000000) >> 1;
        // Fill 0b00000000000000000000000010000000
        raw |= (self.0 & 0b00000000000000000000000001000000) << 1;
        // Fill 0b00000000000000000000000000001110
        raw |= (self.0 & 0b00000000000000000000000000111000) >> 2;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000000000000000100) << 3;
        return sign_extend(raw, width);
    }

    pub fn cimmldsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000011000
        raw |= (self.0 & 0b00000000000000000000000001100000) >> 2;
        // Fill 0b00000000000000000000000111000000
        raw |= (self.0 & 0b00000000000000000000000000011100) << 4;
        return raw;
    }

    pub fn cimmlqsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000010000
        raw |= (self.0 & 0b00000000000000000000000001000000) >> 2;
        // Fill 0b00000000000000000000001111000000
        raw |= (self.0 & 0b00000000000000000000000000111100) << 4;
        return raw;
    }

    pub fn cimmlwsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000011100
        raw |= (self.0 & 0b00000000000000000000000001110000) >> 2;
        // Fill 0b00000000000000000000000011000000
        raw |= (self.0 & 0b00000000000000000000000000001100) << 4;
        return raw;
    }

    pub fn cimmq(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000110000
        raw |= (self.0 & 0b00000000000000000001100000000000) >> 7;
        // Fill 0b00000000000000000000000100000000
        raw |= (self.0 & 0b00000000000000000000010000000000) >> 2;
        // Fill 0b00000000000000000000000011000000
        raw |= (self.0 & 0b00000000000000000000000001100000) << 1;
        return raw;
    }

    pub fn cimmsdsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000111000
        raw |= (self.0 & 0b00000000000000000001110000000000) >> 7;
        // Fill 0b00000000000000000000000111000000
        raw |= (self.0 & 0b00000000000000000000001110000000) >> 1;
        return raw;
    }

    pub fn cimmsh5(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return raw;
    }

    pub fn cimmsh6(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return raw;
    }

    pub fn cimmsqsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000110000
        raw |= (self.0 & 0b00000000000000000001100000000000) >> 7;
        // Fill 0b00000000000000000000001111000000
        raw |= (self.0 & 0b00000000000000000000011110000000) >> 1;
        return raw;
    }

    pub fn cimmswsp(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000111100
        raw |= (self.0 & 0b00000000000000000001111000000000) >> 7;
        // Fill 0b00000000000000000000000011000000
        raw |= (self.0 & 0b00000000000000000000000110000000) >> 1;
        return raw;
    }

    pub fn cimmui(&self) -> i32 {
        let width = 18;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000100000000000000000
        raw |= (self.0 & 0b00000000000000000001000000000000) << 5;
        // Fill 0b00000000000000011111000000000000
        raw |= (self.0 & 0b00000000000000000000000001111100) << 10;
        return sign_extend(raw, width);
    }

    pub fn cimmw(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000111000
        raw |= (self.0 & 0b00000000000000000001110000000000) >> 7;
        // Fill 0b00000000000000000000000000000100
        raw |= (self.0 & 0b00000000000000000000000001000000) >> 4;
        // Fill 0b00000000000000000000000001000000
        raw |= (self.0 & 0b00000000000000000000000000100000) << 1;
        return raw;
    }

    pub fn cnzimmi(&self) -> i32 {
        let width = 6;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000100000
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 7;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return sign_extend(raw, width);
    }

    pub fn crd(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn crd0(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000001
        raw |= (self.0 & 0b00000000000000000001000000000000) >> 12;
        return IntRegister::num(raw as usize);
    }

    pub fn crdq(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000000000011100) >> 2;
        return IntRegister::num(raw as usize);
    }

    pub fn crs1(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn crs1q(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000001110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn crs1rd(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn crs1rdq(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000001110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn crs2(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000000001111100) >> 2;
        return IntRegister::num(raw as usize);
    }

    pub fn crs2q(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000000111
        raw |= (self.0 & 0b00000000000000000000000000011100) >> 2;
        return IntRegister::num(raw as usize);
    }

    pub fn csr12(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000111111111111
        raw |= (self.0 & 0b11111111111100000000000000000000) >> 20;
        return raw;
    }

    pub fn frd(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return FloatRegister::num(raw as usize);
    }

    pub fn frs1(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000011111000000000000000) >> 15;
        return FloatRegister::num(raw as usize);
    }

    pub fn frs2(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000001111100000000000000000000) >> 20;
        return FloatRegister::num(raw as usize);
    }

    pub fn frs3(&self) -> FloatRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b11111000000000000000000000000000) >> 27;
        return FloatRegister::num(raw as usize);
    }

    pub fn imm12(&self) -> i32 {
        let width = 12;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000111111111111
        raw |= (self.0 & 0b11111111111100000000000000000000) >> 20;
        return sign_extend(raw, width);
    }

    pub fn imm20(&self) -> i32 {
        let width = 32;
        let mut raw: u32 = 0;
        // Fill 0b11111111111111111111000000000000
        raw |= (self.0 & 0b11111111111111111111000000000000);
        return sign_extend(raw, width);
    }

    pub fn jimm20(&self) -> i32 {
        let width = 21;
        let mut raw: u32 = 0;
        // Fill 0b00000000000100000000000000000000
        raw |= (self.0 & 0b10000000000000000000000000000000) >> 11;
        // Fill 0b00000000000000000000011111111110
        raw |= (self.0 & 0b01111111111000000000000000000000) >> 20;
        // Fill 0b00000000000000000000100000000000
        raw |= (self.0 & 0b00000000000100000000000000000000) >> 9;
        // Fill 0b00000000000011111111000000000000
        raw |= (self.0 & 0b00000000000011111111000000000000);
        return sign_extend(raw, width);
    }

    pub fn oimm12(&self) -> i32 {
        let width = 12;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000111111111111
        raw |= (self.0 & 0b11111111111100000000000000000000) >> 20;
        return sign_extend(raw, width);
    }

    pub fn oimm20(&self) -> i32 {
        let width = 32;
        let mut raw: u32 = 0;
        // Fill 0b11111111111111111111000000000000
        raw |= (self.0 & 0b11111111111111111111000000000000);
        return sign_extend(raw, width);
    }

    pub fn pred(&self) -> bool {
        return (self.0 & 0b00001111000000000000000000000000) != 0;
    }

    pub fn rd(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return IntRegister::num(raw as usize);
    }

    pub fn rl(&self) -> bool {
        return (self.0 & 0b00000010000000000000000000000000) != 0;
    }

    pub fn rm(&self) -> bool {
        return (self.0 & 0b00000000000000000111000000000000) != 0;
    }

    pub fn rs1(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000011111000000000000000) >> 15;
        return IntRegister::num(raw as usize);
    }

    pub fn rs2(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000001111100000000000000000000) >> 20;
        return IntRegister::num(raw as usize);
    }

    pub fn rs3(&self) -> IntRegister {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b11111000000000000000000000000000) >> 27;
        return IntRegister::num(raw as usize);
    }

    pub fn sbimm12(&self) -> i32 {
        let width = 13;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000001000000000000
        raw |= (self.0 & 0b10000000000000000000000000000000) >> 19;
        // Fill 0b00000000000000000000011111100000
        raw |= (self.0 & 0b01111110000000000000000000000000) >> 20;
        // Fill 0b00000000000000000000000000011110
        raw |= (self.0 & 0b00000000000000000000111100000000) >> 7;
        // Fill 0b00000000000000000000100000000000
        raw |= (self.0 & 0b00000000000000000000000010000000) << 4;
        return sign_extend(raw, width);
    }

    pub fn shamt5(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000001111100000000000000000000) >> 20;
        return raw;
    }

    pub fn shamt6(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000111111
        raw |= (self.0 & 0b00000011111100000000000000000000) >> 20;
        return raw;
    }

    pub fn shamt7(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000001111111
        raw |= (self.0 & 0b00000111111100000000000000000000) >> 20;
        return raw;
    }

    pub fn simm12(&self) -> i32 {
        let width = 12;
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000111111100000
        raw |= (self.0 & 0b11111110000000000000000000000000) >> 20;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000000000000111110000000) >> 7;
        return sign_extend(raw, width);
    }

    pub fn succ(&self) -> bool {
        return (self.0 & 0b00000000111100000000000000000000) != 0;
    }

    pub fn zimm(&self) -> u32 {
        let mut raw: u32 = 0;
        // Fill 0b00000000000000000000000000011111
        raw |= (self.0 & 0b00000000000011111000000000000000) >> 15;
        return raw;
    }
}

/// Enumeration of top-level opcodes for full-length operations.
#[repr(u8)]
pub enum Opcode {
    Amo = 0b0101111,
    Auipc = 0b0010111,
    Branch = 0b1100011,
    Jal = 0b1101111,
    Jalr = 0b1100111,
    Load = 0b0000011,
    LoadFp = 0b0000111,
    Lui = 0b0110111,
    Madd = 0b1000011,
    MiscMem = 0b0001111,
    Msub = 0b1000111,
    Nmadd = 0b1001111,
    Nmsub = 0b1001011,
    Op = 0b0110011,
    Op32 = 0b0111011,
    OpFp = 0b1010011,
    OpImm = 0b0010011,
    OpImm32 = 0b0011011,
    Store = 0b0100011,
    StoreFp = 0b0100111,
    System = 0b1110011,
}

fn sign_extend(v: u32, width: usize) -> i32 {
    // Our methodology here is to do a shift left followed by a shift right
    // while interpreting the value as signed, and thus having the shift right
    // do the necessary sign extension.
    if width == 32 {
        // Easy case: the number is already fully-specified
        return unsafe { transmute::<u32, i32>(v) };
    }
    let shift = ((size_of::<u32>() * 8) - width) as usize;
    let sv = unsafe { transmute::<u32, i32>(v) };
    let shifted = sv << shift;
    let unshifted = shifted >> shift;
    return unshifted;
}

// Given the low-order halfword for a RISC-V instruction, returns the total
// length of that instruction in bytes by interpreting only the instruction
// size scheme.
//
// If the given data is not actually from a RISC-V instruction then the result
// is undefined. The length encoding mechanism uses 16-bit "parcels", so
// in practice the result will always be an even number.
//
// As a special case, the result zero indicates an invalid encoding. Currently
// that result will appear only for an instruction that seems to be using
// the reserved extension for instructions >= 192 bits, which is not
// supported by this implementation due to it being undefined at the time of
// writing.
pub fn instruction_length(low_parcel: u16) -> usize {
    if low_parcel & 0b11 != 0b11 {
        return 2;
    }
    if low_parcel & 0b11111 != 0b11111 {
        return 4;
    }
    if low_parcel & 0b111111 == 0b011111 {
        return 6;
    }
    if low_parcel & 0b1111111 == 0b0111111 {
        return 8;
    }
    if low_parcel & 0b1111111 == 0b1111111 && low_parcel & 0b111000000000000 != 0b111000000000000 {
        let n = (low_parcel >> 12 & 0b111) as usize;
        return 10 + n * 2;
    }
    return 0;
}

#[cfg(test)]
mod tests {

    #[test]
    fn instruction_length() {
        assert_eq!(super::instruction_length(0b0000000000000000), 2);
        assert_eq!(super::instruction_length(0b0000000000000001), 2);
        assert_eq!(super::instruction_length(0b0000000000000010), 2);
        assert_eq!(super::instruction_length(0b0000000000000110), 2);
        assert_eq!(super::instruction_length(0b0000000000000011), 4);
        assert_eq!(super::instruction_length(0b0000000000000111), 4);
        assert_eq!(super::instruction_length(0b0000000000011111), 6);
        assert_eq!(super::instruction_length(0b0000000000111111), 8);
        assert_eq!(super::instruction_length(0b0000000001111111), 10);
        assert_eq!(super::instruction_length(0b0001000001111111), 12);
        assert_eq!(super::instruction_length(0b0010000001111111), 14);
        assert_eq!(super::instruction_length(0b0011000001111111), 16);
        assert_eq!(super::instruction_length(0b0100000001111111), 18);
        assert_eq!(super::instruction_length(0b0101000001111111), 20);
        assert_eq!(super::instruction_length(0b0110000001111111), 22);
        assert_eq!(super::instruction_length(0b0111000001111111), 0); // reserved for future expansion
    }
}
