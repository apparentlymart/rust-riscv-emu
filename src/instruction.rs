
use crate::data::{IntBits, IntValue};
use crate::register::Register;
/// Represents a raw instruction value that hasn't been decoded yet.
pub type RawInstruction = u32;

/// Represents a raw instruction value from the compressed instructions
/// extension that hasn't been decoded yet.
pub type RawInstructionCompressed = u16;

/// Represents a decoded instruction.
///
/// The meaning and validity of the fields in an `Instruction` value depend on
/// the value of `opcode`
#[derive(Debug)]
pub struct Instruction<Imm: IntBits> {
    opcode: u8,
    rd: Register,
    funct3: u8,
    rs1: Register,
    rs2: Register,
    funct7: u8,
    imm: Imm,
}

impl<Imm: IntBits> Instruction<Imm> {
    /// Decodes the given raw instruction as an "R-type" instruction.
    pub fn r_type(raw: RawInstruction) -> Self {
        Self {
            opcode: ((raw >> 0) & 0b1111111) as u8,
            rd: Register::num(((raw >> 7) & 0b11111) as usize),
            funct3: ((raw >> 12) & 0b111) as u8,
            rs1: Register::num(((raw >> 15) & 0b11111) as usize),
            rs2: Register::num(((raw >> 20) & 0b11111) as usize),
            funct7: ((raw >> 25) & 0b1111111) as u8,
            imm: Imm::zero(),
        }
    }

    pub fn i_type(raw: RawInstruction) -> Self {
        Self {
            opcode: ((raw >> 0) & 0b1111111) as u8,
            rd: Register::num(((raw >> 7) & 0b11111) as usize),
            funct3: ((raw >> 12) & 0b111) as u8,
            rs1: Register::num(((raw >> 15) & 0b11111) as usize),
            rs2: Register::zero(),
            funct7: 0,

            // raw contains bits 0 through 11, which we must sign-extend to
            // an i32.
            imm: Imm::from_raw_sign_ext((raw >> 20) & 0b111111111111, 12),
        }
    }
}

impl<Imm: IntBits> PartialEq for Instruction<Imm> {
    fn eq(&self, other: &Self) -> bool {
        (self.opcode == other.opcode
            && self.rd == other.rd
            && self.funct3 == other.funct3
            && self.rs1 == other.rs1
            && self.rs2 == other.rs2
            && self.funct7 == other.funct7
            && self.imm == other.imm)
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, IntBits, Register};

    #[test]
    fn simple_decode_r_type() {
        let ins = Instruction::<u32>::r_type(0b0100000_10000_01000_100_00010_0000001);
        assert_eq!(
            ins,
            Instruction {
                opcode: 1,
                rd: Register::num(2),
                funct3: 4,
                rs1: Register::num(8),
                rs2: Register::num(16),
                funct7: 32,
                imm: 0,
            }
        );
    }

    #[test]
    fn simple_decode_i_type() {
        let ins = Instruction::<u32>::i_type(0b111111111111_10000_100_00010_0000001);
        assert_eq!(
            ins,
            Instruction {
                opcode: 1,
                rd: Register::num(2),
                funct3: 4,
                rs1: Register::num(16),
                rs2: Register::num(0),
                funct7: 0,
                imm: IntBits::from_signed(-1 as i32),
            }
        );
    }
}
