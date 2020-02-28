use crate::raw_instruction::{Opcode, RawInstruction};
use crate::register::{FloatRegister, IntRegister};

/// Enumeration of all operations from the RV64 ISA.
#[derive(Debug, PartialEq)]
pub enum OperationRV64 {
    /// The result of decoding an instruction that isn't valid at all,
    /// according to the current decoder implementation.
    Invalid,

    // RV64I: Base Integer Instruction Set
    /// Add (RV64I)
    Add {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Add Immediate (RV64I)
    Addi {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Add Immediate Word (RV64I)
    Addiw {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Add Word (RV64I)
    Addw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// And (RV64I)
    And {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// And Immediate (RV64I)
    Andi {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Add Upper Immediate to PC (RV64I)
    Auipc { rd: IntRegister, simm: i32 },
    /// Branch Equal (RV64I)
    Beq {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Greater than Equal (RV64I)
    Bge {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Greater than Equal Unsigned (RV64I)
    Bgeu {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Less Than (RV64I)
    Blt {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Less Than Unsigned (RV64I)
    Bltu {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Branch Not Equal (RV64I)
    Bne {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Fence (RV64I)
    Fence { pred: bool, succ: bool },
    /// Fence Instruction (RV64I)
    FenceI,
    /// Jump and Link (RV64I)
    Jal { rd: IntRegister, simm: i32 },
    /// Jump and Link Register (RV64I)
    Jalr {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Byte (RV64I)
    Lb {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Byte Unsigned (RV64I)
    Lbu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Double (RV64I)
    Ld {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Half (RV64I)
    Lh {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Half Unsigned (RV64I)
    Lhu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Upper Immediate (RV64I)
    Lui { rd: IntRegister, simm: i32 },
    /// Load Word (RV64I)
    Lw {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Load Word Unsigned (RV64I)
    Lwu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Or (RV64I)
    Or {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Or Immediate (RV64I)
    Ori {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Store Byte (RV64I)
    Sb {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Store Double (RV64I)
    Sd {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Store Half (RV64I)
    Sh {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Shift Left Logical (RV64I)
    Sll {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Left Logical Immediate (RV64I)
    Slli {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Left Logical Immediate Word (RV64I)
    Slliw {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Left Logical Word (RV64I)
    Sllw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Set Less Than (RV64I)
    Slt {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Set Less Than Immediate (RV64I)
    Slti {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Set Less Than Immediate Unsigned (RV64I)
    Sltiu {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// Set Less Than Unsigned (RV64I)
    Sltu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Arithmetic (RV64I)
    Sra {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Arithmetic Immediate (RV64I)
    Srai {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Right Arithmetic Immediate Word (RV64I)
    Sraiw {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Right Arithmetic Word (RV64I)
    Sraw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Logical (RV64I)
    Srl {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Shift Right Logical Immediate (RV64I)
    Srli {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Right Logical Immediate Word (RV64I)
    Srliw {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: u32,
    },
    /// Shift Right Logical Word (RV64I)
    Srlw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Subtract (RV64I)
    Sub {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Subtract Word (RV64I)
    Subw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Store Word (RV64I)
    Sw {
        rs1: IntRegister,
        rs2: IntRegister,
        simm: i32,
    },
    /// Xor (RV64I)
    Xor {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Xor Immediate (RV64I)
    Xori {
        rd: IntRegister,
        rs1: IntRegister,
        simm: i32,
    },

    // RV64M: Integer Multiply and Divide
    /// Divide Signed (RV64M)
    Div {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Divide Unsigned (RV64M)
    Divu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Divide Unsigned Word (RV64M)
    Divuw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Divide Signed Word (RV64M)
    Divw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply (RV64M)
    Mul {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Signed Signed (RV64M)
    Mulh {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Signed Unsigned (RV64M)
    Mulhsu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiply High Unsigned Unsigned (RV64M)
    Mulhu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Multiple Word (RV64M)
    Mulw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Signed (RV64M)
    Rem {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Unsigned (RV64M)
    Remu {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Unsigned Word (RV64M)
    Remuw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    /// Remainder Signed Word (RV64M)
    Remw {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    // RV64A: Atomic Instructions
    /// Atomic Add Double Word (RV64A)
    AmoaddD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Add Word (RV64A)
    AmoaddW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic And Double Word (RV64A)
    AmoandD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic And Word (RV64A)
    AmoandW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Double Word (RV64A)
    AmomaxD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Word (RV64A)
    AmomaxW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Unsigned Double Word (RV64A)
    AmomaxuD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Maximum Unsigned Word (RV64A)
    AmomaxuW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Double Word (RV64A)
    AmominD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Word (RV64A)
    AmominW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Unsigned Double Word (RV64A)
    AmominuD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Minimum Unsigned Word (RV64A)
    AmominuW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Or Double Word (RV64A)
    AmoorD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Or Word (RV64A)
    AmoorW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Swap Double Word (RV64A)
    AmoswapD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Swap Word (RV64A)
    AmoswapW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Xor Double Word (RV64A)
    AmoxorD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Atomic Xor Word (RV64A)
    AmoxorW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Load Reserved Double Word (RV64A)
    LrD {
        rd: IntRegister,
        rs1: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Load Reserved Word (RV64A)
    LrW {
        rd: IntRegister,
        rs1: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Store Conditional Double Word (RV64A)
    ScD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },
    /// Store Conditional Word (RV64A)
    ScW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        aq: bool,
        rl: bool,
    },

    // RV64S: Supervisor-level Instructions
    /// CSR Atomic Clear Bit (RV64S)
    Csrrc {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Clear Bit Immediate (RV64S)
    Csrrci {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// CSR Atomic Set Bit (RV64S)
    Csrrs {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Set Bit Immediate (RV64S)
    Csrrsi {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// CSR Atomic Read Write (RV64S)
    Csrrw {
        rd: IntRegister,
        rs1: IntRegister,
        csr: u32,
    },
    /// CSR Atomic Read Write Immediate (RV64S)
    Csrrwi {
        rd: IntRegister,
        uimm: u32,
        csr: u32,
    },
    /// Debug-Mode Return (RV64S)
    Dret,
    /// Environment Break to Debugger (RV64S)
    Ebreak,
    /// Environment Call (RV64S)
    Ecall,
    /// Hypervisor Return (RV64S)
    Hret,
    /// Machine-Mode Return (RV64S)
    Mret,
    /// Supervisor Memory Management Fence (RV64S)
    SfenceVm { rs1: IntRegister },
    ///  (RV64S)
    SfenceVma { rs1: IntRegister, rs2: IntRegister },
    /// System Return (RV64S)
    Sret,
    /// User Return (RV64S)
    Uret,
    /// Wait For Interrupt (RV64S)
    Wfi,

    // RV64F: Single-Precision Floating-Point
    /// FP Add (SP) (RV64F)
    FaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (SP) (RV64F)
    FclassS {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert Float to Double Word (SP) (RV64F)
    FcvtLS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Double Word Unsigned (SP) (RV64F)
    FcvtLuS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Double Word to Float (SP) (RV64F)
    FcvtSL {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Double Word Unsigned to Float (SP) (RV64F)
    FcvtSLu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word to Float (SP) (RV64F)
    FcvtSW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (SP) (RV64F)
    FcvtSWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (SP) (RV64F)
    FcvtWS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (SP) (RV64F)
    FcvtWuS {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (SP) (RV64F)
    FdivS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (SP) (RV64F)
    FeqS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than Equal (SP) (RV64F)
    FleS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than (SP) (RV64F)
    FltS {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (SP) (RV64F)
    Flw {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Fused Multiply Add (SP) (RV64F)
    FmaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (SP) (RV64F)
    FmaxS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (SP) (RV64F)
    FminS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (SP) (RV64F)
    FmsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (SP) (RV64F)
    FmulS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Move from Integer Register (SP) (RV64F)
    FmvSX {
        frd: FloatRegister,
        rs1: IntRegister,
    },
    /// FP Move to Integer Register (SP) (RV64F)
    FmvXS {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Negate fused Multiply Add (SP) (RV64F)
    FnmaddS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (SP) (RV64F)
    FnmsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Sign-injection (SP) (RV64F)
    FsgnjS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Sign-injection Negate (SP) (RV64F)
    FsgnjnS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Sign-injection Xor (SP) (RV64F)
    FsgnjxS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Square Root (SP) (RV64F)
    FsqrtS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (SP) (RV64F)
    FsubS {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Store (SP) (RV64F)
    Fsw {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },

    // RV64D: Double-Precision Floating-Point
    /// FP Add (DP) (RV64D)
    FaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (DP) (RV64D)
    FclassD {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert Double Word to Float (DP) (RV64D)
    FcvtDL {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Double Word Unsigned Float (DP) (RV64D)
    FcvtDLu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert SP to DP (RV64D)
    FcvtDS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Word to Float (DP) (RV64D)
    FcvtDW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (DP) (RV64D)
    FcvtDWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Float to Double Word (DP) (RV64D)
    FcvtLD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Double Word Unsigned (DP) (RV64D)
    FcvtLuD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert DP to SP (RV64D)
    FcvtSD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (DP) (RV64D)
    FcvtWD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (DP) (RV64D)
    FcvtWuD {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (DP) (RV64D)
    FdivD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (DP) (RV64D)
    FeqD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (DP) (RV64D)
    Fld {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Less Than Equal (DP) (RV64D)
    FleD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than (DP) (RV64D)
    FltD {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Add (DP) (RV64D)
    FmaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (DP) (RV64D)
    FmaxD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (DP) (RV64D)
    FminD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (DP) (RV64D)
    FmsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (DP) (RV64D)
    FmulD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Move from Integer Register (DP) (RV64D)
    FmvDX {
        frd: FloatRegister,
        rs1: IntRegister,
    },
    /// FP Move to Integer Register (DP) (RV64D)
    FmvXD {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Negate fused Multiply Add (DP) (RV64D)
    FnmaddD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (DP) (RV64D)
    FnmsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Store (DP) (RV64D)
    Fsd {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },
    /// FP to Sign-injection (DP) (RV64D)
    FsgnjD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Negate (DP) (RV64D)
    FsgnjnD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Xor (DP) (RV64D)
    FsgnjxD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// Floating Square Root (DP) (RV64D)
    FsqrtD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (DP) (RV64D)
    FsubD {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },

    // RV64Q: Quadruple-Precision Floating-Point
    /// FP Add (QP) (RV64Q)
    FaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Classify (QP) (RV64Q)
    FclassQ {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Convert QP to DP (RV64Q)
    FcvtDQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Double Word (QP) (RV64Q)
    FcvtLQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Double Word Unsigned (QP) (RV64Q)
    FcvtLuQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert DP to QP (RV64Q)
    FcvtQD {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Double Word to Float (QP) (RV64Q)
    FcvtQL {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Double Word Unsigned Float (QP) (RV64Q)
    FcvtQLu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert SP to QP (RV64Q)
    FcvtQS {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Word to Float (QP) (RV64Q)
    FcvtQW {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert Word Unsigned to Float (QP) (RV64Q)
    FcvtQWu {
        frd: FloatRegister,
        rs1: IntRegister,
        rm: bool,
    },
    /// FP Convert QP to SP (RV64Q)
    FcvtSQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word (QP) (RV64Q)
    FcvtWQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Convert Float to Word Unsigned (QP) (RV64Q)
    FcvtWuQ {
        rd: IntRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Divide (QP) (RV64Q)
    FdivQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Equal (QP) (RV64Q)
    FeqQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Less Than Equal (QP) (RV64Q)
    FleQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Load (QP) (RV64Q)
    Flq {
        frd: FloatRegister,
        rs1: IntRegister,
        simm: i32,
    },
    /// FP Less Than (QP) (RV64Q)
    FltQ {
        rd: IntRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Add (QP) (RV64Q)
    FmaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Maximum (QP) (RV64Q)
    FmaxQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Minimum (QP) (RV64Q)
    FminQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Fused Multiply Subtract (QP) (RV64Q)
    FmsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Multiply (QP) (RV64Q)
    FmulQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },
    /// FP Move from Integer Register (QP) (RV64Q)
    FmvQX {
        frd: FloatRegister,
        rs1: IntRegister,
    },
    /// FP Move to Integer Register (QP) (RV64Q)
    FmvXQ {
        rd: IntRegister,
        frs1: FloatRegister,
    },
    /// FP Negate fused Multiply Add (QP) (RV64Q)
    FnmaddQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP Negate fused Multiply Subtract (QP) (RV64Q)
    FnmsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        frs3: FloatRegister,
        rm: bool,
    },
    /// FP to Sign-injection (QP) (RV64Q)
    FsgnjQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Negate (QP) (RV64Q)
    FsgnjnQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP to Sign-injection Xor (QP) (RV64Q)
    FsgnjxQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
    },
    /// FP Store (QP) (RV64Q)
    Fsq {
        rs1: IntRegister,
        frs2: FloatRegister,
        simm: i32,
    },
    /// Floating Square Root (QP) (RV64Q)
    FsqrtQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        rm: bool,
    },
    /// FP Subtract (QP) (RV64Q)
    FsubQ {
        frd: FloatRegister,
        frs1: FloatRegister,
        frs2: FloatRegister,
        rm: bool,
    },

    // RV64C: Compressed Instructions
    ///  (RV64C)
    CAdd {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CAddi { rs1rd: IntRegister, nzsimm: i32 },
    ///  (RV64C)
    CAddi16Sp { rs1rd: IntRegister, nzsimm: i32 },
    ///  (RV64C)
    CAddi4Spn { rd: IntRegister, nzuimm: u32 },
    ///  (RV64C)
    CAddiw { rs1rd: IntRegister, nzsimm: i32 },
    ///  (RV64C)
    CAddw {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CAnd {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CAndi { rs1rd: IntRegister, nzsimm: i32 },
    ///  (RV64C)
    CBeqz { rs1: IntRegister, simm: i32 },
    ///  (RV64C)
    CBnez { rs1: IntRegister, simm: i32 },
    ///  (RV64C)
    CEbreak,
    ///  (RV64C)
    CFld {
        frd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CFldsp { frd: FloatRegister, uimm: u32 },
    ///  (RV64C)
    CFsd {
        rs1: IntRegister,
        frs2: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CFsdsp { frs2: FloatRegister, uimm: u32 },
    ///  (RV64C)
    CJ { simm: i32 },
    ///  (RV64C)
    CJalr { rd: IntRegister, rs1: IntRegister },
    ///  (RV64C)
    CJr { rd: IntRegister, rs1: IntRegister },
    ///  (RV64C)
    CLd {
        rd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CLdsp { rd: IntRegister, uimm: u32 },
    ///  (RV64C)
    CLi { rs1rd: IntRegister, simm: i32 },
    ///  (RV64C)
    CLui { rd: IntRegister, nzsimm: i32 },
    ///  (RV64C)
    CLw {
        rd: IntRegister,
        rs1: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CLwsp { rd: IntRegister, uimm: u32 },
    ///  (RV64C)
    CMv { rd: IntRegister, rs2: IntRegister },
    ///  (RV64C)
    CNop,
    ///  (RV64C)
    COr {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CSd {
        rs1: IntRegister,
        rs2: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CSdsp { rs2: IntRegister, uimm: u32 },
    ///  (RV64C)
    CSlli { rs1rd: IntRegister, nzuimm: u32 },
    ///  (RV64C)
    CSrai { rs1rd: IntRegister, nzuimm: u32 },
    ///  (RV64C)
    CSrli { rs1rd: IntRegister, nzuimm: u32 },
    ///  (RV64C)
    CSub {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CSubw {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
    ///  (RV64C)
    CSw {
        rs1: IntRegister,
        rs2: IntRegister,
        uimm: u32,
    },
    ///  (RV64C)
    CSwsp { rs2: IntRegister, uimm: u32 },
    ///  (RV64C)
    CXor {
        rs1rd: IntRegister,
        rs2: IntRegister,
    },
}

impl OperationRV64 {
    pub fn decode_from_raw(raw: RawInstruction) -> Self {
        let opcode = raw.opcode();
        if opcode == (Opcode::Amo as u8) {
            if raw.matches(
                0b11111000000000000111000001111111,
                0b00000000000000000011000000101111,
            ) {
                Self::AmoaddD {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                    aq: raw.aq(),
                    rl: raw.rl(),
                }
            } else if raw.matches(
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
                0b01100000000000000011000000101111,
            ) {
                Self::AmoandD {
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
                0b10100000000000000011000000101111,
            ) {
                Self::AmomaxD {
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
                0b11100000000000000011000000101111,
            ) {
                Self::AmomaxuD {
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
                0b10000000000000000011000000101111,
            ) {
                Self::AmominD {
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
                0b11000000000000000011000000101111,
            ) {
                Self::AmominuD {
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
                0b01000000000000000011000000101111,
            ) {
                Self::AmoorD {
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
                0b00001000000000000011000000101111,
            ) {
                Self::AmoswapD {
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
                0b00100000000000000011000000101111,
            ) {
                Self::AmoxorD {
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
                0b00010000000000000011000000101111,
            ) {
                Self::LrD {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
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
                0b00011000000000000011000000101111,
            ) {
                Self::ScD {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
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
                0b00000000000000000011000000000011,
            ) {
                Self::Ld {
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
            } else if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000110000000000011,
            ) {
                Self::Lwu {
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
            if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000000000000111011,
            ) {
                Self::Addw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000101000000111011,
            ) {
                Self::Divuw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000100000000111011,
            ) {
                Self::Divw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000000000000111011,
            ) {
                Self::Mulw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000111000000111011,
            ) {
                Self::Remuw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000010000000000110000000111011,
            ) {
                Self::Remw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000001000000111011,
            ) {
                Self::Sllw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b01000000000000000101000000111011,
            ) {
                Self::Sraw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000101000000111011,
            ) {
                Self::Srlw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b01000000000000000000000000111011,
            ) {
                Self::Subw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    rs2: raw.rs2(),
                }
            } else {
                Self::Invalid
            }
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
                0b11010010001000000000000001010011,
            ) {
                Self::FcvtDL {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010010001100000000000001010011,
            ) {
                Self::FcvtDLu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
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
                0b11000010001000000000000001010011,
            ) {
                Self::FcvtLD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000110001000000000000001010011,
            ) {
                Self::FcvtLQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000000001000000000000001010011,
            ) {
                Self::FcvtLS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000010001100000000000001010011,
            ) {
                Self::FcvtLuD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000110001100000000000001010011,
            ) {
                Self::FcvtLuQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11000000001100000000000001010011,
            ) {
                Self::FcvtLuS {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
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
                0b11010110001000000000000001010011,
            ) {
                Self::FcvtQL {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010110001100000000000001010011,
            ) {
                Self::FcvtQLu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
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
                0b11010000001000000000000001010011,
            ) {
                Self::FcvtSL {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                    rm: raw.rm(),
                }
            } else if raw.matches(
                0b11111111111100000000000001111111,
                0b11010000001100000000000001010011,
            ) {
                Self::FcvtSLu {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
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
                0b11110010000000000000000001010011,
            ) {
                Self::FmvDX {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11110110000000000000000001010011,
            ) {
                Self::FmvQX {
                    frd: raw.frd(),
                    rs1: raw.rs1(),
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
                0b11100010000000000000000001010011,
            ) {
                Self::FmvXD {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
                }
            } else if raw.matches(
                0b11111111111100000111000001111111,
                0b11100110000000000000000001010011,
            ) {
                Self::FmvXQ {
                    rd: raw.rd(),
                    frs1: raw.frs1(),
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
                    shamt: raw.shamt6(),
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
                    shamt: raw.shamt6(),
                }
            } else if raw.matches(
                0b11111000000000000111000001111111,
                0b00000000000000000101000000010011,
            ) {
                Self::Srli {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt6(),
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
            if raw.matches(
                0b00000000000000000111000001111111,
                0b00000000000000000000000000011011,
            ) {
                Self::Addiw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    simm: raw.imm12(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000001000000011011,
            ) {
                Self::Slliw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b01000000000000000101000000011011,
            ) {
                Self::Sraiw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else if raw.matches(
                0b11111110000000000111000001111111,
                0b00000000000000000101000000011011,
            ) {
                Self::Srliw {
                    rd: raw.rd(),
                    rs1: raw.rs1(),
                    shamt: raw.shamt5(),
                }
            } else {
                Self::Invalid
            }
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
                0b00000000000000000011000000100011,
            ) {
                Self::Sd {
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
            } else if raw.matches(0b1110000000000011, 0b0010000000000001) {
                Self::CAddiw {
                    rs1rd: raw.crs1rd(),
                    nzsimm: raw.cnzimmi(),
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
            } else if raw.matches(0b1110000000000011, 0b1010000000000001) {
                Self::CJ { simm: raw.cimmj() }
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
            } else if raw.matches(0b1110000000000011, 0b0110000000000000) {
                Self::CLd {
                    rd: raw.crdq(),
                    rs1: raw.crs1q(),
                    uimm: raw.cimmd(),
                }
            } else if raw.matches(0b1110000000000011, 0b0110000000000010) {
                Self::CLdsp {
                    rd: raw.crd(),
                    uimm: raw.cimmldsp(),
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
            } else if raw.matches(0b1110000000000011, 0b1110000000000000) {
                Self::CSd {
                    rs1: raw.crs1q(),
                    rs2: raw.crs2q(),
                    uimm: raw.cimmd(),
                }
            } else if raw.matches(0b1110000000000011, 0b1110000000000010) {
                Self::CSdsp {
                    rs2: raw.crs2(),
                    uimm: raw.cimmsdsp(),
                }
            } else if raw.matches(0b1110000000000011, 0b0000000000000010) {
                Self::CSlli {
                    rs1rd: raw.crs1rd(),
                    nzuimm: raw.cimmsh6(),
                }
            } else if raw.matches(0b1110110000000011, 0b1000010000000001) {
                Self::CSrai {
                    rs1rd: raw.crs1rdq(),
                    nzuimm: raw.cimmsh6(),
                }
            } else if raw.matches(0b1110110000000011, 0b1000000000000001) {
                Self::CSrli {
                    rs1rd: raw.crs1rdq(),
                    nzuimm: raw.cimmsh6(),
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
