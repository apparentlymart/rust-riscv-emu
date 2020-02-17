
use crate::data::{IntBits, IntValue};
use crate::register::Register;
/// Represents a raw instruction value that hasn't been decoded yet.
pub type RawInstruction = u32;

/// Represents a raw instruction value from the compressed instructions
/// extension that hasn't been decoded yet.
pub type RawInstructionCompressed = u16;

trait RawInstructionParts {
    fn opcode(self) -> u8;
    fn rd(self) -> Register;
    fn rs1(self) -> Register;
    fn rs2(self) -> Register;
    fn funct3(self) -> u8;
    fn funct7(self) -> u8;
}

impl RawInstructionParts for RawInstruction {
    fn opcode(self) -> u8 {
        ((self >> 0) & 0b1111111) as u8
    }

    fn rd(self) -> Register {
        Register::num(((self >> 7) & 0b11111) as usize)
    }

    fn rs1(self) -> Register {
        Register::num(((self >> 15) & 0b11111) as usize)
    }

    fn rs2(self) -> Register {
        Register::num(((self >> 20) & 0b11111) as usize)
    }

    fn funct3(self) -> u8 {
        ((self >> 12) & 0b111) as u8
    }

    fn funct7(self) -> u8 {
        ((self >> 25) & 0b1111111) as u8
    }
}

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
            opcode: raw.opcode(),
            rd: raw.rd(),
            funct3: raw.funct3(),
            rs1: raw.rs1(),
            rs2: raw.rs2(),
            funct7: raw.funct7(),
            imm: Imm::zero(),
        }
    }

    pub fn i_type(raw: RawInstruction) -> Self {
        Self {
            opcode: raw.opcode(),
            rd: raw.rd(),
            funct3: raw.funct3(),
            rs1: raw.rs1(),
            rs2: Register::zero(),
            funct7: 0,

            // raw contains bits 0 through 11, which we must sign-extend to
            // a full value of the appropriate size for the CPU.
            imm: Imm::from_raw_sign_ext((raw >> 20) & 0b111111111111, 12),
        }
    }

    pub fn s_type(raw: RawInstruction) -> Self {
        Self {
            opcode: raw.opcode(),
            rd: Register::zero(),
            funct3: raw.funct3(),
            rs1: raw.rs1(),
            rs2: raw.rs2(),
            funct7: 0,

            // raw contains bits 0 through 11 split across two subfields, which
            // we must sign-extend to a full value of the appropriate size for
            // the CPU.
            imm: Imm::from_raw_sign_ext((raw >> 7) & 0b11111 | ((raw >> 25) & 0b1111111) << 5, 12),
        }
    }

    pub fn u_type(raw: RawInstruction) -> Self {
        Self {
            opcode: raw.opcode(),
            rd: raw.rd(),
            funct3: 0,
            rs1: Register::zero(),
            rs2: Register::zero(),
            funct7: 0,

            // raw contains bits 12 through 31, with the first 12 bits taken
            // as zero. That means we can just mask out the lower bits here,
            // because the significant bits are already in the correct locations.
            imm: Imm::from_raw_sign_ext(raw & 0b11111111111111111111_000000000000, 32),
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
    fn decode_r_type() {
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
    fn decode_i_type() {
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

    #[test]
    fn decode_s_type() {
        let ins = Instruction::<u32>::s_type(0b1111111_10000_01000_100_11110_0000001);
        assert_eq!(
            ins,
            Instruction {
                opcode: 1,
                rd: Register::num(0),
                funct3: 4,
                rs1: Register::num(8),
                rs2: Register::num(16),
                funct7: 0,
                imm: IntBits::from_signed(-2 as i32),
            }
        );
    }

    #[test]
    fn decode_u_type() {
        let ins = Instruction::<u32>::u_type(0b11111111111111111111_11110_0000101);
        assert_eq!(
            ins,
            Instruction {
                opcode: 5,
                rd: Register::num(30),
                funct3: 0,
                rs1: Register::zero(),
                rs2: Register::zero(),
                funct7: 0,
                imm: IntBits::from_signed(-4096 as i32),
            }
        );
    }
}
