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
    processor.core.f.i = true;
    processor.cycles = 7;

    while processor.memory.read(0x02) == 0 && processor.memory.read(0x03) == 0 {
        let old_pc = processor.core.pc;
        let old_core_spec = format!("{}", processor);
        processor.emulate_instruction();
        println!("{:04X}  {}", old_pc, old_core_spec);
    }

    eprintln!(
        "0x{:02x} 0x{:02x}",
        processor.memory.read(0x02),
        processor.memory.read(0x03)
    );
}
