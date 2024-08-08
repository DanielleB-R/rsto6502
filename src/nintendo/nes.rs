use crate::Processor;

use super::{cartridge::Cartridge, ines, ppu::Ppu, NesMemoryMap};

pub struct Nes {
    pub cartridge: *mut dyn Cartridge,
    pub cpu: Processor<NesMemoryMap>,
    pub ppu: Ppu,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Self {
        let cartridge = ines::parse(rom);
        let cartridge_ptr = Box::into_raw(cartridge);

        let ppu = Ppu::new(cartridge_ptr);

        let memory_map = NesMemoryMap::new(cartridge_ptr);
        let cpu = Processor::with_memory(memory_map);

        Self {
            cartridge: cartridge_ptr,
            ppu,
            cpu,
        }
    }
}

impl Drop for Nes {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.cartridge)) };
    }
}
