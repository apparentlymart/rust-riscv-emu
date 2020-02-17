use core::fmt;
use crate::data::{IntBits};

pub struct Register(usize);

impl Register {
    pub fn num(n: usize) -> Self {
        if n > 31 {
            panic!("register number out of range (0-31 inclusive)");
        }
        return Self(n);
    }
}

impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Register({})", self.0)
    }
}

pub struct Registers<XL: IntBits>([XL; 32]);

impl<XL: IntBits> Registers<XL> {
    pub fn new() -> Self {
        Self([XL::zero(); 32])
    }

    pub fn read(&self, reg: Register) -> XL {
        if reg.0 == 0 {
            // Register zero always returns zero.
            return XL::zero();
        }
        self.0[reg.0]
    }

    pub fn write(&mut self, reg: Register, v: XL) {
        if reg.0 == 0 {
            // Writing to register zero is always a no-op.
        }
        self.0[reg.0] = v
    }
}

