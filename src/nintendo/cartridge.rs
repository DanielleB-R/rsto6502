use crate::memory::Memory;

#[derive(Debug, Clone)]
pub struct Cartridge;

impl Memory for Cartridge {
    fn read(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0xbfe0 // the length of an NES cart
    }
}
