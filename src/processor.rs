use crate::flags::Flags;
use crate::instructions::Instruction;
use crate::memory::{Memory, RandomAccessMemory};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Core {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub f: Flags,
    pub sp: u8,
    pub pc: u16,
}

#[derive(Clone)]
pub struct Processor {
    pub core: Core,
    pub memory: RandomAccessMemory,
    // TODO: Is there a better way to do this?
    pub jumped: bool,
    pub(crate) instructions: Vec<Instruction>,
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
    pub fn new() -> Self {
        Self::with_memory(RandomAccessMemory::new(0xffff))
    }

    pub fn with_memory(memory: RandomAccessMemory) -> Self {
        let mut processor = Processor {
            core: Core::new(),
            memory,
            jumped: false,
            instructions: vec![Default::default(); 256],
        };
        processor.build_instruction_table();

        processor
    }

    fn branch(&mut self, addr: u16) {
        let offset = self.memory.read_signed(addr);
        self.core.pc = self.core.pc.wrapping_add(offset as u16);
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

    pub(crate) fn adc(&mut self, addr: u16) {
        let old_a = self.core.a;
        let mut carry = self.core.f.c as u8;
        let operand = self.memory.read(addr);

        if self.core.f.d {
            let mut lnr = (old_a & 0x0f) + (operand & 0x0f) + carry;
            if lnr > 0x09 {
                lnr -= 0x0a;
                carry = 0x01;
            } else {
                carry = 0x00;
            }

            let mut hnr = (old_a >> 4) + (operand >> 4) + carry;
            if hnr > 0x09 {
                hnr -= 0x0a;
                self.core.f.c = true;
            } else {
                self.core.f.c = false;
            }
            self.core.a = (hnr << 4) + (lnr & 0x0f);
        } else {
            let sum = (old_a as u16) + (operand as u16) + (carry as u16);
            self.core.a = (sum & 0xff) as u8;
            self.core.f.c = sum > 0xff;
        }

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
        self.core.f.n = diff < 0;
        self.core.f.z = diff == 0;
        self.core.f.c = diff >= 0;
    }

    pub(crate) fn cpx(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let diff = (self.core.x as i16) - (operand as i16);
        self.core.f.n = diff < 0;
        self.core.f.z = diff == 0;
        self.core.f.c = diff >= 0;
    }

    pub(crate) fn cpy(&mut self, addr: u16) {
        let operand = self.memory.read(addr);
        let diff = (self.core.y as i16) - (operand as i16);
        self.core.f.n = diff < 0;
        self.core.f.z = diff == 0;
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

    pub(crate) fn jmp(&mut self, addr: u16) {
        self.core.pc = addr;
        self.jumped = true;
    }

    pub(crate) fn jsr(&mut self, addr: u16) {
        let ret = self.core.pc + 2;
        self.push_word(ret);
        self.jmp(addr);
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

    pub(crate) fn sbc(&mut self, addr: u16) {
        let old_a = self.core.a;
        let mut carry = (!self.core.f.c) as u8;
        let operand = self.memory.read(addr);

        if self.core.f.d {
            let mut lnr = (old_a & 0x0f)
                .wrapping_sub(operand & 0x0f)
                .wrapping_sub(carry);
            if lnr > 0x09 {
                lnr -= 0x06;
                carry = 0x01;
            } else {
                carry = 0x00;
            }

            let mut hnr = (old_a >> 4).wrapping_sub(operand >> 4).wrapping_sub(carry);
            if hnr > 0x09 {
                hnr -= 0x06;
                self.core.f.c = false;
            } else {
                self.core.f.c = true;
            }
            self.core.a = (hnr << 4) + (lnr & 0x0f);
        } else {
            let diff = (old_a as u16)
                .wrapping_sub(operand as u16)
                .wrapping_sub(carry as u16);
            self.core.a = (diff & 0x0ff) as u8;
            self.core.f.c = diff <= 0x0ff;
            self.core.f.set_z(self.core.a);
            self.core.f.set_n(self.core.a);
            self.core.f.v = (old_a ^ operand) & 0x80 != 0 && (old_a ^ self.core.a) & 0x80 != 0;
        }
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

        let instruction = self.instructions[opcode as usize];
        instruction.apply(self);
        if self.jumped {
            self.jumped = false;
        } else {
            self.core.pc += instruction.length();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{alter_by, alter_default_by};

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
        let mut cpu = Processor::new();
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
        let mut cpu = Processor::new();
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
        let mut cpu = Processor::new();
        let addr: u16 = 0x2000;

        let val = 0x9f;
        cpu.core.a = val;

        cpu.sta(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_stx() {
        let mut cpu = Processor::new();
        let addr: u16 = 0x2000;

        let val = 0xfc;
        cpu.core.x = val;

        cpu.stx(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_sty() {
        let mut cpu = Processor::new();
        let addr: u16 = 0x2000;

        let val = 0x04;
        cpu.core.y = val;

        cpu.sty(addr);
        assert_eq!(val, cpu.memory.contents[addr as usize]);
    }

    #[test]
    fn test_tax() {
        let mut cpu = Processor::new();

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
        let mut cpu = Processor::new();

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
        let mut cpu = Processor::new();

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
        let mut cpu = Processor::new();

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
