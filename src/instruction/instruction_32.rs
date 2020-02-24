use crate::raw_instruction::{Opcode, RawInstruction};
use crate::register::{FloatRegister, IntRegister};

/// Enumeration of all operations from the RV32 ISA.
#[derive(Debug, PartialEq)]
pub enum OperationRV32 {
    Invalid,

    // RV32I: Base Integer Instruction Set
    /// Add (RV32I)
    Add {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Add Immediate (RV32I)
    Addi {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// And (RV32I)
    And {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// And Immediate (RV32I)
    Andi {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Add Upper Immediate to PC (RV32I)
    Auipc {
        rd: IntRegister,
        simm: i32,
    },
    /// Branch Equal (RV32I)
    Beq {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Greater than Equal (RV32I)
    Bge {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Greater than Equal Unsigned (RV32I)
    Bgeu {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Less Than (RV32I)
    Blt {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Less Than Unsigned (RV32I)
    Bltu {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Not Equal (RV32I)
    Bne {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Fence (RV32I)
    Fence {
        pred: bool,
        succ: bool,
    },
    /// Fence Instruction (RV32I)
    FenceI,
    /// Jump and Link (RV32I)
    Jal {
        rd: IntRegister,
        simm: i32,
    },
    /// Jump and Link Register (RV32I)
    Jalr {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Byte (RV32I)
    Lb {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Byte Unsigned (RV32I)
    Lbu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Half (RV32I)
    Lh {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Half Unsigned (RV32I)
    Lhu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Upper Immediate (RV32I)
    Lui {
        rd: IntRegister,
        simm: i32,
    },
    /// Load Word (RV32I)
    Lw {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Or (RV32I)
    Or {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Or Immediate (RV32I)
    Ori {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Store Byte (RV32I)
    Sb {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Store Half (RV32I)
    Sh {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Shift Left Logical (RV32I)
    Sll {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Left Logical Immediate (RV32I)
    Slli {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Set Less Than (RV32I)
    Slt {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Set Less Than Immediate (RV32I)
    Slti {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Set Less Than Immediate Unsigned (RV32I)
    Sltiu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Set Less Than Unsigned (RV32I)
    Sltu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Arithmetic (RV32I)
    Sra {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Arithmetic Immediate (RV32I)
    Srai {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Right Logical (RV32I)
    Srl {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Logical Immediate (RV32I)
    Srli {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Subtract (RV32I)
    Sub {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Store Word (RV32I)
    Sw {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Xor (RV32I)
    Xor {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Xor Immediate (RV32I)
    Xori {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },

    // RV32M: Integer Multiply and Divide
    /// Divide Signed (RV32M)
    Div {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Divide Unsigned (RV32M)
    Divu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply (RV32M)
    Mul {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Signed Signed (RV32M)
    Mulh {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Signed Unsigned (RV32M)
    Mulhsu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Unsigned Unsigned (RV32M)
    Mulhu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Signed (RV32M)
    Rem {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Unsigned (RV32M)
    Remu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    // RV32A: Atomic Instructions
    /// Atomic Add Word (RV32A)
    AmoaddW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic And Word (RV32A)
    AmoandW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Word (RV32A)
    AmomaxW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Unsigned Word (RV32A)
    AmomaxuW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Word (RV32A)
    AmominW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Unsigned Word (RV32A)
    AmominuW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Or Word (RV32A)
    AmoorW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Swap Word (RV32A)
    AmoswapW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Xor Word (RV32A)
    AmoxorW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Load Reserved Word (RV32A)
    LrW {
        rd: IntRegister,
        rs1: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Store Conditional Word (RV32A)
    ScW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },

    // RV32S: Supervisor-level Instructions
    /// CSR Atomic Clear Bit (RV32S)
    Csrrc {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Clear Bit Immediate (RV32S)
    Csrrci {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// CSR Atomic Set Bit (RV32S)
    Csrrs {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Set Bit Immediate (RV32S)
    Csrrsi {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// CSR Atomic Read Write (RV32S)
    Csrrw {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Read Write Immediate (RV32S)
    Csrrwi {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// Debug-Mode Return (RV32S)
    Dret,
    /// Environment Break to Debugger (RV32S)
    Ebreak,
    /// Environment Call (RV32S)
    Ecall,
    /// Hypervisor Return (RV32S)
    Hret,
    /// Machine-Mode Return (RV32S)
    Mret,
    /// Supervisor Memory Management Fence (RV32S)
    SfenceVm {
        rs1: IntRegister,
    },
    ///  (RV32S)
    SfenceVma {
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// System Return (RV32S)
    Sret,
    /// User Return (RV32S)
    Uret,
    /// Wait For Interrupt (RV32S)
    Wfi,

    // RV32F: Single-Precision Floating-Point
    /// FP Add (SP) (RV32F)
    FaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (SP) (RV32F)
    FclassS {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert Word to Float (SP) (RV32F)
    FcvtSW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (SP) (RV32F)
    FcvtSWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (SP) (RV32F)
    FcvtWS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (SP) (RV32F)
    FcvtWuS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (SP) (RV32F)
    FdivS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (SP) (RV32F)
    FeqS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than Equal (SP) (RV32F)
    FleS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than (SP) (RV32F)
    FltS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (SP) (RV32F)
    Flw {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Fused Multiply Add (SP) (RV32F)
    FmaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (SP) (RV32F)
    FmaxS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (SP) (RV32F)
    FminS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (SP) (RV32F)
    FmsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (SP) (RV32F)
    FmulS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Move from Integer Register (SP) (RV32F)
    FmvSX {
        frd: FloatRegister,
        rs1: IntRegister,
    },
    /// FP Move to Integer Register (SP) (RV32F)
    FmvXS {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Negate fused Multiply Add (SP) (RV32F)
    FnmaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (SP) (RV32F)
    FnmsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Sign-injection (SP) (RV32F)
    FsgnjS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Sign-injection Negate (SP) (RV32F)
    FsgnjnS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Sign-injection Xor (SP) (RV32F)
    FsgnjxS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Square Root (SP) (RV32F)
    FsqrtS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (SP) (RV32F)
    FsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Store (SP) (RV32F)
    Fsw {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },

    // RV32D: Double-Precision Floating-Point
    /// FP Add (DP) (RV32D)
    FaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (DP) (RV32D)
    FclassD {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert SP to DP (RV32D)
    FcvtDS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Word to Float (DP) (RV32D)
    FcvtDW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (DP) (RV32D)
    FcvtDWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert DP to SP (RV32D)
    FcvtSD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (DP) (RV32D)
    FcvtWD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (DP) (RV32D)
    FcvtWuD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (DP) (RV32D)
    FdivD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (DP) (RV32D)
    FeqD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (DP) (RV32D)
    Fld {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Less Than Equal (DP) (RV32D)
    FleD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than (DP) (RV32D)
    FltD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Add (DP) (RV32D)
    FmaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (DP) (RV32D)
    FmaxD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (DP) (RV32D)
    FminD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (DP) (RV32D)
    FmsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (DP) (RV32D)
    FmulD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Add (DP) (RV32D)
    FnmaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (DP) (RV32D)
    FnmsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Store (DP) (RV32D)
    Fsd {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },
    /// FP to Sign-injection (DP) (RV32D)
    FsgnjD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Negate (DP) (RV32D)
    FsgnjnD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Xor (DP) (RV32D)
    FsgnjxD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// Floating Square Root (DP) (RV32D)
    FsqrtD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (DP) (RV32D)
    FsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },

    // RV32Q: Quadruple-Precision Floating-Point
    /// FP Add (QP) (RV32Q)
    FaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (QP) (RV32Q)
    FclassQ {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert QP to DP (RV32Q)
    FcvtDQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert DP to QP (RV32Q)
    FcvtQD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert SP to QP (RV32Q)
    FcvtQS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Word to Float (QP) (RV32Q)
    FcvtQW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (QP) (RV32Q)
    FcvtQWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert QP to SP (RV32Q)
    FcvtSQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (QP) (RV32Q)
    FcvtWQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (QP) (RV32Q)
    FcvtWuQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (QP) (RV32Q)
    FdivQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (QP) (RV32Q)
    FeqQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than Equal (QP) (RV32Q)
    FleQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (QP) (RV32Q)
    Flq {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Less Than (QP) (RV32Q)
    FltQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Add (QP) (RV32Q)
    FmaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (QP) (RV32Q)
    FmaxQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (QP) (RV32Q)
    FminQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (QP) (RV32Q)
    FmsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (QP) (RV32Q)
    FmulQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Add (QP) (RV32Q)
    FnmaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (QP) (RV32Q)
    FnmsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP to Sign-injection (QP) (RV32Q)
    FsgnjQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Negate (QP) (RV32Q)
    FsgnjnQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Xor (QP) (RV32Q)
    FsgnjxQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Store (QP) (RV32Q)
    Fsq {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },
    /// Floating Square Root (QP) (RV32Q)
    FsqrtQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (QP) (RV32Q)
    FsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },

    // RV32C: Compressed Instructions
    ///  (RV32C)
    CAdd {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CAddi {
        rs1rd: IntRegister,
        nzsimm: i32,
    },
    ///  (RV32C)
    CAddi16Sp {
        rs1rd: IntRegister,
        nzsimm: i32,
    },
    ///  (RV32C)
    CAddi4Spn {
        rd: IntRegister,
        nzuimm: u32,
    },
    ///  (RV32C)
    CAddw {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CAnd {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CAndi {
        rs1rd: IntRegister,
        nzsimm: i32,
    },
    ///  (RV32C)
    CBeqz {
        rs1: IntRegister,
        simm: i32,
    },
    ///  (RV32C)
    CBnez {
        rs1: IntRegister,
        simm: i32,
    },
    ///  (RV32C)
    CEbreak,
    ///  (RV32C)
    CFld {
        frd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFldsp {
        frd: FloatRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFlw {
        frd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFlwsp {
        frd: FloatRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFsd {
        rs1: IntRegister,
        frs2: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFsdsp {
        frs2: FloatRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFsw {
        rs1: IntRegister,
        frs2: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CFswsp {
        frs2: FloatRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CJ {
        simm: i32,
    },
    ///  (RV32C)
    CJal {
        simm: i32,
    },
    ///  (RV32C)
    CJalr {
        rd: IntRegister,
        rs1: IntRegister,
    },
    ///  (RV32C)
    CJr {
        rd: IntRegister,
        rs1: IntRegister,
    },
    ///  (RV32C)
    CLi {
        rs1rd: IntRegister,
        simm: i32,
    },
    ///  (RV32C)
    CLui {
        rd: IntRegister,
        nzsimm: i32,
    },
    ///  (RV32C)
    CLw {
        rd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CLwsp {
        rd: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CMv {
        rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CNop,
    ///  (RV32C)
    COr {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CSlli {
        rs1rd: IntRegister,
        nzuimm: u32,
    },
    ///  (RV32C)
    CSrai {
        rs1rd: IntRegister,
        nzuimm: u32,
    },
    ///  (RV32C)
    CSrli {
        rs1rd: IntRegister,
        nzuimm: u32,
    },
    ///  (RV32C)
    CSub {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CSubw {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV32C)
    CSw {
        rs1: IntRegister,
        rs2: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CSwsp {
        rs2: IntRegister,
        uimm: u32,
    },
    ///  (RV32C)
    CXor {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
}

impl OperationRV32 {
    pub fn decode_from_raw(raw: RawInstruction) -> Self {
        let opcode = raw.opcode();
        if opcode == (Opcode::Amo as u8) {
            if raw.matches(
                0b11111000000000000111000001111111,
                0b00000000000000000010000000101111,
            ) {
                Self::AmoaddW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b01100000000000000010000000101111,
            ) {
                Self::AmoandW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b10100000000000000010000000101111,
            ) {
                Self::AmomaxW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b11100000000000000010000000101111,
            ) {
                Self::AmomaxuW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b10000000000000000010000000101111,
            ) {
                Self::AmominW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b11000000000000000010000000101111,
            ) {
                Self::AmominuW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b01000000000000000010000000101111,
            ) {
                Self::AmoorW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00001000000000000010000000101111,
            ) {
                Self::AmoswapW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00100000000000000010000000101111,
            ) {
                Self::AmoxorW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111001111100000111000001111111,
                0b00010000000000000010000000101111,
            ) {
                Self::LrW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00011000000000000010000000101111,
            ) {
                Self::ScW {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Auipc as u8) {
            if raw.matches(
                0b00000000000000000000000001111111,
                0b00000000000000000000000000010111,
            ) {
                Self::Auipc {
                    rd: raw.rd(),
                    simm: raw.oimm20(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Branch as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000001100011,
            ) {
                Self::Beq {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000101000001100011,
            ) {
                Self::Bge {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000111000001100011,
            ) {
                Self::Bgeu {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000100000001100011,
            ) {
                Self::Blt {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000110000001100011,
            ) {
                Self::Bltu {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000001000001100011,
            ) {
                Self::Bne {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.sbimm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Jal as u8) {
            if raw.matches(
                0b00000000000000000000000001111111,
                0b00000000000000000000000001101111,
            ) {
                Self::Jal {
                    rd: raw.rd(),
                    simm: raw.jimm20(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Jalr as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000001100111,
            ) {
                Self::Jalr {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Load as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000000000011,
            ) {
                Self::Lb {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000100000000000011,
            ) {
                Self::Lbu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000001000000000011,
            ) {
                Self::Lh {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000101000000000011,
            ) {
                Self::Lhu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000000000011,
            ) {
                Self::Lw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::LoadFp as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000011000000000111,
            ) {
                Self::Fld {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000100000000000111,
            ) {
                Self::Flq {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000000000111,
            ) {
                Self::Flw {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    simm: raw.oimm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Lui as u8) {
            if raw.matches(
                0b00000000000000000000000001111111,
                0b00000000000000000000000000110111,
            ) {
                Self::Lui {
                    rd: raw.rd(),
                    simm: raw.imm20(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Madd as u8) {
            if raw.matches(
                0b00000110000000000000000001111111,
                0b00000010000000000000000001000011,
            ) {
                Self::FmaddD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000110000000000000000001000011,
            ) {
                Self::FmaddQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000000000000000000000001000011,
            ) {
                Self::FmaddS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::MiscMem as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000000001111,
            ) {
                Self::Fence {
                    pred: raw.pred(),
                    succ: raw.succ(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000001000000001111,
            ) {
                Self::FenceI
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Msub as u8) {
            if raw.matches(
                0b00000110000000000000000001111111,
                0b00000010000000000000000001000111,
            ) {
                Self::FmsubD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000110000000000000000001000111,
            ) {
                Self::FmsubQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000000000000000000000001000111,
            ) {
                Self::FmsubS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Nmadd as u8) {
            if raw.matches(
                0b00000110000000000000000001111111,
                0b00000010000000000000000001001111,
            ) {
                Self::FnmaddD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000110000000000000000001001111,
            ) {
                Self::FnmaddQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000000000000000000000001001111,
            ) {
                Self::FnmaddS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Nmsub as u8) {
            if raw.matches(
                0b00000110000000000000000001111111,
                0b00000010000000000000000001001011,
            ) {
                Self::FnmsubD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000110000000000000000001001011,
            ) {
                Self::FnmsubQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b00000110000000000000000001111111,
                0b00000000000000000000000001001011,
            ) {
                Self::FnmsubS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    frs3: raw.frs3(),
                    rm: raw.rm(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Op as u8) {
            if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000000000000110011,
            ) {
                Self::Add {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000111000000110011,
            ) {
                Self::And {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000100000000110011,
            ) {
                Self::Div {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000101000000110011,
            ) {
                Self::Divu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000000000000110011,
            ) {
                Self::Mul {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000001000000110011,
            ) {
                Self::Mulh {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000010000000110011,
            ) {
                Self::Mulhsu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000011000000110011,
            ) {
                Self::Mulhu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000110000000110011,
            ) {
                Self::Or {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000110000000110011,
            ) {
                Self::Rem {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000111000000110011,
            ) {
                Self::Remu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000001000000110011,
            ) {
                Self::Sll {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000010000000110011,
            ) {
                Self::Slt {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000011000000110011,
            ) {
                Self::Sltu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b01000000000000000101000000110011,
            ) {
                Self::Sra {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000101000000110011,
            ) {
                Self::Srl {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b01000000000000000000000000110011,
            ) {
                Self::Sub {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000100000000110011,
            ) {
                Self::Xor {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::Op32 as u8) {
            Self::Invalid
        } else if opcode == (Opcode::OpFp as u8) {
            if raw.matches(
                0b11111110000000000000000001111111,
                0b00000010000000000000000001010011,
            ) {
                Self::FaddD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00000110000000000000000001010011,
            ) {
                Self::FaddQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00000000000000000000000001010011,
            ) {
                Self::FaddS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11100010000000000001000001010011,
            ) {
                Self::FclassD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11100110000000000001000001010011,
            ) {
                Self::FclassQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11100000000000000001000001010011,
            ) {
                Self::FclassS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000010001100000000000001010011,
            ) {
                Self::FcvtDQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000010000000000000000001010011,
            ) {
                Self::FcvtDS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010010000000000000000001010011,
            ) {
                Self::FcvtDW {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010010000100000000000001010011,
            ) {
                Self::FcvtDWu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000110000100000000000001010011,
            ) {
                Self::FcvtQD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000110000000000000000001010011,
            ) {
                Self::FcvtQS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010110000000000000000001010011,
            ) {
                Self::FcvtQW {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010110000100000000000001010011,
            ) {
                Self::FcvtQWu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000000000100000000000001010011,
            ) {
                Self::FcvtSD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01000000001100000000000001010011,
            ) {
                Self::FcvtSQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010000000000000000000001010011,
            ) {
                Self::FcvtSW {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010000000100000000000001010011,
            ) {
                Self::FcvtSWu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000010000000000000000001010011,
            ) {
                Self::FcvtWD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000110000000000000000001010011,
            ) {
                Self::FcvtWQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000000000000000000000001010011,
            ) {
                Self::FcvtWS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000010000100000000000001010011,
            ) {
                Self::FcvtWuD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000110000100000000000001010011,
            ) {
                Self::FcvtWuQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000000000100000000000001010011,
            ) {
                Self::FcvtWuS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00011010000000000000000001010011,
            ) {
                Self::FdivD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00011110000000000000000001010011,
            ) {
                Self::FdivQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00011000000000000000000001010011,
            ) {
                Self::FdivS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100010000000000010000001010011,
            ) {
                Self::FeqD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100110000000000010000001010011,
            ) {
                Self::FeqQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100000000000000010000001010011,
            ) {
                Self::FeqS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100010000000000000000001010011,
            ) {
                Self::FleD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100110000000000000000001010011,
            ) {
                Self::FleQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100000000000000000000001010011,
            ) {
                Self::FleS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100010000000000001000001010011,
            ) {
                Self::FltD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100110000000000001000001010011,
            ) {
                Self::FltQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b10100000000000000001000001010011,
            ) {
                Self::FltS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101010000000000001000001010011,
            ) {
                Self::FmaxD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101110000000000001000001010011,
            ) {
                Self::FmaxQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101000000000000001000001010011,
            ) {
                Self::FmaxS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101010000000000000000001010011,
            ) {
                Self::FminD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101110000000000000000001010011,
            ) {
                Self::FminQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00101000000000000000000001010011,
            ) {
                Self::FminS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00010010000000000000000001010011,
            ) {
                Self::FmulD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00010110000000000000000001010011,
            ) {
                Self::FmulQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00010000000000000000000001010011,
            ) {
                Self::FmulS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11110000000000000000000001010011,
            ) {
                Self::FmvSX {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11100000000000000000000001010011,
            ) {
                Self::FmvXS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100010000000000000000001010011,
            ) {
                Self::FsgnjD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100110000000000000000001010011,
            ) {
                Self::FsgnjQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100000000000000000000001010011,
            ) {
                Self::FsgnjS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100010000000000001000001010011,
            ) {
                Self::FsgnjnD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100110000000000001000001010011,
            ) {
                Self::FsgnjnQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100000000000000001000001010011,
            ) {
                Self::FsgnjnS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100010000000000010000001010011,
            ) {
                Self::FsgnjxD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100110000000000010000001010011,
            ) {
                Self::FsgnjxQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00100000000000000010000001010011,
            ) {
                Self::FsgnjxS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01011010000000000000000001010011,
            ) {
                Self::FsqrtD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01011110000000000000000001010011,
            ) {
                Self::FsqrtQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b01011000000000000000000001010011,
            ) {
                Self::FsqrtS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00001010000000000000000001010011,
            ) {
                Self::FsubD {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00001110000000000000000001010011,
            ) {
                Self::FsubQ {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111110000000000000000001111111,
                0b00001000000000000000000001010011,
            ) {
                Self::FsubS {
                    frd: raw.frd(),
                    frs1: raw.frs1(),
                    frs2: raw.frs2(),
                    rm: raw.rm(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::OpImm as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000000010011,
            ) {
                Self::Addi {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000111000000010011,
            ) {
                Self::Andi {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000110000000010011,
            ) {
                Self::Ori {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00000000000000000001000000010011,
            ) {
                Self::Slli {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000000010011,
            ) {
                Self::Slti {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000011000000010011,
            ) {
                Self::Sltiu {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b01000000000000000101000000010011,
            ) {
                Self::Srai {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00000000000000000101000000010011,
            ) {
                Self::Srli {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000100000000010011,
            ) {
                Self::Xori {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::OpImm32 as u8) {
            Self::Invalid
        } else if opcode == (Opcode::Store as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000000100011,
            ) {
                Self::Sb {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.simm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000001000000100011,
            ) {
                Self::Sh {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.simm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000000100011,
            ) {
                Self::Sw {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    simm: raw.simm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::StoreFp as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000011000000100111,
            ) {
                Self::Fsd {
                    rs1: raw.rs1(),
                    frs2: raw.frs2(),
                    simm: raw.simm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000100000000100111,
            ) {
                Self::Fsq {
                    rs1: raw.rs1(),
                    frs2: raw.frs2(),
                    simm: raw.simm12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000000100111,
            ) {
                Self::Fsw {
                    rs1: raw.rs1(),
                    frs2: raw.frs2(),
                    simm: raw.simm12(),
                }
            } else {
                Self::Invalid
            }
        } else if opcode == (Opcode::System as u8) {
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000011000001110011,
            ) {
                Self::Csrrc {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000111000001110011,
            ) {
                Self::Csrrci {
                    rd: raw.rd(),
                    uimm: raw.zimm(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000010000001110011,
            ) {
                Self::Csrrs {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000110000001110011,
            ) {
                Self::Csrrsi {
                    rd: raw.rd(),
                    uimm: raw.zimm(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000001000001110011,
            ) {
                Self::Csrrw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000101000001110011,
            ) {
                Self::Csrrwi {
                    rd: raw.rd(),
                    uimm: raw.zimm(),
                    csr: raw.csr12(),
                }
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b01111011001000000000000001110011,
            ) {
                Self::Dret
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00000000000100000000000001110011,
            ) {
                Self::Ebreak
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00000000000000000000000001110011,
            ) {
                Self::Ecall
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00100000001000000000000001110011,
            ) {
                Self::Hret
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00110000001000000000000001110011,
            ) {
                Self::Mret
            } else if raw.matches(
                0b11111111111100000111111111111111,
                0b00010000010000000000000001110011,
            ) {
                Self::SfenceVm { rs1: raw.rs1() }
            } else if raw.matches(
                0b11111110000000000111111111111111,
                0b00010010000000000000000001110011,
            ) {
                Self::SfenceVma {
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00010000001000000000000001110011,
            ) {
                Self::Sret
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00000000001000000000000001110011,
            ) {
                Self::Uret
            } else if raw.matches(
                0b11111111111111111111111111111111,
                0b00010000010100000000000001110011,
            ) {
                Self::Wfi
            } else {
                Self::Invalid
            }
        } else {
            if raw.matches(0b1110000000000011, 0b1000000000000010) {
                Self::CAdd {
                    rs1rd: raw.crs1rd(),
                    rs2: raw.crs2(),
                }
            } else if raw.matches(0b1110000000000011, 0b0000000000000001) {
                Self::CAddi {
                    rs1rd: raw.crs1rd(),
                    nzsimm: raw.cnzimmi(),
                }
            } else if raw.matches(0b1110111110000011, 0b0110000100000001) {
                Self::CAddi16Sp {
                    rs1rd: raw.crs1rd(),
                    nzsimm: raw.cimm16sp(),
                }
            } else if raw.matches(0b1110000000000011, 0b0000000000000000) {
                Self::CAddi4Spn {
                    rd: raw.crdq(),
                    nzuimm: raw.cimm4spn(),
                }
            } else if raw.matches(0b1110110001100011, 0b1000110000100001) {
                Self::CAddw {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else if raw.matches(0b1110110001100011, 0b1000110001100001) {
                Self::CAnd {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else if raw.matches(0b1110110000000011, 0b1000100000000001) {
                Self::CAndi {
                    rs1rd: raw.crs1rdq(),
                    nzsimm: raw.cnzimmi(),
                }
            } else if raw.matches(0b1110000000000011, 0b1100000000000001) {
                Self::CBeqz {
                    rs1: raw.crs1q(),
                    simm: raw.cimmb(),
                }
            } else if raw.matches(0b1110000000000011, 0b1110000000000001) {
                Self::CBnez {
                    rs1: raw.crs1q(),
                    simm: raw.cimmb(),
                }
            } else if raw.matches(0b1110111111111111, 0b1000000000000010) {
                Self::CEbreak
            } else if raw.matches(0b1110000000000011, 0b0010000000000000) {
                Self::CFld {
                    frd: raw.cfrdq(),
                    rs1: raw.crs1q(),
                    uimm: raw.cimmd(),
                }
            } else if raw.matches(0b1110000000000011, 0b0010000000000010) {
                Self::CFldsp {
                    frd: raw.cfrd(),
                    uimm: raw.cimmldsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b0110000000000000) {
                Self::CFlw {
                    frd: raw.cfrdq(),
                    rs1: raw.crs1q(),
                    uimm: raw.cimmw(),
                }
            } else if raw.matches(0b1110000000000011, 0b0110000000000010) {
                Self::CFlwsp {
                    frd: raw.cfrd(),
                    uimm: raw.cimmlwsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b1010000000000000) {
                Self::CFsd {
                    rs1: raw.crs1q(),
                    frs2: raw.cfrs2q(),
                    uimm: raw.cimmd(),
                }
            } else if raw.matches(0b1110000000000011, 0b1010000000000010) {
                Self::CFsdsp {
                    frs2: raw.cfrs2(),
                    uimm: raw.cimmsdsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b1110000000000000) {
                Self::CFsw {
                    rs1: raw.crs1q(),
                    frs2: raw.cfrs2q(),
                    uimm: raw.cimmw(),
                }
            } else if raw.matches(0b1110000000000011, 0b1110000000000010) {
                Self::CFswsp {
                    frs2: raw.cfrs2(),
                    uimm: raw.cimmswsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b1010000000000001) {
                Self::CJ { simm: raw.cimmj() }
            } else if raw.matches(0b1110000000000011, 0b0010000000000001) {
                Self::CJal { simm: raw.cimmj() }
            } else if raw.matches(0b1110000001111111, 0b1000000000000010) {
                Self::CJalr {
                    rd: raw.crd0(),
                    rs1: raw.crs1(),
                }
            } else if raw.matches(0b1110000001111111, 0b1000000000000010) {
                Self::CJr {
                    rd: raw.crd0(),
                    rs1: raw.crs1(),
                }
            } else if raw.matches(0b1110000000000011, 0b0100000000000001) {
                Self::CLi {
                    rs1rd: raw.crs1rd(),
                    simm: raw.cimmi(),
                }
            } else if raw.matches(0b1110000000000011, 0b0110000000000001) {
                Self::CLui {
                    rd: raw.crd(),
                    nzsimm: raw.cimmui(),
                }
            } else if raw.matches(0b1110000000000011, 0b0100000000000000) {
                Self::CLw {
                    rd: raw.crdq(),
                    rs1: raw.crs1q(),
                    uimm: raw.cimmw(),
                }
            } else if raw.matches(0b1110000000000011, 0b0100000000000010) {
                Self::CLwsp {
                    rd: raw.crd(),
                    uimm: raw.cimmlwsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b1000000000000010) {
                Self::CMv {
                    rd: raw.crd(),
                    rs2: raw.crs2(),
                }
            } else if raw.matches(0b1110111111111111, 0b0000000000000001) {
                Self::CNop
            } else if raw.matches(0b1110110001100011, 0b1000110001000001) {
                Self::COr {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else if raw.matches(0b1110000000000011, 0b0000000000000010) {
                Self::CSlli {
                    rs1rd: raw.crs1rd(),
                    nzuimm: raw.cimmsh5(),
                }
            } else if raw.matches(0b1110110000000011, 0b1000010000000001) {
                Self::CSrai {
                    rs1rd: raw.crs1rdq(),
                    nzuimm: raw.cimmsh5(),
                }
            } else if raw.matches(0b1110110000000011, 0b1000000000000001) {
                Self::CSrli {
                    rs1rd: raw.crs1rdq(),
                    nzuimm: raw.cimmsh5(),
                }
            } else if raw.matches(0b1110110001100011, 0b1000110000000001) {
                Self::CSub {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else if raw.matches(0b1110110001100011, 0b1000110000000001) {
                Self::CSubw {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else if raw.matches(0b1110000000000011, 0b1100000000000000) {
                Self::CSw {
                    rs1: raw.crs1q(),
                    rs2: raw.crs2q(),
                    uimm: raw.cimmw(),
                }
            } else if raw.matches(0b1110000000000011, 0b1100000000000010) {
                Self::CSwsp {
                    rs2: raw.crs2(),
                    uimm: raw.cimmswsp(),
                }
            } else if raw.matches(0b1110110001100011, 0b1000110000100001) {
                Self::CXor {
                    rs1rd: raw.crs1rdq(),
                    rs2: raw.crs2q(),
                }
            } else {
                Self::Invalid
            }
        }
    }
}
