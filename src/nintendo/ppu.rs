use super::cartridge::Cartridge;
use crate::{Memory, MirroredMemory, RandomAccessMemory};

#[derive(Debug, Clone)]
pub struct PpuMemory {
    pub ram: RandomAccessMemory,
    pub cartridge: *mut dyn Cartridge,
    pub palette_ram: MirroredMemory<RandomAccessMemory>,
}

impl PpuMemory {
    pub fn new(cartridge: *mut dyn Cartridge) -> Self {
        Self {
            ram: RandomAccessMemory::new(0x1000),
            cartridge,
            palette_ram: MirroredMemory::new(RandomAccessMemory::new(0x0020), 0x001f, 0x0100),
        }
    }

    pub fn chr(&self) -> &mut dyn Memory {
        unsafe { (&mut *self.cartridge).chr_mut() }
    }
}

impl Memory for PpuMemory {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.chr().read(addr),
            0x2000..=0x2fff => self.ram.read(addr & 0x1fff),
            0x3000..=0x3eff => unimplemented!(),
            0x3f00..=0x3fff => self.palette_ram.read(addr & 0x00ff),
            _ => panic!("Invalid PPU address"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1fff => self.chr().write(addr, data),
            0x2000..=0x2fff => self.ram.write(addr & 0x1fff, data),
            0x3000..=0x3eff => unimplemented!(),
            0x3f00..=0x3fff => self.palette_ram.write(addr & 0x00ff, data),
            _ => panic!("Invalid PPU address"),
        }
    }

    fn length(&self) -> usize {
        0x4000
    }
}

#[derive(Debug, Clone)]
pub struct Ppu {
    pub memory: PpuMemory,
    pub oam: [u8; 0x100],
}

impl Ppu {
    pub fn new(cartridge: *mut dyn Cartridge) -> Self {
        Self {
            memory: PpuMemory::new(cartridge),
            oam: [0; 0x100],
        }
    }
}
