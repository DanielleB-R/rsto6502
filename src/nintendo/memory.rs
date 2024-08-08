use crate::memory::{self, Memory};

use super::cartridge::Cartridge;

#[derive(Debug, Clone)]
pub struct PpuProxy;

impl Memory for PpuProxy {
    fn read(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0x8
    }
}

#[derive(Debug, Clone)]
pub struct ApuIoProxy;

impl Memory for ApuIoProxy {
    fn read(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0x20
    }
}

pub struct NesMemoryMap {
    pub mirrored_ram: memory::MirroredMemory<memory::RandomAccessMemory>,
    pub ppu_proxy: memory::MirroredMemory<PpuProxy>,
    pub apu_io_proxy: ApuIoProxy,
    pub cartridge: *mut dyn Cartridge,
}

impl NesMemoryMap {
    pub fn new(cartridge: *mut dyn Cartridge) -> Self {
        Self {
            mirrored_ram: memory::MirroredMemory::new(
                memory::RandomAccessMemory::new(0x0800),
                0x07ff,
                0x2000,
            ),
            ppu_proxy: memory::MirroredMemory::new(PpuProxy, 0x0007, 0x2000),
            apu_io_proxy: ApuIoProxy,
            cartridge,
        }
    }
}

impl Memory for NesMemoryMap {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.mirrored_ram.read(addr),
            0x2000..=0x3fff => self.ppu_proxy.read(addr - 0x2000),
            0x4000..=0x401f => self.apu_io_proxy.read(addr - 0x4000),
            _ => unsafe { (&*self.cartridge).read(addr) },
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1fff => self.mirrored_ram.write(addr, data),
            0x2000..=0x3fff => self.ppu_proxy.write(addr - 0x2000, data),
            0x4000..=0x401f => self.apu_io_proxy.write(addr - 0x4000, data),
            _ => unsafe { (&mut *self.cartridge).write(addr, data) },
        }
    }

    fn length(&self) -> usize {
        0x10000
    }
}
