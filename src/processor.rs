use crate::flags::Flags;
use crate::memory::{Memory, RandomAccessMemory};

#[derive(Clone, Debug, PartialEq)]
pub struct Core {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub f: Flags,
    pub sp: u8,
    pub pc: u16,
}

#[derive(Clone, Debug)]
pub struct Processor {
    pub core: Core,
    pub memory: RandomAccessMemory,
}

impl Core {
    pub fn new() -> Core {
        Core {
            a: 0,
            x: 0,
            y: 0,
            f: Flags::default(),
            sp: 0xfe,
            pc: 0x0000,
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            core: Core::new(),
            memory: RandomAccessMemory::new(0xffff),
        }
    }

    pub(crate) fn lda(&mut self, addr: u16) {
        self.core.a = self.memory.read(addr);
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_new() {
        let core = Core::new();

        assert_eq!(core.a, 0);
        assert_eq!(core.x, 0);
        assert_eq!(core.y, 0);
        assert_eq!(core.f, Flags::default());
        assert_eq!(core.sp, 0xfe);
        assert_eq!(core.pc, 0x0000);
    }

    #[test]
    fn test_lda() {
        let mut cpu = Processor::new();
        let addr: u16 = 0x1000;

        let mut val = 0x77;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(
            cpu.core,
            Core {
                a: val,
                ..Core::default()
            }
        );

        val = 0xf8;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(
            cpu.core,
            Core {
                a: val,
                f: Flags {
                    n: true,
                    ..Flags::default()
                },
                ..Core::default()
            }
        );

        val = 0x00;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(
            cpu.core,
            Core {
                a: val,
                f: Flags {
                    z: true,
                    ..Flags::default()
                },
                ..Core::default()
            }
        );
    }
}
