use crate::raw_instruction::RawInstruction;
use crate::register::FloatRegister;
use crate::register::IntRegister;

mod instruction_32;
pub use instruction_32::OperationRV32;

//mod instruction_64;
//pub use instruction_64::OperationRV64;

pub trait Operation {
    fn decode_raw(raw: RawInstruction) -> Self;
}
impl Operation for OperationRV32 {
    fn decode_raw(raw: RawInstruction) -> Self {
        OperationRV32::decode_from_raw(raw)
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction<Op: Operation> {
    pub op: Op,
    pub length: usize,
}

impl<Op: Operation> Instruction<Op> {
    pub fn decode_raw(raw: RawInstruction) -> Self {
        let length = raw.length();
        let op = Op::decode_raw(raw);
        return Self {
            op: op,
            length: length,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, OperationRV32, RawInstruction};
    use crate::register::{FloatRegister, IntRegister};
    type Inst32 = Instruction<OperationRV32>;
    type Op32 = OperationRV32;

    #[test]
    fn instruction_rv32() {
        fn mkinst(raw: u32) -> Inst32 {
            Inst32::decode_raw(RawInstruction::new(raw))
        }

        assert_eq!(
            mkinst(0b0000000_00011_00010_000_00001_0110011),
            Inst32 {
                op: Op32::Add {
                    rd: IntRegister::num(1),
                    rs1: IntRegister::num(2),
                    rs2: IntRegister::num(3),
                },
                length: 4,
            }
        );
        assert_eq!(
            mkinst(0b000000000011_00010_000_00001_0010011),
            Inst32 {
                op: Op32::Addi {
                    rd: IntRegister::num(1),
                    rs1: IntRegister::num(2),
                    simm: 3,
                },
                length: 4,
            }
        );
        assert_eq!(
            mkinst(0b111111111100_00010_000_00001_0010011),
            Inst32 {
                op: Op32::Addi {
                    rd: IntRegister::num(1),
                    rs1: IntRegister::num(2),
                    simm: -4, // sign-extended
                },
                length: 4,
            }
        );
        assert_eq!(
            mkinst(0b0100000_00011_00010_000_00001_0110011),
            Inst32 {
                op: Op32::Sub {
                    rd: IntRegister::num(1),
                    rs1: IntRegister::num(2),
                    rs2: IntRegister::num(3),
                },
                length: 4,
            }
        );
        assert_eq!(
            mkinst(0b000000000001_00000_000_00000_1110011),
            Inst32 {
                op: Op32::Ebreak,
                length: 4,
            }
        );
    }
}
