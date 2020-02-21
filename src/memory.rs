
pub type Word = u32;
pub type Halfword = u16;
pub type Doubleword = u64;
pub type Quadword = u128;
pub type Byte = u8;

/// Represents the external memory bus of the CPU.
///
/// The CPU may access memory as either individual bytes, words, halfwords,
/// doublewords, or quadwords. Implementations of this trait can map
/// memory access requests onto raw host memory buffers (to simulate RAM or ROM)
/// or to simulated registers of an emulated memory-mapped IO device.
///
/// If a memory access returns a `MemoryError` then the depending instruction
/// will fail with a suitable exception.
pub trait Bus<Addr> {
    fn read_byte(&mut self, addr: Addr) -> Result<Byte, MemoryError>;
    fn write_byte(&mut self, addr: Addr, data: Byte) -> Result<(), MemoryError>;
    fn read_word(&mut self, addr: Addr) -> Result<Word, MemoryError>;
    fn write_word(&mut self, addr: Addr, data: Word) -> Result<(), MemoryError>;
    fn read_halfword(&mut self, addr: Addr) -> Result<Halfword, MemoryError>;
    fn write_halfword(&mut self, addr: Addr, data: Halfword) -> Result<(), MemoryError>;
    fn read_doubleword(&mut self, addr: Addr) -> Result<Doubleword, MemoryError>;
    fn write_doubleword(&mut self, addr: Addr, data: Doubleword) -> Result<(), MemoryError>;
    fn read_quadword(&mut self, addr: Addr) -> Result<Quadword, MemoryError>;
    fn write_quadword(&mut self, addr: Addr, data: Quadword) -> Result<(), MemoryError>;
}

/// Represents the ways in which a memory access can fail. These map indirectly
/// onto the processor's exception codes, but the exact mapping depends on
/// what exactly the CPU was aiming to achieve with the particular memory access.
#[derive(Debug)]
pub enum MemoryError {
    Misaligned,
    AccessFault,
    PageFault,
}

pub struct Memory<'b> {
    buf: &'b mut [u8],
    writable: bool,
}

impl<'b> Memory<'b> {
    pub fn new_ram(buf: &'b mut [u8]) -> Self {
        Self {
            buf: buf,
            writable: true,
        }
    }

    pub fn new_rom(buf: &'b mut [u8]) -> Self {
        Self {
            buf: buf,
            writable: false,
        }
    }
}

impl<'b> Bus<usize> for Memory<'b> {
    fn read_byte(&mut self, addr: usize) -> Result<Byte, MemoryError> {
        return Ok(self.buf[addr % self.buf.len()]);
    }

    fn write_byte(&mut self, addr: usize, data: Byte) -> Result<(), MemoryError> {
        if !self.writable {
            return Err(MemoryError::AccessFault);
        }
        let l: usize;
        {
            l = self.buf.len();
        }
        self.buf[addr % l] = data;
        return Ok(());
    }

    fn read_word(&mut self, addr: usize) -> Result<Word, MemoryError> {
        let mut ret: Word = 0;
        for s in 0..4 {
            ret = ret | ((self.buf[(addr + s) % self.buf.len()] as Word) << (s * 8))
        }
        return Ok(ret);
    }

    fn write_word(&mut self, addr: usize, data: Word) -> Result<(), MemoryError> {
        if !self.writable {
            return Err(MemoryError::AccessFault);
        }
        let l: usize;
        {
            l = self.buf.len();
        }
        for s in 0..4 {
            self.buf[(addr + s) % l] = ((data >> (s * 8)) & 0xff) as u8;
        }
        return Ok(());
    }

    fn read_halfword(&mut self, addr: usize) -> Result<Halfword, MemoryError> {
        let mut ret: Halfword = 0;
        for s in 0..2 {
            ret = ret | ((self.buf[(addr + s) % self.buf.len()] as Halfword) << (s * 8))
        }
        return Ok(ret);
    }

    fn write_halfword(&mut self, addr: usize, data: Halfword) -> Result<(), MemoryError> {
        if !self.writable {
            return Err(MemoryError::AccessFault);
        }
        let l: usize;
        {
            l = self.buf.len();
        }
        for s in 0..2 {
            self.buf[(addr + s) % l] = ((data >> (s * 8)) & 0xff) as u8;
        }
        return Ok(());
    }

    fn read_doubleword(&mut self, addr: usize) -> Result<Doubleword, MemoryError> {
        let mut ret: Doubleword = 0;
        for s in 0..8 {
            ret = ret | ((self.buf[(addr + s) % self.buf.len()] as Doubleword) << (s * 8))
        }
        return Ok(ret);
    }

    fn write_doubleword(&mut self, addr: usize, data: Doubleword) -> Result<(), MemoryError> {
        if !self.writable {
            return Err(MemoryError::AccessFault);
        }
        let l: usize;
        {
            l = self.buf.len();
        }
        for s in 0..8 {
            self.buf[(addr + s) % l] = ((data >> (s * 8)) & 0xff) as u8;
        }
        return Ok(());
    }

    fn read_quadword(&mut self, addr: usize) -> Result<Quadword, MemoryError> {
        let mut ret: Quadword = 0;
        for s in 0..16 {
            ret = ret | ((self.buf[(addr + s) % self.buf.len()] as Quadword) << (s * 8))
        }
        return Ok(ret);
    }

    fn write_quadword(&mut self, addr: usize, data: Quadword) -> Result<(), MemoryError> {
        if !self.writable {
            return Err(MemoryError::AccessFault);
        }
        let l: usize;
        {
            l = self.buf.len();
        }
        for s in 0..16 {
            self.buf[(addr + s) % l] = ((data >> (s * 8)) & 0xff) as u8;
        }
        return Ok(());
    }

}

/// Adapter type for wrapping a Bus that expects one address type to make it
/// appear instead as a Bus for another address type, as long as a conversion
/// is available from the "outer" address type to the "inner" address type.
pub struct AddressConverter<Outside, Inside, Wrapped>
where
    Outside: core::convert::TryInto<Inside>,
    Wrapped: Bus<Inside>,
{
    wrapped: Wrapped,
    phantom_outside: core::marker::PhantomData<Outside>,
    phantom_inside: core::marker::PhantomData<Inside>,
}

impl<Outside, Inside, Wrapped> AddressConverter<Outside, Inside, Wrapped>
where
    Outside: core::convert::TryInto<Inside>,
    Wrapped: Bus<Inside>,
{

    // Consumes a bus and produces a wrapping `AddressConverter` that will
    // convert incoming addresses to the given bus's address type.
    //
    // This can be useful, for example, to adapt a `Memory` instance (whose
    // address type is always `usize`) to the address size used by a specific
    // instantiation of `CPU`, such as `u32` for rv32 or `u64` for rv64.
    //
    // The signature of this function does not imply the "outside" type for
    // the address converter. To specify the outside type, assign the result
    // to something that implements `Bus` with the desired address type.
    pub fn new(mut wrapped: Wrapped) -> Self {
        Self {
            wrapped: wrapped,
            phantom_outside: core::marker::PhantomData,
            phantom_inside: core::marker::PhantomData,
        }
    }

    pub fn convert_address(&self, addr: Outside) -> Option<Inside> {
        match addr.try_into() {
            Ok(in_addr) => Some(in_addr),
            Err(_) => None,
        }
    }
}

impl<Outside, Inside, Wrapped> Bus<Outside> for AddressConverter<Outside, Inside, Wrapped>
where
    Outside: core::convert::TryInto<Inside>,
    Wrapped: Bus<Inside>,
{

    fn read_byte(&mut self, addr: Outside) -> Result<Byte, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_byte(addr),
            None => Err(MemoryError::PageFault),
        }
    }

    fn read_halfword(&mut self, addr: Outside) -> Result<Halfword, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_halfword(addr),
            None => Err(MemoryError::PageFault),
        }
    }

    fn read_word(&mut self, addr: Outside) -> Result<Word, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_word(addr),
            None => Err(MemoryError::PageFault),
        }
    }

    fn read_doubleword(&mut self, addr: Outside) -> Result<Doubleword, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_doubleword(addr),
            None => Err(MemoryError::PageFault),
        }
    }

    fn read_quadword(&mut self, addr: Outside) -> Result<Quadword, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_quadword(addr),
            None => Err(MemoryError::PageFault),
        }
    }

    fn write_byte(&mut self, addr: Outside, data: Byte) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_byte(addr, data),
            None => Err(MemoryError::PageFault),
        }
    }

    fn write_halfword(&mut self, addr: Outside, data: Halfword) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_halfword(addr, data),
            None => Err(MemoryError::PageFault),
        }
    }

    fn write_word(&mut self, addr: Outside, data: Word) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_word(addr, data),
            None => Err(MemoryError::PageFault),
        }
    }

    fn write_doubleword(&mut self, addr: Outside, data: Doubleword) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_doubleword(addr, data),
            None => Err(MemoryError::PageFault),
        }
    }

    fn write_quadword(&mut self, addr: Outside, data: Quadword) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_quadword(addr, data),
            None => Err(MemoryError::PageFault),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AddressConverter, Bus, Memory};

    #[test]
    fn memory_writable() {
        let mut buf: [u8; 32] = [0; 32];
        let mut ram = Memory::new_ram(&mut buf);

        // RAM is all zeroes initially, per our initializer above.
        assert_eq!(ram.read_byte(0).unwrap(), 0 as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0 as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0 as u32);
        assert_eq!(ram.read_doubleword(0).unwrap(), 0 as u64);
        assert_eq!(ram.read_quadword(0).unwrap(), 0 as u128);

        // Can also do unaligned reads.
        assert_eq!(ram.read_byte(1).unwrap(), 0 as u8);
        assert_eq!(ram.read_halfword(1).unwrap(), 0 as u16);
        assert_eq!(ram.read_word(1).unwrap(), 0 as u32);
        assert_eq!(ram.read_doubleword(1).unwrap(), 0 as u64);
        assert_eq!(ram.read_quadword(1).unwrap(), 0 as u128);

        // We'll write a quadword into location zero and then reinterpret its
        // bytes as other types. Note that this machine is little-endian,
        // so the "0x10" octet below should end up at address zero.
        ram.write_quadword(0, 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10)
            .unwrap();

        assert_eq!(ram.read_byte(0).unwrap(), 0x10 as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0x0f_10 as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0x0d_0e_0f_10 as u32);
        assert_eq!(
            ram.read_doubleword(0).unwrap(),
            0x09_0a_0b_0c_0d_0e_0f_10 as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10 as u128
        );

        ram.write_byte(0, 0xff).unwrap();
        assert_eq!(ram.read_byte(0).unwrap(), 0xff as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0x0f_ff as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0x0d_0e_0f_ff as u32);
        assert_eq!(
            ram.read_doubleword(0).unwrap(),
            0x09_0a_0b_0c_0d_0e_0f_ff as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_ff as u128
        );

        ram.write_halfword(0, 0xbeef).unwrap();
        assert_eq!(ram.read_byte(0).unwrap(), 0xef as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0xbe_ef as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0x0d_0e_be_ef as u32);
        assert_eq!(
            ram.read_doubleword(0).unwrap(),
            0x09_0a_0b_0c_0d_0e_be_ef as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_be_ef as u128
        );

        ram.write_word(0, 0xdeadbeef).unwrap();
        assert_eq!(ram.read_byte(0).unwrap(), 0xef as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0xbe_ef as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0xde_ad_be_ef as u32);
        assert_eq!(
            ram.read_doubleword(0).unwrap(),
            0x09_0a_0b_0c_de_ad_be_ef as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_09_0a_0b_0c_de_ad_be_ef as u128
        );

        ram.write_doubleword(0, 0xfeedfacecafebeef).unwrap();
        assert_eq!(ram.read_byte(0).unwrap(), 0xef as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0xbe_ef as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0xca_fe_be_ef as u32);
        assert_eq!(
            ram.read_doubleword(0).unwrap(),
            0xfe_ed_fa_ce_ca_fe_be_ef as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_fe_ed_fa_ce_ca_fe_be_ef as u128
        );

        // Repeat unaligned read testing with nonzero data.
        assert_eq!(ram.read_byte(1).unwrap(), 0xbe as u8);
        assert_eq!(ram.read_halfword(1).unwrap(), 0xfe_be as u16);
        assert_eq!(ram.read_word(1).unwrap(), 0xce_ca_fe_be as u32);
        assert_eq!(
            ram.read_doubleword(1).unwrap(),
            0x08_fe_ed_fa_ce_ca_fe_be as u64
        );
        assert_eq!(
            ram.read_quadword(1).unwrap(),
            0x00_01_02_03_04_05_06_07_08_fe_ed_fa_ce_ca_fe_be as u128
        );
    }

    #[test]
    fn address_converter() {
        let mut buf: [u8; 32] = [0; 32];
        let ram = Memory::new_ram(&mut buf);
        let mut bus = AddressConverter::<u32, usize, Memory>::new(ram);

        bus.write_byte(1, 0xfe).unwrap();
        assert_eq!(bus.read_byte(1).unwrap(), 0xfe as u8);
    }
}
