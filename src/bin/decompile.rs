use rsto6502::Decompiler;
use std::env::args;
use std::fs::read;

fn main() {
    let filename = args().nth(1).unwrap();
    let contents = read(filename).unwrap();
    let decompiler = Decompiler::new(&contents);
    decompiler.decompile();
}
