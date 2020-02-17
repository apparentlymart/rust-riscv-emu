
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
pub enum MemoryError {
    Misaligned,
    AccessFault,
    PageFault,
}
