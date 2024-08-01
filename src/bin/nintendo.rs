use rsto6502::{nintendo, Memory, Processor};
use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();

    let filename = args[1].clone();

    let rom_bytes = fs::read(filename).unwrap();

    let cartridge = nintendo::parse(&rom_bytes);

    let memory_map = nintendo::NesMemoryMap::new(cartridge);

    let mut processor = Processor::with_memory(memory_map);

    processor.core.pc = 0xc000;

    while processor.memory.read(0x02) == 0 && processor.memory.read(0x03) == 0 {
        println!("{:04X}", processor.core.pc);
        if processor.core.pc == 0xc9F6 || processor.core.pc == 0xc9f8 {
            eprintln!("{}", processor.core);
        }
        processor.emulate_instruction();
    }

    eprintln!(
        "0x{:02x} 0x{:02x}",
        processor.memory.read(0x02),
        processor.memory.read(0x03)
    );
}
