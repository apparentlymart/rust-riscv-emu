use crate::data::Int;
use core::fmt;

pub enum Register {
    Int(IntRegister),
    Float(FloatRegister),
    ControlStatus(ControlStatusRegister),
}

#[derive(Clone, Copy)]
pub struct IntRegister(usize);

impl IntRegister {
    pub fn numbered(n: usize) -> Self {
        if n > 31 {
            panic!("register number out of range (0-31 inclusive)");
        }
        return Self(n);
    }

    // Decodes a 3-bit integer register selection from a compressed instruction
    // into its equivalent 5-bit register number.
    pub fn c_numbered(n: usize) -> Self {
        // The rd′, rs1′, and rs2′ arguments correspond to registers
        // x8 through x15.
        IntRegister::numbered(n + 8)
    }

    pub fn zero() -> Self {
        Self::numbered(0)
    }

    pub fn num(&self) -> usize {
        self.0
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

#[derive(Clone, Copy)]
pub struct FloatRegister(usize);

impl FloatRegister {
    pub fn numbered(n: usize) -> Self {
        if n > 31 {
            panic!("float register number out of range (0-31 inclusive)");
        }
        return Self(n);
    }

    // Decodes a 3-bit float register selection from a compressed instruction
    // into its equivalent 5-bit register number.
    pub fn c_numbered(n: usize) -> Self {
        // The rd′, rs1′, and rs2′ arguments correspond to registers
        // x8 through x15.
        FloatRegister::numbered(n + 8)
    }

    pub fn num(&self) -> usize {
        self.0
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

#[derive(Clone, Copy)]
pub struct ControlStatusRegister(usize);

impl ControlStatusRegister {
    pub fn numbered(n: usize) -> Self {
        if n >= 4096 {
            panic!("CSR number out of range (0-4095 inclusive)");
        }
        return Self(n);
    }

    pub fn num(&self) -> usize {
        self.0
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
