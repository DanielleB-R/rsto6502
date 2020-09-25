mod decompiler;
mod flags;
mod instructions;
mod macros;
mod memory;
mod processor;

pub use decompiler::Decompiler;
pub use memory::{Memory, RandomAccessMemory};
pub use processor::Processor;

#[cfg(test)]
mod test {
    use super::*;
    use memory::Memory;

    #[test]
    fn test_functional_test() {
        let mut mem = memory::RandomAccessMemory::new(0);
        let test_bin = std::fs::read("6502_functional_test.bin").unwrap();
        mem.contents = test_bin;

        let mut processor = Processor::with_memory(mem);
        processor.core.pc = 0x400;
        processor.core.sp = 0xfd;

        let mut last_pc = 0;
        while processor.core.pc != last_pc {
            last_pc = processor.core.pc;
            processor.emulate_instruction();
        }

        if processor.core.pc != 0x3399 {
            println!(
                "0x{:x} not the correct termination point",
                processor.core.pc
            );
            println!("{:?}", processor.core);
            panic!("failed");
        }
    }
}
