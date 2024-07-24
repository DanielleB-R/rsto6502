pub trait Memory: Clone {
    fn read(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read(addr), self.read(addr + 1)])
    }
    fn read_signed(&self, addr: u16) -> i8 {
        i8::from_le_bytes([self.read(addr)])
    }
    fn write(&mut self, addr: u16, data: u8);
    fn length(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct RandomAccessMemory {
    pub contents: Vec<u8>,
}

impl RandomAccessMemory {
    pub fn new(size: usize) -> Self {
        RandomAccessMemory {
            contents: vec![0; size],
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

#[derive(Debug, Clone)]
pub struct ReadOnlyMemory {
    pub contents: Vec<u8>,
}

impl ReadOnlyMemory {
    pub fn new(size: u16) -> Self {
        Self {
            contents: vec![0; size as usize],
        }
    }
}

impl From<Vec<u8>> for ReadOnlyMemory {
    fn from(value: Vec<u8>) -> Self {
        Self { contents: value }
    }
}

impl Memory for ReadOnlyMemory {
    fn read(&self, addr: u16) -> u8 {
        self.contents[addr as usize]
    }

    fn write(&mut self, _addr: u16, _data: u8) {
        // Silently discard the write
    }

    fn length(&self) -> usize {
        self.contents.len()
    }
}

#[derive(Debug, Clone)]
pub struct MirroredMemory<T: Memory> {
    pub underlying: T,
    pub mask: u16,
    pub size: usize,
}

impl<T> MirroredMemory<T>
where
    T: Memory,
{
    pub fn new(underlying: T, mask: u16, size: usize) -> Self {
        Self {
            underlying,
            mask,
            size,
        }
    }
}

impl<T> Memory for MirroredMemory<T>
where
    T: Memory,
{
    fn read(&self, addr: u16) -> u8 {
        self.underlying.read(addr & self.mask)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.underlying.write(addr & self.mask, data)
    }

    fn length(&self) -> usize {
        self.size
    }
}
