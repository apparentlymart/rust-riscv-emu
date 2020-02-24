use crate::data::Int;
use core::fmt;

pub enum Register {
    Int(IntRegister),
    Float(FloatRegister),
    ControlStatus(ControlStatusRegister),
}

pub struct IntRegister(usize);

impl IntRegister {
    pub fn num(n: usize) -> Self {
        if n > 31 {
            panic!("register number out of range (0-31 inclusive)");
        }
        return Self(n);
    }

    pub fn zero() -> Self {
        Self::num(0)
    }
}

impl PartialEq for IntRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for IntRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntRegister({})", self.0)
    }
}

impl core::convert::From<IntRegister> for Register {
    fn from(ir: IntRegister) -> Register {
        Register::Int(ir)
    }
}

pub struct FloatRegister(usize);

impl FloatRegister {
    pub fn num(n: usize) -> Self {
        if n > 31 {
            panic!("float register number out of range (0-31 inclusive)");
        }
        return Self(n);
    }
}

impl PartialEq for FloatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for FloatRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FloatRegister({})", self.0)
    }
}

impl core::convert::From<FloatRegister> for Register {
    fn from(fr: FloatRegister) -> Register {
        Register::Float(fr)
    }
}

pub struct ControlStatusRegister(usize);

impl ControlStatusRegister {
    pub fn num(n: usize) -> Self {
        if n >= 4096 {
            panic!("CSR number out of range (0-4095 inclusive)");
        }
        return Self(n);
    }
}

impl PartialEq for ControlStatusRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for ControlStatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ControlStatusRegister({})", self.0)
    }
}

impl core::convert::From<ControlStatusRegister> for Register {
    fn from(csr: ControlStatusRegister) -> Register {
        Register::ControlStatus(csr)
    }
}

/// Represents the ways in which a CSR can fail.
#[derive(Debug)]
pub enum CSRError {
    Unsupported,
    Misaligned,
    AccessFault,
}

pub struct Registers<XL: Int> {
    int: [XL; 32],
}

impl<XL: Int> Registers<XL> {
    pub fn new() -> Self {
        Self {
            int: [XL::zero(); 32],
        }
    }

    pub fn read_int(&self, reg: IntRegister) -> XL {
        if reg.0 == 0 {
            // Register zero always returns zero.
            return XL::zero();
        }
        self.int[reg.0]
    }

    pub fn write_int(&mut self, reg: IntRegister, v: XL) {
        if reg.0 == 0 {
            // Writing to register zero is always a no-op.
        }
        self.int[reg.0] = v
    }
}
