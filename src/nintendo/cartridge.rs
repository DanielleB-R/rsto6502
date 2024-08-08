use crate::{memory::Memory, ReadOnlyMemory};

pub trait Cartridge {
    fn prg(&self) -> &dyn Memory;
    fn prg_mut(&mut self) -> &mut dyn Memory;

    fn chr(&self) -> &dyn Memory;
    fn chr_mut(&mut self) -> &mut dyn Memory;
}

impl<T: Cartridge + ?Sized> Memory for T {
    fn read(&self, addr: u16) -> u8 {
        self.prg().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.prg_mut().write(addr, data)
    }

    fn length(&self) -> usize {
        self.prg().length()
    }
}

#[derive(Debug, Clone)]
pub struct NullMemory;

impl Memory for NullMemory {
    fn read(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0xbfe0 // the length of an NES cart
    }
}

#[derive(Debug, Clone)]
pub struct NullCartridge {
    memory: NullMemory,
    chr_rom: NullMemory,
}

impl Cartridge for NullCartridge {
    fn prg(&self) -> &dyn Memory {
        &self.memory
    }

    fn prg_mut(&mut self) -> &mut dyn Memory {
        &mut self.memory
    }

    fn chr(&self) -> &dyn Memory {
        &self.chr_rom
    }

    fn chr_mut(&mut self) -> &mut dyn Memory {
        &mut self.chr_rom
    }
}

struct NROM32KBMemory {
    pub prg_memory: [u8; 0x8000],
}

impl NROM32KBMemory {
    pub fn new(prg_bytes: [u8; 0x8000]) -> Self {
        Self {
            prg_memory: prg_bytes,
        }
    }
}

impl Memory for NROM32KBMemory {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7fff => 0,
            0x8000..=0xffff => self.prg_memory[(addr - 0x8000) as usize],
        }
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0xbfe0
    }
}

struct NROM16KBMemory {
    pub prg_memory: [u8; 0x4000],
}

impl NROM16KBMemory {
    pub fn new(prg_bytes: [u8; 0x4000]) -> Self {
        Self {
            prg_memory: prg_bytes,
        }
    }
}

impl Memory for NROM16KBMemory {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7fff => 0,
            0x8000..=0xbfff => self.prg_memory[(addr - 0x8000) as usize],
            0xc000..=0xffff => self.prg_memory[(addr - 0xc000) as usize],
        }
    }

    fn write(&mut self, _addr: u16, _data: u8) {}

    fn length(&self) -> usize {
        0xbfe0
    }
}

pub struct NROMCartridge {
    pub prg_rom: Box<dyn Memory>,
    pub chr_rom: ReadOnlyMemory,
}

impl NROMCartridge {
    pub fn new(prg_bytes: &[u8], chr_bytes: &[u8]) -> Self {
        Self {
            prg_rom: (match prg_bytes.len() {
                0x4000 => Box::new(NROM16KBMemory::new(prg_bytes.try_into().unwrap())),
                0x8000 => Box::new(NROM32KBMemory::new(prg_bytes.try_into().unwrap())),
                _ => panic!("NROM PRG size wrong"),
            }),
            chr_rom: chr_bytes.into(),
        }
    }
}

impl Cartridge for NROMCartridge {
    fn prg(&self) -> &dyn Memory {
        self.prg_rom.as_ref()
    }

    fn prg_mut(&mut self) -> &mut dyn Memory {
        self.prg_rom.as_mut()
    }

    fn chr(&self) -> &dyn Memory {
        &self.chr_rom
    }

    fn chr_mut(&mut self) -> &mut dyn Memory {
        &mut self.chr_rom
    }
}
