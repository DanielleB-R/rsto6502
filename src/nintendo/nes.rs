use std::rc::Rc;

use crate::Processor;

use super::{cartridge::CartridgeHandle, ines, ppu::Ppu, NesMemoryMap};

pub struct Nes {
    pub cartridge: CartridgeHandle,
    pub cpu: Processor<NesMemoryMap>,
    pub ppu: Ppu,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Self {
        let cartridge = Rc::new(ines::parse(rom));

        let ppu = Ppu::new(Rc::clone(&cartridge));

        let memory_map = NesMemoryMap::new(Rc::clone(&cartridge));
        let cpu = Processor::with_memory(memory_map);

        Self {
            cartridge,
            ppu,
            cpu,
        }
    }
}
