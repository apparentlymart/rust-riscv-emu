use crate::data::Byte;
use crate::data::HalfwordUnsigned as Halfword;
use crate::data::LongwordUnsigned as Longword;
use crate::data::QuadwordUnsigned as Quadword;
use crate::data::WordUnsigned as Word;

/// Represents the external memory bus of the CPU.
///
/// The CPU may access memory as either individual bytes, words, halfwords,
/// Longwords, or quadwords. Implementations of this trait can map
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
    fn read_longword(&mut self, addr: Addr) -> Result<Longword, MemoryError>;
    fn write_longword(&mut self, addr: Addr, data: Longword) -> Result<(), MemoryError>;
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

impl MemoryError {
    pub fn as_code_load_cause(self) -> crate::exception::ExceptionCause {
        match self {
            MemoryError::Misaligned => {
                crate::exception::ExceptionCause::InstructionAddressMisaligned
            }
            MemoryError::AccessFault => crate::exception::ExceptionCause::InstructionAccessFault,
            MemoryError::PageFault => crate::exception::ExceptionCause::InstructionPageFault,
        }
    }
    pub fn as_data_load_cause(self) -> crate::exception::ExceptionCause {
        match self {
            MemoryError::Misaligned => crate::exception::ExceptionCause::LoadAddressMisaligned,
            MemoryError::AccessFault => crate::exception::ExceptionCause::LoadAccessFault,
            MemoryError::PageFault => crate::exception::ExceptionCause::LoadPageFault,
        }
    }
    pub fn as_data_store_cause(self) -> crate::exception::ExceptionCause {
        match self {
            MemoryError::Misaligned => crate::exception::ExceptionCause::StoreAddressMisaligned,
            MemoryError::AccessFault => crate::exception::ExceptionCause::StoreAccessFault,
            MemoryError::PageFault => crate::exception::ExceptionCause::StorePageFault,
        }
    }
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

    fn read_longword(&mut self, addr: usize) -> Result<Longword, MemoryError> {
        let mut ret: Longword = 0;
        for s in 0..8 {
            ret = ret | ((self.buf[(addr + s) % self.buf.len()] as Longword) << (s * 8))
        }
        return Ok(ret);
    }

    fn write_longword(&mut self, addr: usize, data: Longword) -> Result<(), MemoryError> {
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

/// Wraps another `Bus` and uses a provided function to adjust incoming
/// addresses before calling the wrapped bus.
pub struct AddressTransformer<Addr, Wrapped, Callback>
where
    Wrapped: Bus<Addr>,
    Callback: Fn(Addr) -> Result<Addr, MemoryError>,
{
    wrapped: Wrapped,
    callback: Callback,
    phantom_addr: core::marker::PhantomData<Addr>,
}

impl<Addr, Wrapped, Callback> AddressTransformer<Addr, Wrapped, Callback>
where
    Wrapped: Bus<Addr>,
    Callback: Fn(Addr) -> Result<Addr, MemoryError>,
{
    // Consumes a bus and produces a wrapping `AddressTransformer` that will
    // adjust incoming addresses using the given function before passing
    // them on to the wrapped bus.
    //
    // If the callback returns an error, the underlying bus will not be called
    // at all and the error will be returned instead.
    pub fn new(mut wrapped: Wrapped, callback: Callback) -> Self {
        Self {
            wrapped: wrapped,
            callback: callback,
            phantom_addr: core::marker::PhantomData,
        }
    }

    pub fn translate_address(&self, addr: Addr) -> Result<Addr, MemoryError> {
        let callback = &self.callback;
        callback(addr)
    }
}

impl<Addr, Wrapped, Callback> Bus<Addr> for AddressTransformer<Addr, Wrapped, Callback>
where
    Wrapped: Bus<Addr>,
    Callback: Fn(Addr) -> Result<Addr, MemoryError>,
{
    fn read_byte(&mut self, addr: Addr) -> Result<Byte, MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.read_byte(addr),
            Err(e) => Err(e),
        }
    }

    fn read_halfword(&mut self, addr: Addr) -> Result<Halfword, MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.read_halfword(addr),
            Err(e) => Err(e),
        }
    }

    fn read_word(&mut self, addr: Addr) -> Result<Word, MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.read_word(addr),
            Err(e) => Err(e),
        }
    }

    fn read_longword(&mut self, addr: Addr) -> Result<Longword, MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.read_longword(addr),
            Err(e) => Err(e),
        }
    }

    fn read_quadword(&mut self, addr: Addr) -> Result<Quadword, MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.read_quadword(addr),
            Err(e) => Err(e),
        }
    }

    fn write_byte(&mut self, addr: Addr, data: Byte) -> Result<(), MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.write_byte(addr, data),
            Err(e) => Err(e),
        }
    }

    fn write_halfword(&mut self, addr: Addr, data: Halfword) -> Result<(), MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.write_halfword(addr, data),
            Err(e) => Err(e),
        }
    }

    fn write_word(&mut self, addr: Addr, data: Word) -> Result<(), MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.write_word(addr, data),
            Err(e) => Err(e),
        }
    }

    fn write_longword(&mut self, addr: Addr, data: Longword) -> Result<(), MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.write_longword(addr, data),
            Err(e) => Err(e),
        }
    }

    fn write_quadword(&mut self, addr: Addr, data: Quadword) -> Result<(), MemoryError> {
        match self.translate_address(addr) {
            Ok(addr) => self.wrapped.write_quadword(addr, data),
            Err(e) => Err(e),
        }
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

    fn read_longword(&mut self, addr: Outside) -> Result<Longword, MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.read_longword(addr),
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

    fn write_longword(&mut self, addr: Outside, data: Longword) -> Result<(), MemoryError> {
        match self.convert_address(addr) {
            Some(addr) => self.wrapped.write_longword(addr, data),
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
        assert_eq!(ram.read_longword(0).unwrap(), 0 as u64);
        assert_eq!(ram.read_quadword(0).unwrap(), 0 as u128);

        // Can also do unaligned reads.
        assert_eq!(ram.read_byte(1).unwrap(), 0 as u8);
        assert_eq!(ram.read_halfword(1).unwrap(), 0 as u16);
        assert_eq!(ram.read_word(1).unwrap(), 0 as u32);
        assert_eq!(ram.read_longword(1).unwrap(), 0 as u64);
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
            ram.read_longword(0).unwrap(),
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
            ram.read_longword(0).unwrap(),
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
            ram.read_longword(0).unwrap(),
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
            ram.read_longword(0).unwrap(),
            0x09_0a_0b_0c_de_ad_be_ef as u64
        );
        assert_eq!(
            ram.read_quadword(0).unwrap(),
            0x01_02_03_04_05_06_07_08_09_0a_0b_0c_de_ad_be_ef as u128
        );

        ram.write_longword(0, 0xfeedfacecafebeef).unwrap();
        assert_eq!(ram.read_byte(0).unwrap(), 0xef as u8);
        assert_eq!(ram.read_halfword(0).unwrap(), 0xbe_ef as u16);
        assert_eq!(ram.read_word(0).unwrap(), 0xca_fe_be_ef as u32);
        assert_eq!(
            ram.read_longword(0).unwrap(),
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
            ram.read_longword(1).unwrap(),
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
