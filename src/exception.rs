
/// Represents the exception cause codes from the RISC-V machine ISA, as would
/// be written to the `mcause` CSR.
pub enum ExceptionCause {
    // Non-interrupt causes
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    ECallFromUser = 8,
    ECallFromSupervisor = 9,
    ECallFromMachine = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,

    // Interrupt causes (these all have the MSB set)
    SoftwareInterruptFromUser = (1 << 31) | 0,
    SoftwareInterruptFromSupervisor = (1 << 31) | 1,
    SoftwareInterruptFromMachine = (1 << 31) | 3,
    TimerInterruptFromUser = (1 << 31) | 4,
    TimerInterruptFromSupervisor = (1 << 31) | 5,
    TimerInterruptFromMachine = (1 << 31) | 7,
    ExternalInterruptFromUser = (1 << 31) | 8,
    ExternalInterruptFromSupervisor = (1 << 31) | 9,
    ExternalInterruptFromMachine = (1 << 31) | 11,
}
