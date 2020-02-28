use crate::data::Int;
use crate::exception::ExceptionCause;
use crate::hart::Hart;
use crate::instruction::Instruction;
use crate::instruction::OperationRV32;
use crate::memory::{Bus, MemoryError};
use crate::raw_instruction::RawInstruction;
use crate::register::IntRegister;

type Op = OperationRV32;

/// Performs a single execution step against the given RV32 hart.
///
/// An execution step is usually the execution of a single instruction, but
/// it can also include handling exceptions that are raised in retrieving the
/// next instruction from memory.
///
/// When this function returns, the state of the hart will have been modified
/// to reflect the side-effects of the action.
pub fn step_rv32<Mem: Bus<u32>>(hart: &mut impl Hart<u32, u32, f64, Mem>) {
    let pc = hart.read_pc();
    let raw_inst_result: Result<RawInstruction, MemoryError> =
        hart.with_memory(|mem| match mem.read_word(pc) {
            Err(e) => Err(e),
            Ok(data) => Ok(RawInstruction::new(data)),
        });

    match raw_inst_result {
        Ok(raw_inst) => {
            let inst = Instruction::<Op, u32>::decode_raw(raw_inst, pc);

            // We pre-increment the program counter to the default successor
            // instruction here because we've already captured the current
            // instruction's PC as part of inst above. Depending on which
            // instruction we've detected, dispatch_instruction below might
            // change the program counter again before it returns, overriding
            // this default.
            hart.write_pc(pc.wrapping_add(inst.length as u32));
            dispatch_instruction(inst, hart);
        }
        Err(e) => {
            let cause: ExceptionCause = match e {
                MemoryError::Misaligned => ExceptionCause::InstructionAddressMisaligned,
                MemoryError::AccessFault => ExceptionCause::InstructionPageFault,
                MemoryError::PageFault => ExceptionCause::InstructionPageFault,
            };
            hart.exception(cause);
            return;
        }
    }
}

// The main instruction dispatch logic for RV32: selects a suitable
// implementation function based on the specific operation in the instruction.
fn dispatch_instruction<Mem: Bus<u32>>(
    inst: Instruction<Op, u32>,
    hart: &mut impl Hart<u32, u32, f64, Mem>,
) {
    match inst.op {
        Op::Add { rd, rs1, rs2 } => exec_add(hart, inst, rd, rs1, rs2),
        Op::Addi { rd, rs1, simm } => exec_addi(hart, inst, rd, rs1, simm),
        _ => hart.exception(ExceptionCause::IllegalInstruction),
    };
}

// Add: Add rs2 to rs1 and place the result into rd.
//
// > rd ← sx(rs1) + sx(rs2)
fn exec_add<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) {
    let a = hart.read_int_register(rs1).to_signed();
    let b = hart.read_int_register(rs2).to_signed();
    let result = a.wrapping_add(b);
    hart.write_int_register(rd, u32::from_signed(result))
}

// Add Immediate: Add sign-extended 12-bit immediate to register rs1 and place the result in rd.
//
// > rd ← rs1 + sx(imm)
fn exec_addi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) {
    let a = hart.read_int_register(rs1).to_signed();
    let b = simm;
    let result = a.wrapping_add(b);
    hart.write_int_register(rd, u32::from_signed(result))
}
