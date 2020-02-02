pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read(addr), self.read(addr + 1)])
    }
    fn write(&mut self, addr: u16, data: u8);
    fn length(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct RandomAccessMemory {
    pub contents: Vec<u8>,
}

impl RandomAccessMemory {
    pub fn new(size: u16) -> Self {
        RandomAccessMemory {
            contents: vec![0; size as usize],
        }
    }
}

impl Memory for RandomAccessMemory {
    fn read(&self, addr: u16) -> u8 {
        self.contents[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.contents[addr as usize] = data;
    }

    fn length(&self) -> usize {
        self.contents.len()
    }
}
