use std::fmt::Display;

use crate::decode_6502;
use crate::flags::Flags;
use crate::memory::Memory;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Core {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub f: Flags,
    pub sp: u8,
    pub pc: u16,
}

impl Display for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A:{:02X} X:{:02X} Y:{:02X} P:{} SP:{:02X}",
            self.a, self.x, self.y, self.f, self.sp
        )
    }
}

#[derive(Clone)]
pub struct Processor<T: Memory> {
    pub core: Core,
    pub memory: T,
    // TODO: Is there a better way to do this?
    pub jumped: bool,
    pub cycles: usize,
}

impl Core {
    pub fn new() -> Core {
        Core {
            a: 0,
            x: 0,
            y: 0,
            f: Flags::default(),
            sp: 0xfd,
            pc: 0x0000,
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Memory> Display for Processor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ppu_cycles = self.cycles * 3;
        write!(
            f,
            "{} PPU:{:3},{:3} CYC:{}",
            self.core,
            ppu_cycles / 341,
            ppu_cycles % 341,
            self.cycles
        )
    }
}

impl<T: Memory> Processor<T> {
    pub fn with_memory(memory: T) -> Self {
        Self {
            core: Core::new(),
            memory,
            jumped: false,
            cycles: 0,
        }
    }

    fn branch(&mut self, addr: u16) {
        let offset = self.memory.read_signed(addr);
        self.core.pc = self.core.pc.wrapping_add(offset as u16);
        self.cycles += 1;
    }

    fn pull(&mut self) -> u8 {
        self.core.sp = self.core.sp.wrapping_add(1);
        self.memory.read(self.stack_addr())
    }

    fn push(&mut self, value: u8) {
        self.memory.write(self.stack_addr(), value);
        self.core.sp = self.core.sp.wrapping_sub(1);
    }

    fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push((value & 0xff) as u8);
    }

    fn stack_addr(&self) -> u16 {
        0x100 | (self.core.sp as u16)
    }

    // ADDRESSING MODES:
    pub(crate) fn immediate(&self) -> u16 {
        self.core.pc + 1
    }

    pub(crate) fn immediate_operand(&self) -> u8 {
        self.memory.read(self.immediate())
    }

    pub(crate) fn zero_page(&self) -> u16 {
        self.immediate_operand() as u16
    }

    pub(crate) fn zero_page_x(&self) -> u16 {
        self.immediate_operand().wrapping_add(self.core.x) as u16
    }

    pub(crate) fn zero_page_y(&self) -> u16 {
        self.immediate_operand().wrapping_add(self.core.y) as u16
    }

    pub(crate) fn absolute(&self) -> u16 {
        self.memory.read_word(self.immediate())
    }

    pub(crate) fn absolute_x(&mut self) -> u16 {
        let base = self.absolute();
        let addr = base.wrapping_add(self.core.x as u16);

        if base & 0xff00 != addr & 0xff00 {
            self.cycles += 1
        }

        addr
    }

    pub(crate) fn absolute_x_for_store(&self) -> u16 {
        self.absolute().wrapping_add(self.core.x as u16)
    }

    pub(crate) fn absolute_y(&mut self) -> u16 {
        let base = self.absolute();
        let addr = base.wrapping_add(self.core.y as u16);

        if base & 0xff00 != addr & 0xff00 {
            self.cycles += 1
        }

        addr
    }

    pub(crate) fn absolute_y_for_store(&self) -> u16 {
        self.absolute().wrapping_add(self.core.y as u16)
    }

    pub(crate) fn indirect(&self) -> u16 {
        self.wrapping_read(self.absolute())
    }

    fn wrapping_read(&self, indirect_addr: u16) -> u16 {
        // If the base address is 0x??ff, we don't read the word correctly
        if indirect_addr & 0x00ff == 0x00ff {
            u16::from_le_bytes([
                self.memory.read(indirect_addr),
                self.memory.read(indirect_addr & 0xff00),
            ])
        } else {
            self.memory.read_word(indirect_addr)
        }
    }

    pub(crate) fn indexed_indirect(&self) -> u16 {
        self.wrapping_read(self.zero_page_x())
    }

    pub(crate) fn indirect_indexed(&mut self) -> u16 {
        let base = self.wrapping_read(self.zero_page());
        let addr = base.wrapping_add(self.core.y as u16);

        if base & 0xff00 != addr & 0xff00 {
            self.cycles += 1
        }

        addr
    }

    // OPCODES
    pub(crate) fn adc(&mut self, addr: u16) {
        let old_a = self.core.a;
        let carry = self.core.f.c as u8;
        let operand = self.memory.read(addr);

        let sum = (old_a as u16) + (operand as u16) + (carry as u16);
        self.core.a = (sum & 0xff) as u8;
        self.core.f.c = sum > 0xff;

        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
        self.core.f.v = (old_a ^ operand) & 0x80 == 0 && (old_a ^ self.core.a) & 0x80 != 0;
    }

    pub(crate) fn and(&mut self, addr: u16) {
        self.core.a &= self.memory.read(addr);
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn asla(&mut self) {
        self.core.f.c = self.core.a & 0x80 != 0;
        self.core.a <<= 1;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn asl(&mut self, addr: u16) {
        let mut operand = self.memory.read(addr);
        self.core.f.c = operand & 0x80 != 0;
        operand <<= 1;
        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn bcc(&mut self, addr: u16) {
        if !self.core.f.c {
            self.branch(addr);
        }
    }

    pub(crate) fn bcs(&mut self, addr: u16) {
        if self.core.f.c {
            self.branch(addr);
        }
    }

    pub(crate) fn beq(&mut self, addr: u16) {
        if self.core.f.z {
            self.branch(addr);
        }
    }

    pub(crate) fn bit(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let anded = self.core.a & operand;

        self.core.f.z = anded == 0;
        self.core.f.v = operand & 0x40 != 0;
        self.core.f.n = operand & 0x80 != 0;
    }

    pub(crate) fn bmi(&mut self, addr: u16) {
        if self.core.f.n {
            self.branch(addr);
        }
    }

    pub(crate) fn bne(&mut self, addr: u16) {
        if !self.core.f.z {
            self.branch(addr);
        }
    }

    pub(crate) fn bpl(&mut self, addr: u16) {
        if !self.core.f.n {
            self.branch(addr);
        }
    }

    pub(crate) fn brk(&mut self) {
        let ret = self.core.pc + 2;
        self.push_word(ret);
        self.php();
        self.core.f.i = true;
        self.jmp(self.memory.read_word(0xfffe));
    }

    pub(crate) fn bvc(&mut self, addr: u16) {
        if !self.core.f.v {
            self.branch(addr);
        }
    }

    pub(crate) fn bvs(&mut self, addr: u16) {
        if self.core.f.v {
            self.branch(addr);
        }
    }

    pub(crate) fn clc(&mut self) {
        self.core.f.c = false;
    }

    pub(crate) fn cld(&mut self) {
        self.core.f.d = false;
    }

    pub(crate) fn cli(&mut self) {
        self.core.f.i = false;
    }

    pub(crate) fn clv(&mut self) {
        self.core.f.v = false;
    }

    pub(crate) fn cmp(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let diff = (self.core.a as i16) - (operand as i16);
        let result = (diff & 0xff) as u8;
        self.core.f.set_n(result);
        self.core.f.set_z(result);
        self.core.f.c = diff >= 0;
    }

    pub(crate) fn cpx(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let diff = (self.core.x as i16) - (operand as i16);
        let result = (diff & 0xff) as u8;
        self.core.f.set_n(result);
        self.core.f.set_z(result);
        self.core.f.c = diff >= 0;
    }

    pub(crate) fn cpy(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let diff = (self.core.y as i16) - (operand as i16);
        let result = (diff & 0xff) as u8;
        self.core.f.set_n(result);
        self.core.f.set_z(result);
        self.core.f.c = diff >= 0;
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn dcp(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        self.memory.write(addr, operand.wrapping_sub(1));
        let diff = (self.core.a as i16) - (operand as i16);
        let result = (diff & 0xff) as u8;
        self.core.f.set_n(result);
        self.core.f.set_z(result);
        self.core.f.c = diff >= 0;
    }

    pub(crate) fn dec(&mut self, addr: u16) {
        let mut operand = self.memory.read(addr);
        operand = operand.wrapping_sub(1);
        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn dex(&mut self) {
        self.core.x = self.core.x.wrapping_sub(1);
        self.core.f.set_z(self.core.x);
        self.core.f.set_n(self.core.x);
    }

    pub(crate) fn dey(&mut self) {
        self.core.y = self.core.y.wrapping_sub(1);
        self.core.f.set_z(self.core.y);
        self.core.f.set_n(self.core.y);
    }

    pub(crate) fn eor(&mut self, addr: u16) {
        self.core.a ^= self.memory.read(addr);
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn inc(&mut self, addr: u16) {
        let mut operand = self.memory.read(addr);
        operand = operand.wrapping_add(1);
        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn inx(&mut self) {
        self.core.x = self.core.x.wrapping_add(1);
        self.core.f.set_z(self.core.x);
        self.core.f.set_n(self.core.x);
    }

    pub(crate) fn iny(&mut self) {
        self.core.y = self.core.y.wrapping_add(1);
        self.core.f.set_z(self.core.y);
        self.core.f.set_n(self.core.y);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn isc(&mut self, addr: u16) {
        self.sbc(addr);
        let operand = self.memory.read(addr);
        self.memory.write(addr, operand.wrapping_sub(1));
    }

    pub(crate) fn jmp(&mut self, addr: u16) {
        self.core.pc = addr;
        self.jumped = true;
    }

    pub(crate) fn jsr(&mut self, addr: u16) {
        let ret = self.core.pc + 2;
        self.push_word(ret);
        self.jmp(addr);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn lax(&mut self, addr: u16) {
        let value = self.memory.read(addr);
        self.core.a = value;
        self.core.x = value;
        self.core.f.set_z(value);
        self.core.f.set_n(value);
    }

    pub(crate) fn lda(&mut self, addr: u16) {
        self.core.a = self.memory.read(addr);
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn ldx(&mut self, addr: u16) {
        self.core.x = self.memory.read(addr);
        self.core.f.set_z(self.core.x);
        self.core.f.set_n(self.core.x);
    }

    pub(crate) fn ldy(&mut self, addr: u16) {
        self.core.y = self.memory.read(addr);
        self.core.f.set_z(self.core.y);
        self.core.f.set_n(self.core.y);
    }

    pub(crate) fn lsr(&mut self, addr: u16) {
        let mut operand = self.memory.read(addr);
        self.core.f.c = operand & 0x01 != 0;
        operand >>= 1;
        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn lsra(&mut self) {
        self.core.f.c = self.core.a & 0x01 != 0;
        self.core.a >>= 1;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn nop(&mut self) {
        // do nothing
    }

    pub(crate) fn nop_addr(&mut self, _addr: u16) {
        // do nothing
    }

    pub(crate) fn ora(&mut self, addr: u16) {
        self.core.a |= self.memory.read(addr);
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn pha(&mut self) {
        self.push(self.core.a);
    }

    pub(crate) fn php(&mut self) {
        // PHP always sets bits 4 and 5
        self.push(self.core.f.get_byte() | 0x30);
    }

    pub(crate) fn pla(&mut self) {
        self.core.a = self.pull();
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn plp(&mut self) {
        let byte = self.pull();
        self.core.f.set_byte(byte);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn rla(&mut self, addr: u16) {
        let carry = self.core.f.c as u8;
        let mut operand = self.memory.read(addr);

        self.core.a &= operand;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);

        self.core.f.c = operand & 0x80 != 0;
        operand = (operand << 1) | carry;

        self.memory.write(addr, operand);
    }

    pub(crate) fn rol(&mut self, addr: u16) {
        let carry = self.core.f.c as u8;
        let mut operand = self.memory.read(addr);

        self.core.f.c = operand & 0x80 != 0;
        operand = (operand << 1) | carry;

        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn rola(&mut self) {
        let carry = self.core.f.c as u8;

        self.core.f.c = self.core.a & 0x80 != 0;
        self.core.a = (self.core.a << 1) | carry;

        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn ror(&mut self, addr: u16) {
        let carry = (self.core.f.c as u8) << 7;
        let mut operand = self.memory.read(addr);

        self.core.f.c = operand & 0x01 != 0;
        operand = (operand >> 1) | carry;

        self.memory.write(addr, operand);
        self.core.f.set_z(operand);
        self.core.f.set_n(operand);
    }

    pub(crate) fn rora(&mut self) {
        let carry = (self.core.f.c as u8) << 7;

        self.core.f.c = self.core.a & 0x01 != 0;
        self.core.a = (self.core.a >> 1) | carry;

        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn rra(&mut self, _addr: u16) {
        // TODO implement behaviour here
    }

    pub(crate) fn rti(&mut self) {
        self.plp();
        self.jumped = true;
        self.rts();
    }

    pub(crate) fn rts(&mut self) {
        let lob = self.pull();
        let hob = self.pull();
        self.core.pc = ((hob as u16) << 8) | (lob as u16);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn sax(&mut self, addr: u16) {
        let value = self.core.a & self.core.x;
        self.memory.write(addr, value);
    }

    pub(crate) fn sbc(&mut self, addr: u16) {
        let old_a = self.core.a;
        let carry = (!self.core.f.c) as u8;
        let operand = self.memory.read(addr);

        let diff = (old_a as u16)
            .wrapping_sub(operand as u16)
            .wrapping_sub(carry as u16);
        self.core.a = (diff & 0x0ff) as u8;
        self.core.f.c = diff <= 0x0ff;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
        self.core.f.v = (old_a ^ operand) & 0x80 != 0 && (old_a ^ self.core.a) & 0x80 != 0;
    }

    pub(crate) fn sec(&mut self) {
        self.core.f.c = true;
    }

    pub(crate) fn sed(&mut self) {
        self.core.f.d = true;
    }

    pub(crate) fn sei(&mut self) {
        self.core.f.i = true;
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn slo(&mut self, addr: u16) {
        let operand = self.memory.read(addr);

        self.core.f.c = operand & 0x80 != 0;
        self.memory.write(addr, operand << 1);

        self.core.a |= operand;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    // UNDOCUMENTED OPCODE
    pub(crate) fn sre(&mut self, addr: u16) {
        let operand = self.memory.read(addr);

        self.core.f.c = operand & 0x01 != 0;
        self.memory.write(addr, operand >> 1);

        self.core.a ^= operand;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn sta(&mut self, addr: u16) {
        self.memory.write(addr, self.core.a);
    }

    pub(crate) fn stx(&mut self, addr: u16) {
        self.memory.write(addr, self.core.x);
    }

    pub(crate) fn sty(&mut self, addr: u16) {
        self.memory.write(addr, self.core.y);
    }

    pub(crate) fn tax(&mut self) {
        self.core.x = self.core.a;
        self.core.f.set_z(self.core.x);
        self.core.f.set_n(self.core.x);
    }

    pub(crate) fn tay(&mut self) {
        self.core.y = self.core.a;
        self.core.f.set_z(self.core.y);
        self.core.f.set_n(self.core.y);
    }

    pub(crate) fn tsx(&mut self) {
        self.core.x = self.core.sp;
        self.core.f.set_z(self.core.x);
        self.core.f.set_n(self.core.x);
    }

    pub(crate) fn txa(&mut self) {
        self.core.a = self.core.x;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub(crate) fn txs(&mut self) {
        self.core.sp = self.core.x;
    }

    pub(crate) fn tya(&mut self) {
        self.core.a = self.core.y;
        self.core.f.set_z(self.core.a);
        self.core.f.set_n(self.core.a);
    }

    pub fn emulate_instruction(&mut self) {
        let opcode = self.memory.read(self.core.pc);

        let (length, cycles) = decode_6502!(opcode; self);
        self.cycles += cycles;
        if self.jumped {
            self.jumped = false;
        } else {
            self.core.pc += length;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::RandomAccessMemory;
    use crate::{alter_by, alter_default_by};

    pub fn new_processor() -> Processor<RandomAccessMemory> {
        Processor::with_memory(RandomAccessMemory::new(0x1000000))
    }

    #[test]
    fn test_core_new() {
        let core = Core::new();

        assert_eq!(core.a, 0);
        assert_eq!(core.x, 0);
        assert_eq!(core.y, 0);
        assert_eq!(core.f, Flags::default());
        assert_eq!(core.sp, 0xfd);
        assert_eq!(core.pc, 0x0000);
    }

    #[test]
    fn test_clc() {
        let mut cpu = new_processor();

        cpu.core.f.c = true;
        cpu.clc();
        assert_eq!(cpu.core, alter_default_by!(Core, f.c => false));

        cpu.clc();
        assert_eq!(cpu.core, alter_default_by!(Core, f.c => false));
    }

    #[test]
    fn test_cld() {
        let mut cpu = new_processor();

        cpu.core.f.d = true;
        cpu.cld();
        assert_eq!(cpu.core, alter_default_by!(Core, f.d => false));

        cpu.cld();
        assert_eq!(cpu.core, alter_default_by!(Core, f.d => false));
    }

    #[test]
    fn test_cli() {
        let mut cpu = new_processor();

        cpu.core.f.i = true;
        cpu.cli();
        assert_eq!(cpu.core, alter_default_by!(Core, f.i => false));

        cpu.cli();
        assert_eq!(cpu.core, alter_default_by!(Core, f.i => false));
    }

    #[test]
    fn test_cmp() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let val: u8 = 0x00;
        cpu.core.a = 0x80;
        cpu.memory.write(addr, val);
        cpu.cmp(addr);
        assert!(!cpu.core.f.z);
        assert!(cpu.core.f.n);
        assert!(cpu.core.f.c);
    }

    #[test]
    fn test_cpx() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let val: u8 = 0x00;
        cpu.core.x = 0x80;
        cpu.memory.write(addr, val);
        cpu.cpx(addr);
        assert!(!cpu.core.f.z);
        assert!(cpu.core.f.n);
        assert!(cpu.core.f.c);
    }

    #[test]
    fn test_cpy() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let val: u8 = 0x00;
        cpu.core.y = 0x80;
        cpu.memory.write(addr, val);
        cpu.cpy(addr);
        assert!(!cpu.core.f.z);
        assert!(cpu.core.f.n);
        assert!(cpu.core.f.c);
    }

    #[test]
    fn test_lda() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let mut val = 0x77;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, a => val));

        val = 0xf8;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, a => val, f.n => true));

        val = 0x00;
        cpu.memory.write(addr, val);
        cpu.lda(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, a => val, f.z => true));
    }

    #[test]
    fn test_ldx() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let mut val = 0x77;
        cpu.memory.write(addr, val);
        cpu.ldx(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, x => val));

        val = 0xf8;
        cpu.memory.write(addr, val);
        cpu.ldx(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, x => val, f.n => true));

        val = 0x00;
        cpu.memory.write(addr, val);
        cpu.ldx(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, x => val, f.z => true));
    }

    #[test]
    fn test_ldy() {
        let mut cpu = new_processor();
        let addr: u16 = 0x1000;

        let mut val = 0x77;
        cpu.memory.write(addr, val);
        cpu.ldy(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, y => val));

        val = 0xf8;
        cpu.memory.write(addr, val);
        cpu.ldy(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, y => val, f.n => true));

        val = 0x00;
        cpu.memory.write(addr, val);
        cpu.ldy(addr);
        assert_eq!(cpu.core, alter_default_by!(Core, y => val, f.z => true));
    }

    #[test]
    fn test_sta() {
        let mut cpu = new_processor();
        let addr: u16 = 0x2000;

        let val = 0x9f;
        cpu.core.a = val;

        cpu.sta(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_stx() {
        let mut cpu = new_processor();
        let addr: u16 = 0x2000;

        let val = 0xfc;
        cpu.core.x = val;

        cpu.stx(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_sty() {
        let mut cpu = new_processor();
        let addr: u16 = 0x2000;

        let val = 0x04;
        cpu.core.y = val;

        cpu.sty(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_tax() {
        let mut cpu = new_processor();

        let val = 0xff;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, x => val, f.n => true);

        cpu.tax();
        assert_eq!(expected, cpu.core);

        let val = 0x00;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, x => val, f.n => false, f.z => true);

        cpu.tax();
        assert_eq!(expected, cpu.core);

        let val = 0x54;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, x => val, f.z => false);

        cpu.tax();
        assert_eq!(expected, cpu.core);
    }

    #[test]
    fn test_tay() {
        let mut cpu = new_processor();

        let val = 0xff;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, y => val, f.n => true);

        cpu.tay();
        assert_eq!(expected, cpu.core);

        let val = 0x00;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, y => val, f.n => false, f.z => true);

        cpu.tay();
        assert_eq!(expected, cpu.core);

        let val = 0x54;
        cpu.core.a = val;
        let expected = alter_by!(cpu.core, y => val, f.z => false);

        cpu.tay();
        assert_eq!(expected, cpu.core);
    }

    #[test]
    fn test_txa() {
        let mut cpu = new_processor();

        let val = 0xff;
        cpu.core.x = val;
        let expected = alter_by!(cpu.core, a => val, f.n => true);

        cpu.txa();
        assert_eq!(expected, cpu.core);

        let val = 0x00;
        cpu.core.x = val;
        let expected = alter_by!(cpu.core, a => val, f.n => false, f.z => true);

        cpu.txa();
        assert_eq!(expected, cpu.core);

        let val = 0x54;
        cpu.core.x = val;
        let expected = alter_by!(cpu.core, a => val, f.z => false);

        cpu.txa();
        assert_eq!(expected, cpu.core);
    }
    #[test]
    fn test_tya() {
        let mut cpu = new_processor();

        let val = 0xff;
        cpu.core.y = val;
        let expected = alter_by!(cpu.core, a => val, f.n => true);

        cpu.tya();
        assert_eq!(expected, cpu.core);

        let val = 0x00;
        cpu.core.y = val;
        let expected = alter_by!(cpu.core, a => val, f.n => false, f.z => true);

        cpu.tya();
        assert_eq!(expected, cpu.core);

        let val = 0x54;
        cpu.core.y = val;
        let expected = alter_by!(cpu.core, a => val, f.z => false);

        cpu.tya();
        assert_eq!(expected, cpu.core);
    }
}
