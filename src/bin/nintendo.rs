use rsto6502::{nintendo, Memory};
use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();

    let filename = args[1].clone();

    let rom_bytes = fs::read(filename).unwrap();

    let mut nes = nintendo::Nes::new(&rom_bytes);

    nes.cpu.core.pc = 0xc000;
    nes.cpu.core.f.i = true;
    nes.cpu.cycles = 7;

    while nes.cpu.memory.read(0x02) == 0 && nes.cpu.memory.read(0x03) == 0 {
        let old_pc = nes.cpu.core.pc;
        let old_core_spec = format!("{}", nes.cpu);
        nes.cpu.emulate_instruction();
        println!("{:04X}  {}", old_pc, old_core_spec);
    }

    eprintln!(
        "0x{:02x} 0x{:02x}",
        nes.cpu.memory.read(0x02),
        nes.cpu.memory.read(0x03)
    );
}
