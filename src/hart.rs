use crate::exception::ExceptionCause;
use crate::memory::Bus;
use crate::register::{CSRError, ControlStatusRegister, FloatRegister, IntRegister};

/// Represents the mutable state for a RISC-V "hardware thread", and is
/// responsible for providing the execution environment(s) for code running
/// on that thread.
///
/// A `Hart` does not run code itself, but rather provides the mutable system
/// state that code could manipulate.
///
/// A `Hart` is generic over any type for address, integer data, and floating
/// point data.
pub trait Hart<Addr, IntData, FloatData> {
    /// Reads the current value of the program counter register.
    ///
    /// The Hart implementation itself does not enforce the alignment
    /// invariants required by the ISA on either read or write, so the
    /// caller must handle that somehow itself.
    fn read_pc(&self) -> Addr;

    /// Writes a new value for the program counter register.
    ///
    /// The Hart implementation itself does not enforce the alignment
    /// invariants required by the ISA on either read or write, so the
    /// caller must handle that somehow itself.
    fn write_pc(&mut self, v: Addr);

    /// Reads the current value of the given integer register.
    fn read_int_register(&self, reg: IntRegister) -> IntData;

    /// Writes a new value to the given integer register.
    fn write_int_register(&mut self, reg: IntRegister, v: IntData);

    /// Reads the current value of the given floating point register.
    fn read_float_register(&self, reg: FloatRegister) -> FloatData;

    /// Writes a new value to the given floating point register.
    fn write_float_register(&mut self, reg: FloatRegister, v: FloatData);

    /// Reads the current value of the given CSR.
    fn read_csr(&self, reg: ControlStatusRegister) -> Result<IntData, CSRError>;

    /// Writes a new value to the given CSR.
    fn write_csr(&mut self, reg: ControlStatusRegister, v: IntData) -> Result<(), CSRError>;

    /// Calls the given closure with a mutable borrow of the hart's memory bus,
    /// taking any necessary steps to ensure if a bus is shared between
    /// many harts that they do so safely. Exactly what "safe" means is
    /// left to the implementation to define.
    fn with_memory<R>(&mut self, f: impl FnOnce(&mut Bus<Addr>) -> R) -> R;

    /// Returns the hard to the reset state required by the relevant ISA
    /// spec, recording the given value as the reset cause.
    fn reset(&mut self, cause: IntData);

    /// Allows an external caller to trigger exception-handling
    /// behavior appropriate to the current execution environment and
    /// execution state. This might be used by an external instruction
    /// execution implementation, for example, to signal an error condition
    /// related to the current instruction being executed.
    ///
    /// How exactly the exception handling modifies the hart state is
    /// implementation-defined. For an implementation following the
    /// standard machine-level ISA specification, it is likely to update
    /// some CSRs related to trap handling and alter the program counter
    /// to refer to a trap handling function.
    fn exception(&mut self, cause: ExceptionCause);

    /// Signals a data memory fence, as represented by the `fence`
    /// instruction in the RISC-V base integer ISAs.
    ///
    /// Exactly what this does, if anything, is defined by the implementation.
    /// An implementation that includes per-hart data caches might flush
    /// those caches in response to a call to this method, for example.
    ///
    /// The predecessor and successor fields from the `fence` instruction
    /// are not currently exposed. The definition of this method may change
    /// in future to include precessor and successor information.
    fn fence_data(&mut self) {
        // default implementation does nothing
    }

    /// Signals an instruction memory fence, as represented by the `fence.i`
    /// instruction in the RISC-V base integer ISAs.
    ///
    /// Exactly what this does, if anything, is defined by the implementation.
    /// An implementation that includes per-hart code caches might flush
    /// those caches in response to a call to this method, for example.
    fn fence_code(&mut self) {
        // default implementation does nothing
    }

    /// Signals a virtual memory configuration fence, as represented by the
    /// `sfence.vma` instruction in the Supervisor-level ISA.
    ///
    /// The `rs1` and `rs2` arguments have the meaning defined for the
    /// `sfence.vma` instruction.
    ///
    /// This method may trigger exception handling as a side-effect, as if
    /// there had been a call to the method "exception", if the current
    /// hart state does not permit virtual memory updates or if the
    /// Supervisor-level ISA is not included in this implementation at all.
    fn fence_virtual_memory_config(&mut self, rs1: IntRegister, rs2: IntRegister) {
        // default implementation raises an illegal instruction exception,
        // to suggest that the supervisor-level ISA is not implemented
        // at all.
        self.exception(ExceptionCause::IllegalInstruction)
    }
}
