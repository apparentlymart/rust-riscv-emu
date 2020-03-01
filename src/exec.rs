mod exec_32;

pub use exec_32::step_rv32;

/// Represents the outcome of perfoming one or more execution steps on a Hart.
#[derive(Debug)]
pub enum ExecStatus<Addr> {
    /// Indicates that the hart is ready to take one or more additional steps
    /// with no external stimulus or response.
    Running,

    /// Indicates that the executor encountered a "wait for interrupt"
    /// instruction. The caller may wish to pause execution until an interrupt
    /// has been requested.
    ///
    /// It is acceptable for a caller to treat `WaitingForInterrupt` as an
    /// alias for `Running`, because the `wfi` instruction is just a hint
    /// for execution environments that are able to make use of that
    /// information, such as by switching to a low-power mode.
    WaitingForInterrupt,

    /// Indicates that the executor encountered an "environment call"
    /// instruction that is _not_ being handled internally by the hart.
    ///
    /// This can be used to provide services to the execution environment
    /// from the calling Rust program.
    ///
    /// A `Hart` implementation that implements multiple privilege modes may
    /// intercept calls from some or all privilege levels itself, in which
    /// case `EnvironmentCall` would _not_ be reported.
    ///
    /// The argument is the address of the `ecall` instruction that caused
    /// this status to be generated. Use this rather than the hart's own
    /// program counter value because the hart PC will already have been
    /// adjusted to point to the `ecall`'s direct successor.
    EnvironmentCall(Addr),

    /// Indicates that the executor encountered an "environment break"
    /// instruction that is _not_ being handled internally by the hart.
    ///
    /// For example, this could be used when the caller is implementing a
    /// debugging interface, allowing an environment break instruction to
    /// serve as a breakpoint transferring control back into the calling
    /// Rust program.
    ///
    /// A `Hart` implementation that implements multiple privilege modes
    /// may intercept breaks from some or all privilege levels itself, in
    /// which case `EnvironmentBreak` would _not_ be reported.
    ///
    /// The argument is the address of the `ebreak` instruction that caused
    /// this status to be generated. Use this rather than the hart's own
    /// program counter value because the hart PC will already have been
    /// adjusted to point to the `ebreak`'s direct successor.
    EnvironmentBreak(Addr),
}
