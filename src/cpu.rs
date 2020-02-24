use crate::data::Int;
use crate::memory::Bus;
use crate::register::{IntRegister, Registers};

pub struct CPU<IntData, Mem>
where
    IntData: Int,
    Mem: Bus<IntData>,
{
    memory: Mem,
    registers: Registers<IntData>,
    pc: IntData,
}

impl<IntData, Mem> CPU<IntData, Mem>
where
    IntData: Int,
    Mem: Bus<IntData>,
{
    pub fn new(memory: Mem) -> Self {
        Self {
            memory: memory,
            registers: Registers::new(),
            pc: IntData::zero(),
        }
    }

    pub fn read_int_register(&self, reg: IntRegister) -> IntData {
        self.registers.read_int(reg)
    }

    pub fn write_int_register(&mut self, reg: IntRegister, v: IntData) {
        self.registers.write_int(reg, v)
    }
}
