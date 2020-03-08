use crate::data::sign_extend;
use crate::data::{Float, Int};
use crate::exception::ExceptionCause;
use crate::exec::ExecStatus;
use crate::hart::Hart;
use crate::instruction::Instruction;
use crate::instruction::OperationRV32;
use crate::memory::{Bus, MemoryError};
use crate::raw_instruction::RawInstruction;
use crate::register::{ControlStatusRegister, FloatRegister, IntRegister};

type Op = OperationRV32;

/// Performs a single execution step against the given RV32 hart.
///
/// An execution step is usually the execution of a single instruction, but
/// it can also include handling exceptions that are raised in retrieving the
/// next instruction from memory.
///
/// When this function returns, the state of the hart will have been modified
/// to reflect the side-effects of the action.
pub fn step_rv32<Mem: Bus<u32>>(hart: &mut impl Hart<u32, u32, f64, Mem>) -> ExecStatus<u32> {
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
            return dispatch_instruction(inst, hart);
        }
        Err(e) => {
            let cause: ExceptionCause = match e {
                MemoryError::Misaligned => ExceptionCause::InstructionAddressMisaligned,
                MemoryError::AccessFault => ExceptionCause::InstructionPageFault,
                MemoryError::PageFault => ExceptionCause::InstructionPageFault,
            };
            hart.exception(cause);
            return ExecStatus::Running;
        }
    }
}

// The main instruction dispatch logic for RV32: selects a suitable
// implementation function based on the specific operation in the instruction.
fn dispatch_instruction<Mem: Bus<u32>>(
    inst: Instruction<Op, u32>,
    hart: &mut impl Hart<u32, u32, f64, Mem>,
) -> ExecStatus<u32> {
    match inst.op {
        Op::Add { rd, rs1, rs2 } => exec_add(hart, inst, rd, rs1, rs2),
        Op::Addi { rd, rs1, simm } => exec_addi(hart, inst, rd, rs1, simm),
        Op::AmoaddW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amoadd_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmoandW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amoand_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmomaxW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amomax_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmomaxuW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amomaxu_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmominW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amomin_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmominuW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amominu_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmoorW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amoor_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmoswapW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amoswap_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::AmoxorW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_amoxor_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::And { rd, rs1, rs2 } => exec_and(hart, inst, rd, rs1, rs2),
        Op::Andi { rd, rs1, simm } => exec_andi(hart, inst, rd, rs1, simm),
        Op::Auipc { rd, simm } => exec_auipc(hart, inst, rd, simm),
        Op::Beq { rs1, rs2, simm } => exec_beq(hart, inst, rs1, rs2, simm),
        Op::Bge { rs1, rs2, simm } => exec_bge(hart, inst, rs1, rs2, simm),
        Op::Bgeu { rs1, rs2, simm } => exec_bgeu(hart, inst, rs1, rs2, simm),
        Op::Blt { rs1, rs2, simm } => exec_blt(hart, inst, rs1, rs2, simm),
        Op::Bltu { rs1, rs2, simm } => exec_bltu(hart, inst, rs1, rs2, simm),
        Op::Bne { rs1, rs2, simm } => exec_bne(hart, inst, rs1, rs2, simm),
        Op::CAdd { rs1rd, rs2 } => exec_c_add(hart, inst, rs1rd, rs2),
        Op::CAddi { rs1rd, nzsimm } => exec_c_addi(hart, inst, rs1rd, nzsimm),
        Op::CAddi16Sp { rs1rd, nzsimm } => exec_c_addi16sp(hart, inst, rs1rd, nzsimm),
        Op::CAddi4Spn { rd, nzuimm } => exec_c_addi4spn(hart, inst, rd, nzuimm),
        Op::CAddw { rs1rd, rs2 } => exec_c_addw(hart, inst, rs1rd, rs2),
        Op::CAnd { rs1rd, rs2 } => exec_c_and(hart, inst, rs1rd, rs2),
        Op::CAndi { rs1rd, nzsimm } => exec_c_andi(hart, inst, rs1rd, nzsimm),
        Op::CBeqz { rs1, simm } => exec_c_beqz(hart, inst, rs1, simm),
        Op::CBnez { rs1, simm } => exec_c_bnez(hart, inst, rs1, simm),
        Op::CEbreak => exec_c_ebreak(hart, inst),
        Op::CFld { frd, rs1, uimm } => exec_c_fld(hart, inst, frd, rs1, uimm),
        Op::CFldsp { frd, uimm } => exec_c_fldsp(hart, inst, frd, uimm),
        Op::CFlw { frd, rs1, uimm } => exec_c_flw(hart, inst, frd, rs1, uimm),
        Op::CFlwsp { frd, uimm } => exec_c_flwsp(hart, inst, frd, uimm),
        Op::CFsd { rs1, frs2, uimm } => exec_c_fsd(hart, inst, rs1, frs2, uimm),
        Op::CFsdsp { frs2, uimm } => exec_c_fsdsp(hart, inst, frs2, uimm),
        Op::CFsw { rs1, frs2, uimm } => exec_c_fsw(hart, inst, rs1, frs2, uimm),
        Op::CFswsp { frs2, uimm } => exec_c_fswsp(hart, inst, frs2, uimm),
        Op::CJ { simm } => exec_c_j(hart, inst, simm),
        Op::CJal { simm } => exec_c_jal(hart, inst, simm),
        Op::CJalr { rd, rs1 } => exec_c_jalr(hart, inst, rd, rs1),
        Op::CJr { rd, rs1 } => exec_c_jr(hart, inst, rd, rs1),
        Op::CLi { rs1rd, simm } => exec_c_li(hart, inst, rs1rd, simm),
        Op::CLui { rd, nzsimm } => exec_c_lui(hart, inst, rd, nzsimm),
        Op::CLw { rd, rs1, uimm } => exec_c_lw(hart, inst, rd, rs1, uimm),
        Op::CLwsp { rd, uimm } => exec_c_lwsp(hart, inst, rd, uimm),
        Op::CMv { rd, rs2 } => exec_c_mv(hart, inst, rd, rs2),
        Op::CNop => exec_c_nop(hart, inst),
        Op::COr { rs1rd, rs2 } => exec_c_or(hart, inst, rs1rd, rs2),
        Op::CSlli { rs1rd, nzuimm } => exec_c_slli(hart, inst, rs1rd, nzuimm),
        Op::CSrai { rs1rd, nzuimm } => exec_c_srai(hart, inst, rs1rd, nzuimm),
        Op::CSrli { rs1rd, nzuimm } => exec_c_srli(hart, inst, rs1rd, nzuimm),
        Op::CSub { rs1rd, rs2 } => exec_c_sub(hart, inst, rs1rd, rs2),
        Op::CSubw { rs1rd, rs2 } => exec_c_subw(hart, inst, rs1rd, rs2),
        Op::CSw { rs1, rs2, uimm } => exec_c_sw(hart, inst, rs1, rs2, uimm),
        Op::CSwsp { rs2, uimm } => exec_c_swsp(hart, inst, rs2, uimm),
        Op::CXor { rs1rd, rs2 } => exec_c_xor(hart, inst, rs1rd, rs2),
        Op::Csrrc { rd, rs1, csr } => exec_csrrc(hart, inst, rd, rs1, csr),
        Op::Csrrci { rd, uimm, csr } => exec_csrrci(hart, inst, rd, uimm, csr),
        Op::Csrrs { rd, rs1, csr } => exec_csrrs(hart, inst, rd, rs1, csr),
        Op::Csrrsi { rd, uimm, csr } => exec_csrrsi(hart, inst, rd, uimm, csr),
        Op::Csrrw { rd, rs1, csr } => exec_csrrw(hart, inst, rd, rs1, csr),
        Op::Csrrwi { rd, uimm, csr } => exec_csrrwi(hart, inst, rd, uimm, csr),
        Op::Div { rd, rs1, rs2 } => exec_div(hart, inst, rd, rs1, rs2),
        Op::Divu { rd, rs1, rs2 } => exec_divu(hart, inst, rd, rs1, rs2),
        Op::Dret => exec_dret(hart, inst),
        Op::Ebreak => exec_ebreak(hart, inst),
        Op::Ecall => exec_ecall(hart, inst),
        Op::FaddD {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fadd_d(hart, inst, frd, frs1, frs2, rm),
        Op::FaddQ {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fadd_q(hart, inst, frd, frs1, frs2, rm),
        Op::FaddS {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fadd_s(hart, inst, frd, frs1, frs2, rm),
        Op::FclassD { rd, frs1 } => exec_fclass_d(hart, inst, rd, frs1),
        Op::FclassQ { rd, frs1 } => exec_fclass_q(hart, inst, rd, frs1),
        Op::FclassS { rd, frs1 } => exec_fclass_s(hart, inst, rd, frs1),
        Op::FcvtDQ { frd, frs1, rm } => exec_fcvt_d_q(hart, inst, frd, frs1, rm),
        Op::FcvtDS { frd, frs1, rm } => exec_fcvt_d_s(hart, inst, frd, frs1, rm),
        Op::FcvtDW { frd, rs1, rm } => exec_fcvt_d_w(hart, inst, frd, rs1, rm),
        Op::FcvtDWu { frd, rs1, rm } => exec_fcvt_d_wu(hart, inst, frd, rs1, rm),
        Op::FcvtQD { frd, frs1, rm } => exec_fcvt_q_d(hart, inst, frd, frs1, rm),
        Op::FcvtQS { frd, frs1, rm } => exec_fcvt_q_s(hart, inst, frd, frs1, rm),
        Op::FcvtQW { frd, rs1, rm } => exec_fcvt_q_w(hart, inst, frd, rs1, rm),
        Op::FcvtQWu { frd, rs1, rm } => exec_fcvt_q_wu(hart, inst, frd, rs1, rm),
        Op::FcvtSD { frd, frs1, rm } => exec_fcvt_s_d(hart, inst, frd, frs1, rm),
        Op::FcvtSQ { frd, frs1, rm } => exec_fcvt_s_q(hart, inst, frd, frs1, rm),
        Op::FcvtSW { frd, rs1, rm } => exec_fcvt_s_w(hart, inst, frd, rs1, rm),
        Op::FcvtSWu { frd, rs1, rm } => exec_fcvt_s_wu(hart, inst, frd, rs1, rm),
        Op::FcvtWD { rd, frs1, rm } => exec_fcvt_w_d(hart, inst, rd, frs1, rm),
        Op::FcvtWQ { rd, frs1, rm } => exec_fcvt_w_q(hart, inst, rd, frs1, rm),
        Op::FcvtWS { rd, frs1, rm } => exec_fcvt_w_s(hart, inst, rd, frs1, rm),
        Op::FcvtWuD { rd, frs1, rm } => exec_fcvt_wu_d(hart, inst, rd, frs1, rm),
        Op::FcvtWuQ { rd, frs1, rm } => exec_fcvt_wu_q(hart, inst, rd, frs1, rm),
        Op::FcvtWuS { rd, frs1, rm } => exec_fcvt_wu_s(hart, inst, rd, frs1, rm),
        Op::FdivD {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fdiv_d(hart, inst, frd, frs1, frs2, rm),
        Op::FdivQ {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fdiv_q(hart, inst, frd, frs1, frs2, rm),
        Op::FdivS {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fdiv_s(hart, inst, frd, frs1, frs2, rm),
        Op::Fence { pred, succ } => exec_fence(hart, inst, pred, succ),
        Op::FenceI => exec_fence_i(hart, inst),
        Op::FeqD { rd, frs1, frs2 } => exec_feq_d(hart, inst, rd, frs1, frs2),
        Op::FeqQ { rd, frs1, frs2 } => exec_feq_q(hart, inst, rd, frs1, frs2),
        Op::FeqS { rd, frs1, frs2 } => exec_feq_s(hart, inst, rd, frs1, frs2),
        Op::Fld { frd, rs1, simm } => exec_fld(hart, inst, frd, rs1, simm),
        Op::FleD { rd, frs1, frs2 } => exec_fle_d(hart, inst, rd, frs1, frs2),
        Op::FleQ { rd, frs1, frs2 } => exec_fle_q(hart, inst, rd, frs1, frs2),
        Op::FleS { rd, frs1, frs2 } => exec_fle_s(hart, inst, rd, frs1, frs2),
        Op::Flq { frd, rs1, simm } => exec_flq(hart, inst, frd, rs1, simm),
        Op::FltD { rd, frs1, frs2 } => exec_flt_d(hart, inst, rd, frs1, frs2),
        Op::FltQ { rd, frs1, frs2 } => exec_flt_q(hart, inst, rd, frs1, frs2),
        Op::FltS { rd, frs1, frs2 } => exec_flt_s(hart, inst, rd, frs1, frs2),
        Op::Flw { frd, rs1, simm } => exec_flw(hart, inst, frd, rs1, simm),
        Op::FmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmadd_d(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmaddQ {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmadd_q(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmadd_s(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmaxD { frd, frs1, frs2 } => exec_fmax_d(hart, inst, frd, frs1, frs2),
        Op::FmaxQ { frd, frs1, frs2 } => exec_fmax_q(hart, inst, frd, frs1, frs2),
        Op::FmaxS { frd, frs1, frs2 } => exec_fmax_s(hart, inst, frd, frs1, frs2),
        Op::FminD { frd, frs1, frs2 } => exec_fmin_d(hart, inst, frd, frs1, frs2),
        Op::FminQ { frd, frs1, frs2 } => exec_fmin_q(hart, inst, frd, frs1, frs2),
        Op::FminS { frd, frs1, frs2 } => exec_fmin_s(hart, inst, frd, frs1, frs2),
        Op::FmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmsub_d(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmsubQ {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmsub_q(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fmsub_s(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FmulD {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fmul_d(hart, inst, frd, frs1, frs2, rm),
        Op::FmulQ {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fmul_q(hart, inst, frd, frs1, frs2, rm),
        Op::FmulS {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fmul_s(hart, inst, frd, frs1, frs2, rm),
        Op::FmvSX { frd, rs1 } => exec_fmv_s_x(hart, inst, frd, rs1),
        Op::FmvXS { rd, frs1 } => exec_fmv_x_s(hart, inst, rd, frs1),
        Op::FnmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmadd_d(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FnmaddQ {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmadd_q(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FnmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmadd_s(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FnmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmsub_d(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FnmsubQ {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmsub_q(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::FnmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => exec_fnmsub_s(hart, inst, frd, frs1, frs2, frs3, rm),
        Op::Fsd { rs1, frs2, simm } => exec_fsd(hart, inst, rs1, frs2, simm),
        Op::FsgnjD { frd, frs1, frs2 } => exec_fsgnj_d(hart, inst, frd, frs1, frs2),
        Op::FsgnjQ { frd, frs1, frs2 } => exec_fsgnj_q(hart, inst, frd, frs1, frs2),
        Op::FsgnjS { frd, frs1, frs2 } => exec_fsgnj_s(hart, inst, frd, frs1, frs2),
        Op::FsgnjnD { frd, frs1, frs2 } => exec_fsgnjn_d(hart, inst, frd, frs1, frs2),
        Op::FsgnjnQ { frd, frs1, frs2 } => exec_fsgnjn_q(hart, inst, frd, frs1, frs2),
        Op::FsgnjnS { frd, frs1, frs2 } => exec_fsgnjn_s(hart, inst, frd, frs1, frs2),
        Op::FsgnjxD { frd, frs1, frs2 } => exec_fsgnjx_d(hart, inst, frd, frs1, frs2),
        Op::FsgnjxQ { frd, frs1, frs2 } => exec_fsgnjx_q(hart, inst, frd, frs1, frs2),
        Op::FsgnjxS { frd, frs1, frs2 } => exec_fsgnjx_s(hart, inst, frd, frs1, frs2),
        Op::Fsq { rs1, frs2, simm } => exec_fsq(hart, inst, rs1, frs2, simm),
        Op::FsqrtD { frd, frs1, rm } => exec_fsqrt_d(hart, inst, frd, frs1, rm),
        Op::FsqrtQ { frd, frs1, rm } => exec_fsqrt_q(hart, inst, frd, frs1, rm),
        Op::FsqrtS { frd, frs1, rm } => exec_fsqrt_s(hart, inst, frd, frs1, rm),
        Op::FsubD {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fsub_d(hart, inst, frd, frs1, frs2, rm),
        Op::FsubQ {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fsub_q(hart, inst, frd, frs1, frs2, rm),
        Op::FsubS {
            frd,
            frs1,
            frs2,
            rm,
        } => exec_fsub_s(hart, inst, frd, frs1, frs2, rm),
        Op::Fsw { rs1, frs2, simm } => exec_fsw(hart, inst, rs1, frs2, simm),
        Op::Hret => exec_hret(hart, inst),
        Op::Jal { rd, simm } => exec_jal(hart, inst, rd, simm),
        Op::Jalr { rd, rs1, simm } => exec_jalr(hart, inst, rd, rs1, simm),
        Op::Lb { rd, rs1, simm } => exec_lb(hart, inst, rd, rs1, simm),
        Op::Lbu { rd, rs1, simm } => exec_lbu(hart, inst, rd, rs1, simm),
        Op::Lh { rd, rs1, simm } => exec_lh(hart, inst, rd, rs1, simm),
        Op::Lhu { rd, rs1, simm } => exec_lhu(hart, inst, rd, rs1, simm),
        Op::LrW { rd, rs1, aq, rl } => exec_lr_w(hart, inst, rd, rs1, aq, rl),
        Op::Lui { rd, simm } => exec_lui(hart, inst, rd, simm),
        Op::Lw { rd, rs1, simm } => exec_lw(hart, inst, rd, rs1, simm),
        Op::Mret => exec_mret(hart, inst),
        Op::Mul { rd, rs1, rs2 } => exec_mul(hart, inst, rd, rs1, rs2),
        Op::Mulh { rd, rs1, rs2 } => exec_mulh(hart, inst, rd, rs1, rs2),
        Op::Mulhsu { rd, rs1, rs2 } => exec_mulhsu(hart, inst, rd, rs1, rs2),
        Op::Mulhu { rd, rs1, rs2 } => exec_mulhu(hart, inst, rd, rs1, rs2),
        Op::Or { rd, rs1, rs2 } => exec_or(hart, inst, rd, rs1, rs2),
        Op::Ori { rd, rs1, simm } => exec_ori(hart, inst, rd, rs1, simm),
        Op::Rem { rd, rs1, rs2 } => exec_rem(hart, inst, rd, rs1, rs2),
        Op::Remu { rd, rs1, rs2 } => exec_remu(hart, inst, rd, rs1, rs2),
        Op::Sb { rs1, rs2, simm } => exec_sb(hart, inst, rs1, rs2, simm),
        Op::ScW {
            rd,
            rs1,
            rs2,
            aq,
            rl,
        } => exec_sc_w(hart, inst, rd, rs1, rs2, aq, rl),
        Op::SfenceVm { rs1 } => exec_sfence_vm(hart, inst, rs1),
        Op::SfenceVma { rs1, rs2 } => exec_sfence_vma(hart, inst, rs1, rs2),
        Op::Sh { rs1, rs2, simm } => exec_sh(hart, inst, rs1, rs2, simm),
        Op::Sll { rd, rs1, rs2 } => exec_sll(hart, inst, rd, rs1, rs2),
        Op::Slli { rd, rs1, shamt } => exec_slli(hart, inst, rd, rs1, shamt),
        Op::Slt { rd, rs1, rs2 } => exec_slt(hart, inst, rd, rs1, rs2),
        Op::Slti { rd, rs1, simm } => exec_slti(hart, inst, rd, rs1, simm),
        Op::Sltiu { rd, rs1, simm } => exec_sltiu(hart, inst, rd, rs1, simm),
        Op::Sltu { rd, rs1, rs2 } => exec_sltu(hart, inst, rd, rs1, rs2),
        Op::Sra { rd, rs1, rs2 } => exec_sra(hart, inst, rd, rs1, rs2),
        Op::Srai { rd, rs1, shamt } => exec_srai(hart, inst, rd, rs1, shamt),
        Op::Sret => exec_sret(hart, inst),
        Op::Srl { rd, rs1, rs2 } => exec_srl(hart, inst, rd, rs1, rs2),
        Op::Srli { rd, rs1, shamt } => exec_srli(hart, inst, rd, rs1, shamt),
        Op::Sub { rd, rs1, rs2 } => exec_sub(hart, inst, rd, rs1, rs2),
        Op::Sw { rs1, rs2, simm } => exec_sw(hart, inst, rs1, rs2, simm),
        Op::Uret => exec_uret(hart, inst),
        Op::Wfi => exec_wfi(hart, inst),
        Op::Xor { rd, rs1, rs2 } => exec_xor(hart, inst, rd, rs1, rs2),
        Op::Xori { rd, rs1, simm } => exec_xori(hart, inst, rd, rs1, simm),
        _ => {
            hart.exception(ExceptionCause::IllegalInstruction);
            ExecStatus::Running
        }
    }
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
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        u32::from_signed(a.to_signed().wrapping_add(b.to_signed()))
    })
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
) -> ExecStatus<u32> {
    exec_binary_op_imm(hart, rd, rs1, simm, |a, b| {
        u32::from_signed(a.to_signed().wrapping_add(b))
    })
}

// Atomic Add Word: Load word from address in rs1 into rd, add rd and rs2, write the result to the address in rs1.
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32(rs2) + s32[rs1]
fn exec_amoadd_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic And Word: Load word from address in rs1 into rd, and rd and rs2, write the result to the address in rs1.
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32(rs2) ∧ s32[rs1]
fn exec_amoand_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Maximum Word: Load word from address in rs1 into rd, find maximum of rd and rs2, write the result to the address in rs1 (signed).
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32_max(s32(rs2), s32[rs1])
fn exec_amomax_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Maximum Unsigned Word: Load word from address in rs1 into rd, find maximum of rd and rs2, write the result to the address in rs1 (unsigned).
//
// > rd ← s32[rs1] ∥ u32[rs1] ← u32_max(u32(rs2), u32[rs1])
fn exec_amomaxu_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Minimum Word: Load word from address in rs1 into rd, find minimum of rd and rs2, write the result to the address in rs1 (signed).
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32_min(s32(rs2), s32[rs1])
fn exec_amomin_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Minimum Unsigned Word: Load word from address in rs1 into rd, find minimum of rd and rs2, write the result to the address in rs1 (unsigned).
//
// > rd ← s32[rs1] ∥ u32[rs1] ← u32_min(u32(rs2), u32[rs1])
fn exec_amominu_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Or Word: Load word from address in rs1 into rd, or rd and rs2, write the result to the address in rs1.
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32(rs2) ∨ s32[rs1]
fn exec_amoor_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Swap Word: Load word from address in rs1 into rd, swap rd and rs2, write the result to the address in rs1.
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32(rs2)
fn exec_amoswap_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Atomic Xor Word: Load word from address in rs1 into rd, xor rd and rs2, write the result to the address in rs1.
//
// > rd ← s32[rs1] ∥ u32[rs1] ← s32(rs2) ⊻ s32[rs1]
fn exec_amoxor_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// And: Set rd to the bitwise and of rs1 and rs2.
//
// > rd ← ux(rs1) ∧ ux(rs2)
fn exec_and<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| a & b)
}

// And Immediate: Set rd to the bitwise and of rs1 with the sign-extended 12-bit immediate.
//
// > rd ← ux(rs1) ∧ ux(imm)
fn exec_andi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_binary_op_imm(hart, rd, rs1, simm, |a, b| a & u32::from_signed(b))
}

// Add Upper Immediate to PC: Place the PC plus the 20-bit signed immediate (shited 12 bits left) into rd (used before JALR).
//
// > rd ← pc + imm
fn exec_auipc<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rd: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    let result = inst.pc.wrapping_add(u32::from_signed(simm).to_unsigned());
    hart.write_int_register(rd, u32::from_unsigned(result));
    ExecStatus::Running
}

// Branch Equal: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 == rs2.
//
// > if rs1 = rs2 then pc ← pc + imm
fn exec_beq<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| a == b)
}

// Branch Greater than Equal: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 >= rs2 (signed).
//
// > if rs1 ≥ rs2 then pc ← pc + imm
fn exec_bge<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| {
        a.to_signed() >= b.to_signed()
    })
}

// Branch Greater than Equal Unsigned: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 >= rs2 (unsigned).
//
// > if rs1 ≥ rs2 then pc ← pc + imm
fn exec_bgeu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| {
        a.to_unsigned() >= b.to_unsigned()
    })
}

// Branch Less Than: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 < rs2 (signed).
//
// > if rs1 < rs2 then pc ← pc + imm
fn exec_blt<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| {
        a.to_signed() < b.to_signed()
    })
}

// Branch Less Than Unsigned: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 < rs2 (unsigned).
//
// > if rs1 < rs2 then pc ← pc + imm
fn exec_bltu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| {
        a.to_unsigned() < b.to_unsigned()
    })
}

// Branch Not Equal: Branch to PC relative 12-bit signed immediate (shifted 1 bit left) if rs1 != rs2.
//
// > if rs1 ≠ rs2 then pc ← pc + imm
fn exec_bne<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_branch_binary_cond(hart, inst, rs1, rs2, simm, |a, b| a != b)
}

// : .
//
// >
fn exec_c_add<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_add(hart, inst, rs1rd, rs1rd, rs2)
}

// : .
//
// >
fn exec_c_addi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzsimm: i32,
) -> ExecStatus<u32> {
    exec_addi(hart, inst, rs1rd, rs1rd, nzsimm)
}

// : .
//
// >
fn exec_c_addi16sp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzsimm: i32,
) -> ExecStatus<u32> {
    exec_addi(hart, inst, rs1rd, IntRegister::numbered(2), nzsimm)
}

// : .
//
// >
fn exec_c_addi4spn<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rd: IntRegister,
    nzuimm: u32,
) -> ExecStatus<u32> {
    exec_addi(
        hart,
        inst,
        rd,
        IntRegister::numbered(2),
        u32::from_unsigned(nzuimm).to_signed(),
    )
}

// : .
//
// >
fn exec_c_addw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_and<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_and(hart, inst, rs1rd, rs1rd, rs2)
}

// : .
//
// >
fn exec_c_andi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzsimm: i32,
) -> ExecStatus<u32> {
    exec_andi(hart, inst, rs1rd, rs1rd, nzsimm)
}

// : .
//
// >
fn exec_c_beqz<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_beq(hart, inst, rs1, IntRegister::zero(), simm)
}

// : .
//
// >
fn exec_c_bnez<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_bne(hart, inst, rs1, IntRegister::zero(), simm)
}

// : .
//
// >
fn exec_c_ebreak<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    exec_ebreak(hart, inst)
}

// : .
//
// >
fn exec_c_fld<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: IntRegister,
    rs1: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_fldsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_flw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: IntRegister,
    rs1: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_flwsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_fsd<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    frs2: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_fsdsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frs2: FloatRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_fsw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    frs2: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_fswsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frs2: FloatRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_j<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_jal<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_jalr<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_jr<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_li<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_addi(hart, inst, rs1rd, IntRegister::zero(), simm)
}

// : .
//
// >
fn exec_c_lui<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rd: IntRegister,
    nzsimm: i32,
) -> ExecStatus<u32> {
    exec_lui(hart, inst, rd, nzsimm)
}

// : .
//
// >
fn exec_c_lw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_lwsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_mv<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_nop<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // Nothing to do.
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_or<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_or(hart, inst, rs1rd, rs1rd, rs2)
}

// : .
//
// >
fn exec_c_slli<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzuimm: u32,
) -> ExecStatus<u32> {
    exec_slli(hart, inst, rs1rd, rs1rd, nzuimm)
}

// : .
//
// >
fn exec_c_srai<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzuimm: u32,
) -> ExecStatus<u32> {
    exec_srai(hart, inst, rs1rd, rs1rd, nzuimm)
}

// : .
//
// >
fn exec_c_srli<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    nzuimm: u32,
) -> ExecStatus<u32> {
    exec_srli(hart, inst, rs1rd, rs1rd, nzuimm)
}

// : .
//
// >
fn exec_c_sub<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_sub(hart, inst, rs1rd, rs1rd, rs2)
}

// : .
//
// >
fn exec_c_subw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_sub(hart, inst, rs1rd, rs1rd, rs2)
}

// : .
//
// >
fn exec_c_sw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_c_swsp<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs2: IntRegister,
    uimm: u32,
) -> ExecStatus<u32> {
    exec_sw(
        hart,
        inst,
        IntRegister::numbered(2),
        rs2,
        u32::from_unsigned(uimm).to_signed(),
    )
}

// : .
//
// >
fn exec_c_xor<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1rd: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_xor(hart, inst, rs1rd, rs1rd, rs2)
}

// CSR Atomic Clear Bit: CSR Atomic Clear Bit reads the CSR, clears CSR bits set in rs1, and writes previous value to rd.
//
// >
fn exec_csrrc<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    csr: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// CSR Atomic Clear Bit Immediate: CSR Atomic Clear Bit Immediate reads the CSR, clears CSR bits set in the immediate, and writes previous value to rd.
//
// >
fn exec_csrrci<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    uimm: u32,
    csr: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// CSR Atomic Set Bit: CSR Atomic Set Bit reads the CSR, sets CSR bits set in rs1, and writes previous value to rd.
//
// >
fn exec_csrrs<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    csr: u32,
) -> ExecStatus<u32> {
    if rs1.num() == 0 {
        match hart.read_csr(ControlStatusRegister::numbered(csr as usize)) {
            Ok(result) => {
                hart.write_int_register(rd, u32::from_unsigned(result));
            }
            Err(e) => {
                hart.exception(ExceptionCause::IllegalInstruction);
            }
        };
    } else {
        // TODO: Implement the atomic read/or/write behavior for other rs1 registers
        hart.exception(ExceptionCause::IllegalInstruction);
    }
    ExecStatus::Running
}

// CSR Atomic Set Bit Immediate: CSR Atomic Set Bit Immediate reads the CSR, sets CSR bits set in the immediate, and writes previous value to rd.
//
// >
fn exec_csrrsi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    uimm: u32,
    csr: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// CSR Atomic Read Write: CSR Atomic Read Write writes the value in rs1 to the CSR, and writes previous value to rd.
//
// >
fn exec_csrrw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    csr: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// CSR Atomic Read Write Immediate: CSR Atomic Read Write Immediate writes the immediate value to the CSR, and writes previous value to rd.
//
// >
fn exec_csrrwi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    uimm: u32,
    csr: u32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Divide Signed: Divide rs1 (dividend) by rs2 (divisor) and place the quotient in rd (signed).
//
// > rd ← sx(rs1) ÷ sx(rs2)
fn exec_div<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_signed();
        let bw = b.to_signed();
        if bw == 0 {
            // Division by zero produces an all-ones result
            return u32::from_unsigned(0xffffffff);
        }
        if aw == -2147483648 && bw == -1 {
            // most negative value divided by -1 is an overflow
            return u32::from_signed(-2147483648);
        }
        let result = aw / bw;
        u32::from_signed(result)
    })
}

// Divide Unsigned: Divide rs1 (dividend) by rs2 (divisor) and place the quotient in rd (unsigned).
//
// > rd ← ux(rs1) ÷ ux(rs2)
fn exec_divu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_unsigned();
        let bw = b.to_unsigned();
        if bw == 0 {
            // Division by zero produces all-ones
            return u32::from_unsigned(0xffffffff);
        }
        let result = aw / bw;
        u32::from_unsigned(result)
    })
}

// Debug-Mode Return: .
//
// >
fn exec_dret<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Environment Break to Debugger: .
//
// >
fn exec_ebreak<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    if !hart.environment_break(inst.pc) {
        ExecStatus::EnvironmentBreak(inst.pc)
    } else {
        ExecStatus::Running
    }
}

// Environment Call: .
//
// >
fn exec_ecall<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    if !hart.environment_call(inst.pc) {
        ExecStatus::EnvironmentCall(inst.pc)
    } else {
        ExecStatus::Running
    }
}

// FP Add (DP): Add the double-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) + f64(frs2)
fn exec_fadd_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    hart.exception(ExceptionCause::IllegalInstruction);
    return ExecStatus::Running;

    let a = hart.read_float_register(frs1).to_double();
    let b = hart.read_float_register(frs2).to_double();
    let result = a + b;
    hart.write_float_register(frd, Float::from_double(result));
    // TODO: Handle "rm" argument
}

// FP Add (QP): Add the quadruple-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) + f128(frs2)
fn exec_fadd_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Add (SP): Add the single-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) + f32(frs2)
fn exec_fadd_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Classify (DP): Set rd to a 10-bit mask indicating the class of the double-precision value in frs1.
//
// > rd ← rd ← f64_classify(f64(frs1))
fn exec_fclass_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Classify (QP): Set rd to a 10-bit mask indicating the class of the quadruple-precision value in frs1.
//
// > rd ← rd ← f128_classify(f128(frs1))
fn exec_fclass_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Classify (SP): Set rd to a 10-bit mask indicating the class of the single-precision value in frs1.
//
// > rd ← f32_classify(f32(frs1))
fn exec_fclass_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert QP to DP: Convert the quadruple-precision value in frs1 to double-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f64(f128(frs1))
fn exec_fcvt_d_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert SP to DP: Convert the single-precision value in frs1 to double-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f64(f32(frs1))
fn exec_fcvt_d_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word to Float (DP): Convert the 64-bit signed integer in rs1 to a double-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f64(s32(rs1))
fn exec_fcvt_d_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word Unsigned to Float (DP): Convert the 64-bit unsigned integer in rs1 to a double-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f64(u32(rs1))
fn exec_fcvt_d_wu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert DP to QP: Convert the double-precision value in frs1 to quadruple-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f128(f64(frs1))
fn exec_fcvt_q_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert SP to QP: Convert the single-precision value in frs1 to quadruple-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f128(f32(frs1))
fn exec_fcvt_q_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word to Float (QP): Convert the 64-bit signed integer in rs1 to a quadruple-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f128(s32(rs1))
fn exec_fcvt_q_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word Unsigned to Float (QP): Convert the 64-bit unsigned integer in rs1 to a quadruple-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f128(u32(rs1))
fn exec_fcvt_q_wu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert DP to SP: Convert the double-precision value in frs1 to single-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f32(f64(frs1))
fn exec_fcvt_s_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert QP to SP: Convert the quadruple-precision value in frs1 to single-precision, then write the result to frd.
//
// > frm ← rm ; frd ← f32(f128(frs1))
fn exec_fcvt_s_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word to Float (SP): Convert the 32-bit signed integer in rs1 to a single-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f32(s32(rs1))
fn exec_fcvt_s_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Word Unsigned to Float (SP): Convert the 32-bit unsigned integer in rs1 to a single-precision value, then write the result to frd.
//
// > frm ← rm ; frd ← f32(u32(rs1))
fn exec_fcvt_s_wu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word (DP): Convert the double-precision value in frs1 to a 32-bit signed integer, then write the result to rd.
//
// > frm ← rm ; rd ← s32(f64(frs1))
fn exec_fcvt_w_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word (QP): Convert the quadruple-precision value in frs1 to a 32-bit signed integer, then write the result to rd.
//
// > frm ← rm ; rd ← s32(f128(frs1))
fn exec_fcvt_w_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word (SP): Convert the single-precision value in frs1 to a 32-bit signed integer, then write the result to rd.
//
// > frm ← rm ; rd ← s32(f32(frs1))
fn exec_fcvt_w_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word Unsigned (DP): Convert the double-precision value in frs1 to a 32-bit unsigned integer, then write the result to rd.
//
// > frm ← rm ; if f64(frs1) > 0 then rd ← u32(f64(frs1) else rd ← 0
fn exec_fcvt_wu_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word Unsigned (QP): Convert the quadruple-precision value in frs1 to a 32-bit unsigned integer, then write the result to rd.
//
// > frm ← rm ; if f128(frs1) > 0 then rd ← u32(f128(frs1) else rd ← 0
fn exec_fcvt_wu_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Convert Float to Word Unsigned (SP): Convert the single-precision value in frs1 to a 32-bit unsigned integer, then write the result to rd.
//
// > frm ← rm ; if f32(frs1) > 0 then rd ← u32(f32(frs1) else rd ← 0
fn exec_fcvt_wu_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Divide (DP): Divide the double-precision value in frs1 into frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) ÷ f64(frs2)
fn exec_fdiv_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Divide (QP): Divide the quadruple-precision value in frs1 into frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) ÷ f128(frs2)
fn exec_fdiv_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Divide (SP): Divide the single-precision value in frs1 into frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) ÷ f32(frs2)
fn exec_fdiv_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Fence: Order device I/O and memory accesses viewed by other threads and devices.
//
// >
fn exec_fence<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    _pred: bool,
    _succ: bool,
) -> ExecStatus<u32> {
    hart.fence_data();
    ExecStatus::Running
}

// Fence Instruction: Synchronize the instruction and data streams.
//
// >
fn exec_fence_i<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    hart.fence_code();
    ExecStatus::Running
}

// FP Equal (DP): Set rd to 1 if frs1 is equal to frs2, otherwise set rd to 0.
//
// > if f64(frs1) = f64(frs2) then rd ← 1 else rd ← 0
fn exec_feq_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Equal (QP): Set rd to 1 if frs1 is equal to frs2, otherwise set rd to 0.
//
// > if f128(frs1) = f128(frs2) then rd ← 1 else rd ← 0
fn exec_feq_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Equal (SP): Set rd to 1 if the single-precision value in frs1 is equal to frs2, otherwise set rd to 0.
//
// > if f32(frs1) = f32(frs2) then rd ← 1 else rd ← 0
fn exec_feq_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Load (DP): Loads a double-precision foating-point value from memory into foating-point register frd.
//
// > frd ← f64[rs1 + imm]
fn exec_fld<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than Equal (DP): Set rd to 1 if frs1 is less than or equal to frs2, otherwise set rd to 0.
//
// > if f64(frs1) ≤ f64(frs2) then rd ← 1 else rd ← 0
fn exec_fle_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than Equal (QP): Set rd to 1 if frs1 is less than or equal to frs2, otherwise set rd to 0.
//
// > if f128(frs1) ≤ f128(frs2) then rd ← 1 else rd ← 0
fn exec_fle_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than Equal (SP): Set rd to 1 if the single-precision value in frs1 is less than or equal to frs2, otherwise set rd to 0.
//
// > if f32(frs1) ≤ f32(frs2) then rd ← 1 else rd ← 0
fn exec_fle_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Load (QP): Loads a quadruple-precision foating-point value from memory into foating-point register frd.
//
// > frd ← f128[rs1 + imm]
fn exec_flq<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than (DP): Set rd to 1 if frs1 is less than frs2, otherwise set rd to 0.
//
// > if f64(frs1) < f64(frs2) then rd ← 1 else rd ← 0
fn exec_flt_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than (QP): Set rd to 1 if frs1 is less than frs2, otherwise set rd to 0.
//
// > if f128(frs1) < f128(frs2) then rd ← 1 else rd ← 0
fn exec_flt_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Less Than (SP): Set rd to 1 if the single-precision value in frs1 is less than frs2, otherwise set rd to 0.
//
// > if f32(frs1) < f32(frs2) then rd ← 1 else rd ← 0
fn exec_flt_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Load (SP): Loads a single-precision foating-point value from memory into foating-point register frd.
//
// > frd ← f32[rs1 + imm]
fn exec_flw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Add (DP): Multiply the double-precision values in frs1 and frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) × f64(frs2) + f64(frs3)
fn exec_fmadd_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Add (QP): Multiply the quadruple-precision values in frs1 and frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) × f128(frs2) + f128(frs3)
fn exec_fmadd_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Add (SP): Multiply the single-precision values in frs1 and frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) × f32(frs2) + f32(frs3)
fn exec_fmadd_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Maximum (DP): .
//
// > frd ← f64_max(f64(frs1), f64(frs2))
fn exec_fmax_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Maximum (QP): .
//
// > frd ← f128_max(f128(frs1), f128(frs2))
fn exec_fmax_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Maximum (SP): Take the larger quadruple-precision value from frs1 and frs2, then write the result to frd.
//
// > frd ← f32_max(f32(frs1), f32(frs2))
fn exec_fmax_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Minimum (DP): .
//
// > frd ← f64_min(f64(frs1), f64(frs2))
fn exec_fmin_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Minimum (QP): .
//
// > frd ← f128_min(f128(frs1), f128(frs2))
fn exec_fmin_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Minimum (SP): Take the smaller quadruple-precision value from frs1 and frs2, then write the result to frd.
//
// > frd ← f32_min(f32(frs1), f32(frs2))
fn exec_fmin_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Subtract (DP): Multiply the double-precision values in frs1 and frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) × f64(frs2) - f64(frs3)
fn exec_fmsub_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Subtract (QP): Multiply the quadruple-precision values in frs1 and frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) × f128(frs2) - f128(frs3)
fn exec_fmsub_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Fused Multiply Subtract (SP): Multiply the single-precision values in frs1 and frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) × f32(frs2) - f32(frs3)
fn exec_fmsub_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Multiply (DP): Multiply the double-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) × f64(frs2)
fn exec_fmul_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Multiply (QP): Multiply the quadruple-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) × f128(frs2)
fn exec_fmul_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Multiply (SP): Multiply the single-precision values in frs1 and frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) × f32(frs2)
fn exec_fmul_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Move from Integer Register (SP): Write the lower 32-bits of the integer register rs1 into the single-precision register frd.
//
// > frd ← s32(rs1)
fn exec_fmv_s_x<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    rs1: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Move to Integer Register (SP): Write the sign extended single-precision value in frs1 into the integer register rd.
//
// > rd ← s32(frs1)
fn exec_fmv_x_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    frs1: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Add (DP): Multiply the double-precision value in frs1 with the negated value in frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) × -f64(frs2) - f64(frs3)
fn exec_fnmadd_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Add (QP): Multiply the quadruple-precision value in frs1 with the negated value in frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) × -f128(frs2) - f128(frs3)
fn exec_fnmadd_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Add (SP): Multiply the single-precision value in frs1 with the negated value in frs2, then subtract rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) × -f32(frs2) - f32(frs3)
fn exec_fnmadd_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Subtract (DP): Multiply the double-precision value in frs1 with the negated value in frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) × -f64(frs2) + f64(frs3)
fn exec_fnmsub_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Subtract (QP): Multiply the quadruple-precision value in frs1 with the negated value in frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) × -f128(frs2) + f128(frs3)
fn exec_fnmsub_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Negate fused Multiply Subtract (SP): Multiply the single-precision value in frs1 with the negated value in frs2, then add rs3 and write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) × -f32(frs2) + f32(frs3)
fn exec_fnmsub_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    frs3: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Store (DP): Stores a double-precision foating-point value from foating-point register frs2 to memory.
//
// > f64[rs1 + imm] ← f64(frs2)
fn exec_fsd<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    frs2: FloatRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection (DP): Take the double-precision value from frs1 and inject the sign bit from frs2, then write the result to frd.
//
// > frd ← f64_copysign(f64(frs1), f64(frs2))
fn exec_fsgnj_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection (QP): Take the quadruple-precision value from frs1 and inject the sign bit from frs2, then write the result to frd.
//
// > frd ← f128_copysign(f128(frs1), f128(frs2))
fn exec_fsgnj_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Sign-injection (SP): Take the single-precision value from frs1 and inject the sign bit from frs2, then write the result to frd.
//
// > frd ← f32_copysign(f32(frs1), f32(frs2))
fn exec_fsgnj_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection Negate (DP): Take the double-precision value from frs1 and inject the negated sign bit from frs2, then write the result to frd.
//
// > frd ← f64_copysign(f64(frs1), -f64(frs2))
fn exec_fsgnjn_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection Negate (QP): Take the quadruple-precision value from frs1 and inject the negated sign bit from frs2, then write the result to frd.
//
// > frd ← f128_copysign(f128(frs1), -f128(frs2))
fn exec_fsgnjn_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Sign-injection Negate (SP): Take the single-precision value from frs1 and inject the negated sign bit from frs2, then write the result to frd.
//
// > frd ← f32_copysign(f32(frs1), -f32(frs2))
fn exec_fsgnjn_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection Xor (DP): Take the double-precision value from frs1 and inject the xor of the sign bits frs1 and frs2, then write the result to frd.
//
// > frd ← f64_xorsign(f64(frs1), f64(frs2))
fn exec_fsgnjx_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP to Sign-injection Xor (QP): Take the quadruple-precision value from frs1 and inject the xor of the sign bits frs1 and frs2, then write the result to frd.
//
// > frd ← f128_xorsign(f128(frs1), f128(frs2))
fn exec_fsgnjx_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Sign-injection Xor (SP): Take the single-precision value from frs1 and inject the xor of the sign bits frs1 and frs2, then write the result to frd.
//
// > frd ← f32_xorsign(f32(frs1), f32(frs2))
fn exec_fsgnjx_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Store (QP): Stores a quadruple-precision foating-point value from foating-point register frs2 to memory.
//
// > f128[rs1 + imm] ← f128(frs2)
fn exec_fsq<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    frs2: FloatRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Floating Square Root (DP): Calculate the square root of the double-precision value in frs1, then write the result to frd.
//
// > frm ← rm ; frd ← f64_sqrt(f64(frs1))
fn exec_fsqrt_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Floating Square Root (QP): Calculate the square root of the quadruple-precision value in frs1, then write the result to frd.
//
// > frm ← rm ; frd ← f128_sqrt(f128(frs1))
fn exec_fsqrt_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Square Root (SP): Calculate the square root of the single-precision value in frs1, then write the result to frd.
//
// > frm ← rm ; frd ← f32_sqrt(f32(frs1))
fn exec_fsqrt_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Subtract (DP): Subtract the double-precision values in frs1 from frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f64(frs1) - f64(frs2)
fn exec_fsub_d<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Subtract (QP): Subtract the quadruple-precision values in frs1 from frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f128(frs1) - f128(frs2)
fn exec_fsub_q<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Subtract (SP): Subtract the single-precision values in frs1 from frs2, then write the result to frd.
//
// > frm ← rm ; frd ← f32(frs1) - f32(frs2)
fn exec_fsub_s<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    frd: FloatRegister,
    frs1: FloatRegister,
    frs2: FloatRegister,
    rm: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// FP Store (SP): Stores a single-precision foating-point value from foating-point register frs2 to memory.
//
// > f32[rs1 + imm] ← f32(frs2)
fn exec_fsw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    frs2: FloatRegister,
    simm: i32,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Hypervisor Return: .
//
// >
fn exec_hret<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Jump and Link: Jump to the PC plus 20-bit signed immediate while saving PC+4 into rd.
//
// > rd ← pc + length(inst) ; pc ← pc + imm
fn exec_jal<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rd: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    let ret_pc = inst.pc.wrapping_add(inst.length as u32);
    hart.write_int_register(rd, u32::from_unsigned(ret_pc));
    let new_pc = inst.pc.wrapping_add(u32::from_signed(simm).to_unsigned());
    hart.write_pc(new_pc);
    ExecStatus::Running
}

// Jump and Link Register: Jump to rs1 plus the 12-bit signed immediate while saving PC+4 into rd.
//
// > rd ← pc + length(inst) ; pc ← (rs1 + imm) ∧ -2
fn exec_jalr<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    let base_addr = hart.read_int_register(rs1).to_unsigned();
    let new_pc = base_addr.wrapping_add(u32::from_signed(simm).to_unsigned())
        & 0b11111111111111111111111111111110;
    hart.write_pc(new_pc);

    let ret_pc = inst.pc.wrapping_add(inst.length as u32);
    hart.write_int_register(rd, u32::from_unsigned(ret_pc));

    ExecStatus::Running
}

// Load Byte: Load 8-bit value from addr in rs1 plus the 12-bit signed immediate and place sign-extended result into rd.
//
// > rd ← s8[rs1 + imm]
fn exec_lb<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_load_mem(hart, rd, rs1, simm, |mem, addr| {
        let v = mem.read_byte(addr)?;
        let sv = sign_extend(v as u32, 8);
        Ok(u32::from_signed(sv))
    })
}

// Load Byte Unsigned: Load 8-bit value from addr in rs1 plus the 12-bit signed immediate and place zero-extended result into rd.
//
// > rd ← u8[rs1 + imm]
fn exec_lbu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_load_mem(hart, rd, rs1, simm, |mem, addr| {
        let v = mem.read_byte(addr)?;
        Ok(u32::from_unsigned(v as u32))
    })
}

// Load Half: Load 16-bit value from addr in rs1 plus the 12-bit signed immediate and place sign-extended result into rd.
//
// > rd ← s16[rs1 + imm]
fn exec_lh<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_load_mem(hart, rd, rs1, simm, |mem, addr| {
        let v = mem.read_halfword(addr)?;
        let sv = sign_extend(v as u32, 16);
        Ok(u32::from_signed(sv))
    })
}

// Load Half Unsigned: Load 32-bit value from addr in rs1 plus the 12-bit signed immediate and place zero-extended result into rd.
//
// > rd ← u16[rs1 + imm]
fn exec_lhu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_load_mem(hart, rd, rs1, simm, |mem, addr| {
        let v = mem.read_halfword(addr)?;
        Ok(u32::from_unsigned(v as u32))
    })
}

// Load Reserved Word: Load word from address in rs1, place the sign-extended result in rd and register a reservation on the memory word.
//
// > lr ← rs1 ∥ rd ← sx(s32[rs1])
fn exec_lr_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Load Upper Immediate: Set and sign extend the 20-bit immediate (shited 12 bits left) and zero the bottom 12 bits into rd.
//
// > rd ← imm
fn exec_lui<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    hart.write_int_register(rd, u32::from_signed(simm));
    ExecStatus::Running
}

// Load Word: Load 32-bit value from addr in rs1 plus the 12-bit signed immediate and place sign-extended result into rd.
//
// > rd ← s32[rs1 + imm]
fn exec_lw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_load_mem(hart, rd, rs1, simm, |mem, addr| {
        let v = mem.read_word(addr)?;
        let sv = sign_extend(v as u32, 32);
        Ok(u32::from_signed(sv))
    })
}

// Machine-Mode Return: .
//
// >
fn exec_mret<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Multiply: Multiply rs1 by rs2 and place the result in rd.
//
// > rd ← ux(rs1) × ux(rs2)
fn exec_mul<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_unsigned() as u64;
        let bw = b.to_unsigned() as u64;
        let result = aw * bw;
        u32::from_unsigned(result as u32) // truncate high-order bits
    })
}

// Multiply High Signed Signed: Multiply signed rs1 by signed rs2 and place the high bits of the result in rd.
//
// > rd ← (sx(rs1) × sx(rs2)) » xlen
fn exec_mulh<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_signed() as i64;
        let bw = b.to_signed() as i64;
        let result = (aw * bw) >> 32;
        u32::from_signed(result as i32)
    })
}

// Multiply High Signed Unsigned: Multiply signed rs1 by unsigned rs2 and place the high bits of the result in rd.
//
// > rd ← (sx(rs1) × ux(rs2)) » xlen
fn exec_mulhsu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        // We can't mix signed and unsigned in an operation, but because i64
        // can represent the entire range of both u32 and i32 we can
        // convert both to signed, do signed multiplication, and then
        // capture the high word of the result as normal.
        let aw = a.to_signed() as i64;
        let bw = b.to_unsigned() as i64;
        let result = (aw * bw) >> 32;
        u32::from_signed(result as i32)
    })
}

// Multiply High Unsigned Unsigned: Multiply unsigned rs1 by unsigned rs2 and place the high bits of the result in rd.
//
// > rd ← (ux(rs1) × ux(rs2)) » xlen
fn exec_mulhu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_unsigned() as u64;
        let bw = b.to_unsigned() as u64;
        let result = (aw * bw) >> 32;
        u32::from_unsigned(result as u32)
    })
}

// Or: Set rd to the bitwise or of rs1 and rs2.
//
// > rd ← ux(rs1) ∨ ux(rs2)
fn exec_or<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        u32::from_unsigned(a.to_unsigned() | b.to_unsigned())
    })
}

// Or Immediate: Set rd to the bitwise or of rs1 with the sign-extended 12-bit immediate.
//
// > rd ← ux(rs1) ∨ ux(imm)
fn exec_ori<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_binary_op_imm(hart, rd, rs1, simm, |a, b| {
        u32::from_unsigned(a.to_unsigned() | u32::from_signed(b))
    })
}

// Remainder Signed: Divide rs1 (dividend) by rs2 (divisor) and place the remainder in rd (signed).
//
// > rd ← sx(rs1) mod sx(rs2)
fn exec_rem<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_signed();
        let bw = b.to_signed();
        if bw == 0 {
            // Division by zero produces the first operand
            return u32::from_signed(aw);
        }
        if aw == -2147483648 && bw == -1 {
            // most negative value divided by -1 is an overflow, producing zero
            return u32::from_signed(0);
        }
        let result = aw % bw;
        u32::from_signed(result)
    })
}

// Remainder Unsigned: Divide rs1 (dividend) by rs2 (divisor) and place the remainder in rd (unsigned).
//
// > rd ← ux(rs1) mod ux(rs2)
fn exec_remu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let aw = a.to_unsigned();
        let bw = b.to_unsigned();
        if bw == 0 {
            // Division by zero produces the first operand
            return u32::from_unsigned(aw);
        }
        let result = aw % bw;
        u32::from_unsigned(result)
    })
}

// Store Byte: Store 8-bit value from the low bits of rs2 to addr in rs1 plus the 12-bit signed immediate.
//
// > u8[rs1 + imm] ← rs2
fn exec_sb<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_store_mem(hart, rs1, rs2, simm, |mem, addr, v| {
        mem.write_byte(addr, v.to_unsigned() as u8)
    })
}

// Store Conditional Word: Write word in rs1 to the address in rs2 if a valid reservation exists, write 0 on success or 1 on failure to rd.
//
// > if lr = rs1 then u32[rs1] ← u32(rs2); rd ← 0 else rd ← 1
fn exec_sc_w<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    aq: bool,
    rl: bool,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Supervisor Memory Management Fence: Supervisor memory-management fence synchronizes updates to in-memory memory-management data structures.
//
// >
fn exec_sfence_vm<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// : .
//
// >
fn exec_sfence_vma<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    hart.fence_virtual_memory_config(rs1, rs2);
    ExecStatus::Running
}

// Store Half: Store 16-bit value from the low bits of rs2 to addr in rs1 plus the 12-bit signed immediate.
//
// > u16[rs1 + imm] ← rs2
fn exec_sh<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_store_mem(hart, rs1, rs2, simm, |mem, addr, v| {
        mem.write_halfword(addr, v.to_unsigned() as u16)
    })
}

// Shift Left Logical: Shift rs1 left by the by the lower 5 or 6 (RV32/64) bits in rs2 and place the result into rd.
//
// > rd ← ux(rs1) « rs2
fn exec_sll<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let shamt = b.to_unsigned() & 0b11111;
        u32::from_unsigned(a.to_unsigned() << shamt)
    })
}

// Shift Left Logical Immediate: Shift rs1 left by the 5 or 6 (RV32/64) bit (RV64) immediate and place the result into rd.
//
// > rd ← ux(rs1) « ux(imm)
fn exec_slli<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    shamt: u32,
) -> ExecStatus<u32> {
    exec_shift_op_imm(hart, rd, rs1, shamt, |a, shamt| {
        u32::from_unsigned(a.to_unsigned() << shamt)
    })
}

// Set Less Than: Set rd to 1 if rs1 is less than rs2, otherwise set rd to 0 (signed).
//
// > rd ← sx(rs1) < sx(rs2)
fn exec_slt<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        if a.to_signed() < b.to_signed() {
            1
        } else {
            0
        }
    })
}

// Set Less Than Immediate: Set rd to 1 if rs1 is less than the sign-extended 12-bit immediate, otherwise set rd to 0 (signed).
//
// > rd ← sx(rs1) < sx(imm)
fn exec_slti<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_binary_op_imm(
        hart,
        rd,
        rs1,
        simm,
        |a, b| {
            if a.to_signed() < b {
                1
            } else {
                0
            }
        },
    )
}

// Set Less Than Immediate Unsigned: Set rd to 1 if rs1 is less than the sign-extended 12-bit immediate, otherwise set rd to 0 (unsigned).
//
// > rd ← ux(rs1) < ux(imm)
fn exec_sltiu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_binary_op_imm(hart, rd, rs1, simm, |a, b| {
        if a.to_unsigned() < u32::from_signed(b).to_unsigned() {
            1
        } else {
            0
        }
    })
}

// Set Less Than Unsigned: Set rd to 1 if rs1 is less than rs2, otherwise set rd to 0 (unsigned).
//
// > rd ← ux(rs1) < ux(rs2)
fn exec_sltu<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        if a.to_unsigned() < b.to_unsigned() {
            1
        } else {
            0
        }
    })
}

// Shift Right Arithmetic: Shift rs1 right by the by the lower 5 or 6 (RV32/64) bits in rs2 and place the result into rd while retaining the sign.
//
// > rd ← sx(rs1) » rs2
fn exec_sra<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let shamt = b.to_unsigned() & 0b11111;
        u32::from_signed(a.to_signed() >> shamt)
    })
}

// Shift Right Arithmetic Immediate: Shift rs1 right by the 5 or 6 (RV32/64) bit immediate and place the result into rd while retaining the sign.
//
// > rd ← sx(rs1) » ux(imm)
fn exec_srai<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    shamt: u32,
) -> ExecStatus<u32> {
    exec_shift_op_imm(hart, rd, rs1, shamt, |a, shamt| {
        u32::from_signed(a.to_signed() >> shamt)
    })
}

// System Return: System Return returns to the supervisor mode privilege level after handling a trap.
//
// >
fn exec_sret<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Shift Right Logical: Shift rs1 right by the by the lower 5 or 6 (RV32/64) bits in rs2 and place the result into rd.
//
// > rd ← ux(rs1) » rs2
fn exec_srl<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        let shamt = b.to_unsigned() & 0b11111;
        u32::from_unsigned(a.to_unsigned() >> shamt)
    })
}

// Shift Right Logical Immediate: Shift rs1 right by the 5 or 6 (RV32/64) bit immediate and place the result into rd.
//
// > rd ← ux(rs1) » ux(imm)
fn exec_srli<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    shamt: u32,
) -> ExecStatus<u32> {
    exec_shift_op_imm(hart, rd, rs1, shamt, |a, shamt| {
        u32::from_unsigned(a.to_unsigned() >> shamt)
    })
}

// Subtract: Subtract rs2 from rs1 and place the result into rd.
//
// > rd ← sx(rs1) - sx(rs2)
fn exec_sub<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    let a = hart.read_int_register(rs1).to_signed();
    let b = hart.read_int_register(rs2).to_signed();
    let result = a.wrapping_sub(b);
    hart.write_int_register(rd, u32::from_signed(result));
    ExecStatus::Running
}

// Store Word: Store 32-bit value from the low bits of rs2 to addr in rs1 plus the 12-bit signed immediate.
//
// > u32[rs1 + imm] ← rs2
fn exec_sw<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_store_mem(hart, rs1, rs2, simm, |mem, addr, v| {
        mem.write_word(addr, v.to_unsigned())
    })
}

// User Return: .
//
// >
fn exec_uret<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    // TODO: Implement
    hart.exception(ExceptionCause::IllegalInstruction);
    ExecStatus::Running
}

// Wait For Interrupt: Wait for Interrupt indicates the hart can be stalled until an interrupt needs servicing.
//
// >
fn exec_wfi<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
) -> ExecStatus<u32> {
    ExecStatus::WaitingForInterrupt
}

// Xor: Set rd to the bitwise xor of rs1 and rs2.
//
// > rd ← ux(rs1) ⊻ ux(rs2)
fn exec_xor<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
) -> ExecStatus<u32> {
    exec_binary_op(hart, rd, rs1, rs2, |a, b| {
        u32::from_unsigned(a.to_unsigned() ^ b.to_unsigned())
    })
}

// Xor Immediate: Set rd to the bitwise xor of rs1 with the sign-extended 12-bit immediate.
//
// > rd ← ux(rs1) ⊻ ux(imm)
fn exec_xori<Mem: Bus<u32>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    _inst: Instruction<Op, u32>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
) -> ExecStatus<u32> {
    exec_binary_op_imm(hart, rd, rs1, simm, |a, b| {
        u32::from_unsigned(a.to_unsigned() ^ u32::from_signed(b))
    })
}

fn exec_binary_op<Mem: Bus<u32>, F: FnOnce(u32, u32) -> u32>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    callback: F,
) -> ExecStatus<u32> {
    let a = hart.read_int_register(rs1);
    let b = hart.read_int_register(rs2);
    let result = callback(a, b);
    hart.write_int_register(rd, result);
    ExecStatus::Running
}

fn exec_binary_op_imm<Mem: Bus<u32>, F: FnOnce(u32, i32) -> u32>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    rd: IntRegister,
    rs1: IntRegister,
    imm: i32,
    callback: F,
) -> ExecStatus<u32> {
    let a = hart.read_int_register(rs1);
    let b = imm;
    let result = callback(a, b);
    hart.write_int_register(rd, result);
    ExecStatus::Running
}

fn exec_shift_op_imm<Mem: Bus<u32>, F: FnOnce(u32, u32) -> u32>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    rd: IntRegister,
    rs1: IntRegister,
    shamt: u32,
    callback: F,
) -> ExecStatus<u32> {
    let a = hart.read_int_register(rs1);
    let b = shamt;
    let result = callback(a, b);
    hart.write_int_register(rd, result);
    ExecStatus::Running
}

fn exec_branch_binary_cond<Mem: Bus<u32>, F: FnOnce(u32, u32) -> bool>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    inst: Instruction<Op, u32>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
    callback: F,
) -> ExecStatus<u32> {
    let a = hart.read_int_register(rs1);
    let b = hart.read_int_register(rs2);
    if callback(a, b) {
        let new_pc = inst.pc.wrapping_add(u32::from_signed(simm).to_unsigned());
        hart.write_pc(new_pc);
    }
    ExecStatus::Running
}

fn exec_load_mem<Mem: Bus<u32>, F: FnOnce(&mut Mem, u32) -> Result<u32, MemoryError>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    rd: IntRegister,
    rs1: IntRegister,
    simm: i32,
    callback: F,
) -> ExecStatus<u32> {
    let base_addr = hart.read_int_register(rs1).to_unsigned();
    let addr = base_addr.wrapping_add(u32::from_signed(simm).to_unsigned());
    let result = hart.with_memory(|mem| callback(mem, addr));
    match result {
        Ok(v) => hart.write_int_register(rd, v),
        Err(e) => hart.exception(e.as_data_load_cause()),
    };
    ExecStatus::Running
}

fn exec_store_mem<Mem: Bus<u32>, F: FnOnce(&mut Mem, u32, u32) -> Result<(), MemoryError>>(
    hart: &mut impl Hart<u32, u32, f64, Mem>,
    rs1: IntRegister,
    rs2: IntRegister,
    simm: i32,
    callback: F,
) -> ExecStatus<u32> {
    let v = hart.read_int_register(rs2);
    let base_addr = hart.read_int_register(rs1).to_unsigned();
    let addr = base_addr.wrapping_add(u32::from_signed(simm).to_unsigned());
    let result = hart.with_memory(|mem| callback(mem, addr, v));
    match result {
        Ok(_) => {}
        Err(e) => hart.exception(e.as_data_store_cause()),
    };
    ExecStatus::Running
}
