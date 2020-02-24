use crate::raw_instruction::RawInstruction;
use crate::register::FloatRegister;
use crate::register::IntRegister;

mod instruction_32;
pub use instruction_32::OperationRV32;

//mod instruction_64;
//pub use instruction_64::OperationRV64;

/// Represents a single operation that can be constructed by
/// decoding a `RawInstruction` value.
pub trait Operation {
    fn decode_raw(raw: RawInstruction) -> Self;
}

/// The RV32 implementation of `Operation` supports the operating encodings
/// from the RV32 base ISA and some of its extensions.
impl Operation for OperationRV32 {
    fn decode_raw(raw: RawInstruction) -> Self {
        OperationRV32::decode_from_raw(raw)
    }
}

/// Represents a decoded instruction ready to execute.
///
/// This type annotates an operation with its physical location in memory and
/// its original encoded length, so that an execution engine can properly
/// evaluate any PC-relative offsets in the operation and can locate the
/// non-branch successor instruction.
#[derive(Debug, PartialEq, Clone)]
pub struct Instruction<Op: Operation, Addr> {
    /// Op is the operation the instruction should perform.
    pub op: Op,

    /// The program counter value where the instruction was found. This can
    /// be used to interpret PC-relative offsets in the operation.
    pub pc: Addr,

    /// The length of the raw instruction this was decoded from. This will
    /// not necessarily match the canonical length of the associated
    /// operation, because the operation may have been normalized e.g. by
    /// replacing a compressed instruction with its standard-length equivalent.
    pub length: usize,
}

impl<Op: Operation, Addr> Instruction<Op, Addr> {
    /// Decodes the given raw instruction (using the associated `Operation`
    /// implementation) and then associates it with the given program counter
    /// value to produce an `Instruction` ready to execute.
    pub fn decode_raw(raw: RawInstruction, pc: Addr) -> Self {
        let length = raw.length();
        let op = Op::decode_raw(raw);
        return Self {
            op: op,
            pc: pc,
            length: length,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, OperationRV32, RawInstruction};
    use crate::register::{FloatRegister, IntRegister};
    type Inst32 = Instruction<OperationRV32, u32>;
    type Op32 = OperationRV32;

    #[test]
    fn instruction_rv32() {
        fn mkinst(raw: u32) -> Inst32 {
            Inst32::decode_raw(RawInstruction::new(raw), 0xdeadbeef)
        }

        assert_eq!(
            mkinst(0b0000000_00011_00010_000_00001_0110011),
            Inst32 {
                op: Op32::Add {
                    rd: IntRegister::num(1),
                    rs1: IntRegister::num(2),
                    rs2: IntRegister::num(3),
                },
                pc: 0xdeadbeef,
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
                pc: 0xdeadbeef,
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
                pc: 0xdeadbeef,
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
                pc: 0xdeadbeef,
                length: 4,
            }
        );
        assert_eq!(
            mkinst(0b000000000001_00000_000_00000_1110011),
            Inst32 {
                op: Op32::Ebreak,
                pc: 0xdeadbeef,
                length: 4,
            }
        );
    }
}
