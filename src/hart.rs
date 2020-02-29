use crate::data::{Float, Int, Zero};
use crate::exception::ExceptionCause;
use crate::isa::BaseISA;
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
pub trait Hart<Addr, IntData, FloatData, Mem>
where
    Mem: Bus<Addr>,
{
    /// Reads the current value of the program counter register.
    ///
    /// The Hart implementation itself does not enforce the alignment
    /// invariants required by the ISA on either read or write, so the
    /// caller must handle that somehow itself.
    fn read_pc(&self) -> Addr;

    /// Writes a new value for the program counter register.
    ///
    /// The Hart implementation does not enforce the alignment
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
    fn with_memory<R>(&mut self, f: impl FnOnce(&mut Mem) -> R) -> R;

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

    /// Called when handling an "environment call" instruction, to give the
    /// hart an opportunity to handle it.
    ///
    /// Returns `true` to indicate that the hart handled the environment call
    /// itself (e.g. by switching its internal privilege level and switching to
    /// an exception handler), or `false` to indicate that the call was made
    /// from the hart's outermost execution environment and must therefore be
    /// handled by the calling Rust program instead.
    fn environment_call(&mut self, addr: Addr) -> bool {
        // By default we let the caller handle it.
        false
    }

    /// Called when handling an "environment break" instruction, to give the
    /// hart an opportunity to handle it.
    ///
    /// Returns `true` to indicate that the hart handled the environment break
    /// itself (e.g. by giving control to a debugger running inside the Hart
    /// itself), or `false` to indicate that the break should be handled by the
    /// calling Rust program instead.
    fn environment_break(&mut self, addr: Addr) -> bool {
        // By default we let the caller handle it.
        false
    }
}

/// An implementation of `Hart` representing a single-core, single-threaded
/// user environment where one Hart is running in isolation with exclusive
/// access to a memory bus.
///
/// This implementation cannot represent one of many Harts in a multi-core or
/// hardware-multi-threaded system, because it takes exclusive ownership over
/// the memory bus it will use. (A multi-thread-aware implementation would need
/// to safely share access to a memory bus, e.g. via a mutex. No such
/// implementation is provided in this crate.)
///
/// This implementation does not support virtual memory, and treats fence
/// instructions as no-op.
pub struct SingleThreadUserHart<ISA, Mem>
where
    ISA: BaseISA,
    Mem: Bus<<ISA::Int as Int>::Unsigned>,
{
    pc: <ISA::Int as Int>::Unsigned,
    int_regs: [ISA::Int; 32],
    float_regs: [ISA::Float; 32],
    csrs: SingleThreadUserHartCSRs<ISA>,
    mem: Mem,
}

impl<ISA, Mem> SingleThreadUserHart<ISA, Mem>
where
    ISA: BaseISA,
    Mem: Bus<<ISA::Int as Int>::Unsigned>,
{
    pub fn new(mem: Mem) -> Self {
        Self {
            pc: Self::pc_at_reset(),
            int_regs: Self::int_registers_at_reset(),
            float_regs: Self::float_registers_at_reset(),
            csrs: Self::csrs_at_reset(),
            mem: mem,
        }
    }

    fn pc_at_reset() -> <ISA::Int as Int>::Unsigned {
        ISA::Int::from_unsigned_word(0).to_unsigned()
    }

    fn int_registers_at_reset() -> [ISA::Int; 32] {
        [
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
            ISA::Int::zero(),
        ]
    }

    fn float_registers_at_reset() -> [ISA::Float; 32] {
        [
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
            ISA::Float::zero(),
        ]
    }

    fn csrs_at_reset() -> SingleThreadUserHartCSRs<ISA> {
        SingleThreadUserHartCSRs {
            ustatus: ISA::Int::zero(),
            uie: ISA::Int::zero(),
            utvec: ISA::Int::zero(),
            uscratch: ISA::Int::zero(),
            uepc: ISA::Int::zero(),
            ucause: ISA::Int::zero(),
            utval: ISA::Int::zero(),
            uip: ISA::Int::zero(),
            fflags: ISA::Int::zero(),
            frm: ISA::Int::zero(),
        }
    }
}

impl<ISA, Mem> Hart<<ISA::Int as Int>::Unsigned, ISA::Int, ISA::Float, Mem>
    for SingleThreadUserHart<ISA, Mem>
where
    ISA: BaseISA,
    Mem: Bus<<ISA::Int as Int>::Unsigned>,
{
    fn read_pc(&self) -> <ISA::Int as Int>::Unsigned {
        self.pc
    }

    fn write_pc(&mut self, v: <ISA::Int as Int>::Unsigned) {
        self.pc = v
    }

    fn read_int_register(&self, reg: IntRegister) -> ISA::Int {
        self.int_regs[reg.num()]
    }

    fn write_int_register(&mut self, reg: IntRegister, v: ISA::Int) {
        if reg.num() == 0 {
            // Register zero is always fixed at zero, so we ignore this request.
            return;
        }
        self.int_regs[reg.num()] = v
    }

    fn read_float_register(&self, reg: FloatRegister) -> ISA::Float {
        self.float_regs[reg.num()]
    }

    fn write_float_register(&mut self, reg: FloatRegister, v: ISA::Float) {
        self.float_regs[reg.num()] = v
    }

    fn read_csr(&self, reg: ControlStatusRegister) -> Result<ISA::Int, CSRError> {
        match reg.num() {
            // TODO: Implement the CSRs that we have in SingleThreadUserHartCSRs
            _ => Err(CSRError::Unsupported),
        }
    }

    fn write_csr(&mut self, reg: ControlStatusRegister, v: ISA::Int) -> Result<(), CSRError> {
        match reg.num() {
            // TODO: Implement the CSRs that we have in SingleThreadUserHartCSRs
            _ => Err(CSRError::Unsupported),
        }
    }

    fn with_memory<R>(&mut self, f: impl FnOnce(&mut Mem) -> R) -> R {
        f(&mut self.mem)
    }

    fn reset(&mut self, cause: ISA::Int) {
        self.pc = Self::pc_at_reset();
        self.int_regs = Self::int_registers_at_reset();
        self.float_regs = Self::float_registers_at_reset();
        self.csrs = Self::csrs_at_reset();
        self.csrs.ucause = cause;
    }

    fn exception(&mut self, cause: ExceptionCause) {
        let vec_raw = self.csrs.utvec.to_unsigned();
        let mask = ISA::Int::from_unsigned_word(0b11).to_unsigned();
        let vec_base = vec_raw & !mask;

        // TODO: Implement vectored mode. For now we only support direct mode.
        //let vec_mode = vec_raw & mask;
        //let mode_vectored = ISA::Int::from_unsigned_word(1).to_unsigned();

        let new_pc: <ISA::Int as Int>::Unsigned = vec_base;
        self.write_pc(new_pc);
        self.csrs.ucause = ISA::Int::from_unsigned_word(cause as u32);
        // TODO" Other exception-related CSRs
    }
}

struct SingleThreadUserHartCSRs<ISA>
where
    ISA: BaseISA,
{
    pub ustatus: ISA::Int,
    pub uie: ISA::Int,
    pub utvec: ISA::Int,
    pub uscratch: ISA::Int,
    pub uepc: ISA::Int,
    pub ucause: ISA::Int,
    pub utval: ISA::Int,
    pub uip: ISA::Int,
    pub fflags: ISA::Int,
    pub frm: ISA::Int,
}

#[cfg(test)]
mod tests {
    use super::{Hart, SingleThreadUserHart};
    use crate::isa::RV32;
    use crate::memory::AddressConverter;
    use crate::memory::Bus;
    use crate::memory::Memory;
    use crate::register::{FloatRegister, IntRegister};

    #[test]
    fn single_thread_user_hart() {
        let mut mem_buf = [0 as u8; 1024];
        let mem = Memory::new_ram(&mut mem_buf);
        let mut hart: SingleThreadUserHart<RV32, AddressConverter<u32, usize, Memory>> =
            SingleThreadUserHart::new(AddressConverter::new(mem));

        let x0 = IntRegister::numbered(0);
        let x1 = IntRegister::numbered(1);
        let f0 = FloatRegister::numbered(0);

        assert_eq!(hart.read_int_register(x0), 0, "x0 initially zero");
        hart.write_int_register(x0, 2);
        assert_eq!(hart.read_int_register(x0), 0, "x0 still zero after write");

        assert_eq!(hart.read_int_register(x1), 0, "x1 initially zero");
        hart.write_int_register(x1, 2);
        assert_eq!(
            hart.read_int_register(x1),
            2,
            "x1 value changed after write"
        );

        assert_eq!(hart.read_float_register(f0), 0.0, "f0 initially zero");
        hart.write_float_register(f0, 1.5);
        assert_eq!(
            hart.read_float_register(f0),
            1.5,
            "f0 value changed after write"
        );

        hart.with_memory(|mem| {
            mem.write_byte(0, 5).unwrap();
            let v = mem.read_byte(0).unwrap();
            assert_eq!(v, 5, "was able to write to and then read from memory");
        })
    }
}
