use crate::data::{IntValueRaw, IntValue};
use crate::memory::Bus;
use crate::register::{IntRegister, Registers};

pub struct CPU<XL, Addr, Mem>
where
    XL: IntValueRaw<Address = Addr>,
    Addr: IntValue,
    Mem: Bus<Addr>,
{
    memory: Mem,
    registers: Registers<XL>,
    pc: XL,
}

impl<XL, Addr, Mem> CPU<XL, Addr, Mem>
where
    XL: IntValueRaw<Address = Addr>,
    Addr: IntValue,
    Mem: Bus<Addr>,
{
    pub fn new(memory: Mem) -> Self {
        Self {
            memory: memory,
            registers: Registers::new(),
            pc: XL::zero(),
        }
    }

    pub fn read_int_register(&self, reg: IntRegister) -> XL {
        self.registers.read_int(reg)
    }

    pub fn write_int_register(&mut self, reg: IntRegister, v: XL) {
        self.registers.write_int(reg, v)
    }
}
