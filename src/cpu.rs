use crate::data::{IntBits, IntValue};
use crate::memory::Bus;
use crate::register::{Register, Registers};

pub struct CPU<XL, Addr, Mem>
where
    XL: IntBits<Address = Addr>,
    Addr: IntValue,
    Mem: Bus<Addr>,
{
    memory: Mem,
    registers: Registers<XL>,
    pc: XL,
}

impl<XL, Addr, Mem> CPU<XL, Addr, Mem>
where
    XL: IntBits<Address = Addr>,
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

    pub fn read_register(&self, reg: Register) -> XL {
        self.registers.read(reg)
    }


    pub fn write_register(&mut self, reg: Register, v: XL) {
        self.registers.write(reg, v)
    }

}
