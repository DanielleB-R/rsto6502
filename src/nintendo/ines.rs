use super::cartridge::{Cartridge, NROMCartridge};

// panics if the rom is invalid
pub fn parse(rom: &[u8]) -> Box<dyn Cartridge> {
    let header = &rom[0..16];

    // check magic number
    assert_eq!(header[0], 0x4e);
    assert_eq!(header[1], 0x45);
    assert_eq!(header[2], 0x53);
    assert_eq!(header[3], 0x1a);

    let prg_size = 0x4000 * (header[4] as usize);
    // let chr_size = 0x2000 * (header[5] as usize);

    // ignoring trainers for now
    let chr_start_offset = prg_size + 16;
    // let chr_end_offset = chr_start_offset + chr_size;

    let prg = &rom[16..chr_start_offset];

    Box::new(NROMCartridge::new(prg))
}
