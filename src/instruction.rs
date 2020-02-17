use crate::register::Register;

/// Represents a raw instruction value that hasn't been decoded yet.
pub type RawInstruction = i32;

/// Represents a raw instruction value from the compressed instructions
/// extension that hasn't been decoded yet.
pub type RawInstructionCompressed = i16;

/// Represents a decoded instruction.
///
/// The meaning and validity of the fields in an `Instruction` value depend on
/// the value of `opcode`
#[derive(Debug)]
pub struct Instruction {
    opcode: u8,
    rd: Register,
    funct3: u8,
    rs1: Register,
    rs2: Register,
    funct7: u8,
    imm: i32,
}

impl Instruction {
    /// Decodes the given raw instruction as an "R-type" instruction.
    pub fn r_type(raw: RawInstruction) -> Self {
        Self {
            opcode: ((raw >> 0) & 0b1111111) as u8,
            rd: Register::num(((raw >> 7) & 0b11111) as usize),
            funct3: ((raw >> 12) & 0b111) as u8,
            rs1: Register::num(((raw >> 15) & 0b11111) as usize),
            rs2: Register::num(((raw >> 20) & 0b11111) as usize),
            funct7: ((raw >> 25) & 0b1111111) as u8,
            imm: 0,
        }
    }
}

impl PartialEq for Instruction {
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
    use super::{Instruction, Register};

    #[test]
    fn simple_decode_r_type() {
        let ins = Instruction::r_type(0x00000000);
        assert_eq!(
            ins,
            Instruction {
                opcode: 0,
                rd: Register::num(0),
                funct3: 0,
                rs1: Register::num(0),
                rs2: Register::num(0),
                funct7: 0,
                imm: 0,
            }
        );
    }
}
