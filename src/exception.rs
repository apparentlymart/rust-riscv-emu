/// Represents the exception cause codes from the RISC-V machine ISA, as would
/// be written to the `mcause` CSR.
pub enum ExceptionCause {
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
}

/// Represents the interrupt cause codes from the RISC-V machine ISA.
pub enum InterruptCause {
    SoftwareInterruptFromUser = 0,
    SoftwareInterruptFromSupervisor = 1,
    SoftwareInterruptFromMachine = 3,
    TimerInterruptFromUser = 4,
    TimerInterruptFromSupervisor = 5,
    TimerInterruptFromMachine = 7,
    ExternalInterruptFromUser = 8,
    ExternalInterruptFromSupervisor = 9,
    ExternalInterruptFromMachine = 11,
}

/// Represents either an exception or an interrupt cause.
pub enum Cause {
    Exception(ExceptionCause),
    Interrupt(InterruptCause),
}
