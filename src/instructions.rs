use crate::memory::Memory;
use crate::processor::Processor;

type NoOperandOperation = fn(&mut Processor);
type Operation = fn(&mut Processor, u16);

#[derive(Clone, Copy)]
pub enum Instruction {
    NoOperand(NoOperandOperation),
    Immediate(Operation),
    Absolute(Operation),
    AbsoluteX(Operation),
    AbsoluteY(Operation),
    ZeroPage(Operation),
    ZeroPageX(Operation),
    ZeroPageY(Operation),
    Indirect(Operation),
    IndexedIndirect(Operation),
    IndirectIndexed(Operation),
    Invalid,
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Instruction {
    pub fn length(&self) -> u16 {
        match self {
            Self::NoOperand(_) => 1,
            Self::Immediate(_) => 2,
            Self::Absolute(_) => 3,
            Self::AbsoluteX(_) => 3,
            Self::AbsoluteY(_) => 3,
            Self::ZeroPage(_) => 2,
            Self::ZeroPageX(_) => 2,
            Self::ZeroPageY(_) => 2,
            Self::Indirect(_) => 3,
            Self::IndexedIndirect(_) => 2,
            Self::IndirectIndexed(_) => 2,
            Self::Invalid => 1,
        }
    }

    pub fn apply(&self, cpu: &mut Processor) {
        match self {
            Self::NoOperand(op) => op(cpu),
            Self::Immediate(op) => op(cpu, cpu.immediate_address()),
            Self::Absolute(op) => op(cpu, cpu.absolute_address()),
            Self::AbsoluteX(op) => op(cpu, cpu.absolute_x_address()),
            Self::AbsoluteY(op) => op(cpu, cpu.absolute_y_address()),
            Self::ZeroPage(op) => op(cpu, cpu.zero_page_address()),
            Self::ZeroPageX(op) => op(cpu, cpu.zero_page_x_address()),
            Self::ZeroPageY(op) => op(cpu, cpu.zero_page_y_address()),
            Self::Indirect(op) => op(cpu, cpu.indirect_address()),
            Self::IndexedIndirect(op) => op(cpu, cpu.indexed_indirect_address()),
            Self::IndirectIndexed(op) => op(cpu, cpu.indirect_indexed_address()),
            Self::Invalid => panic!("invalid opcode"),
        }
    }
}

impl Processor {
    pub(crate) fn immediate_address(&self) -> u16 {
        self.core.pc + 1
    }

    pub(crate) fn immediate_operand(&self) -> u8 {
        self.memory.read(self.immediate_address())
    }

    pub(crate) fn zero_page_address(&self) -> u16 {
        self.immediate_operand() as u16
    }

    pub(crate) fn zero_page_x_address(&self) -> u16 {
        self.immediate_operand().wrapping_add(self.core.x) as u16
    }

    pub(crate) fn zero_page_y_address(&self) -> u16 {
        self.immediate_operand().wrapping_add(self.core.y) as u16
    }

    pub(crate) fn absolute_address(&self) -> u16 {
        self.memory.read_word(self.immediate_address())
    }

    pub(crate) fn absolute_x_address(&self) -> u16 {
        self.absolute_address().wrapping_add(self.core.x as u16)
    }

    pub(crate) fn absolute_y_address(&self) -> u16 {
        self.absolute_address().wrapping_add(self.core.y as u16)
    }

    pub(crate) fn indirect_address(&self) -> u16 {
        self.memory.read_word(self.absolute_address())
    }

    pub(crate) fn indexed_indirect_address(&self) -> u16 {
        self.memory.read_word(self.zero_page_x_address())
    }

    pub(crate) fn indirect_indexed_address(&self) -> u16 {
        self.memory
            .read_word(self.zero_page_address())
            .wrapping_add(self.core.y as u16)
    }

    pub(crate) fn build_instruction_table(&mut self) {
        self.instructions[0x00] = Instruction::NoOperand(Self::brk);
        self.instructions[0x01] = Instruction::IndexedIndirect(Self::ora);
        self.instructions[0x05] = Instruction::ZeroPage(Self::ora);
        self.instructions[0x06] = Instruction::ZeroPage(Self::asl);
        self.instructions[0x08] = Instruction::NoOperand(Self::php);
        self.instructions[0x09] = Instruction::Immediate(Self::ora);
        self.instructions[0x0a] = Instruction::NoOperand(Self::asla);
        self.instructions[0x0d] = Instruction::Absolute(Self::ora);
        self.instructions[0x0e] = Instruction::Absolute(Self::asl);

        self.instructions[0x10] = Instruction::Immediate(Self::bpl);
        self.instructions[0x11] = Instruction::IndirectIndexed(Self::ora);
        self.instructions[0x15] = Instruction::ZeroPageX(Self::ora);
        self.instructions[0x16] = Instruction::ZeroPageX(Self::asl);
        self.instructions[0x18] = Instruction::NoOperand(Self::clc);
        self.instructions[0x19] = Instruction::AbsoluteY(Self::ora);
        self.instructions[0x1d] = Instruction::AbsoluteX(Self::ora);
        self.instructions[0x1e] = Instruction::AbsoluteX(Self::asl);

        self.instructions[0x20] = Instruction::Absolute(Self::jsr);
        self.instructions[0x21] = Instruction::IndexedIndirect(Self::and);
        self.instructions[0x24] = Instruction::ZeroPage(Self::bit);
        self.instructions[0x25] = Instruction::ZeroPage(Self::and);
        self.instructions[0x26] = Instruction::ZeroPage(Self::rol);
        self.instructions[0x28] = Instruction::NoOperand(Self::plp);
        self.instructions[0x29] = Instruction::Immediate(Self::and);
        self.instructions[0x2a] = Instruction::NoOperand(Self::rola);
        self.instructions[0x2c] = Instruction::Absolute(Self::bit);
        self.instructions[0x2d] = Instruction::Absolute(Self::and);
        self.instructions[0x2e] = Instruction::Absolute(Self::rol);

        self.instructions[0x30] = Instruction::Immediate(Self::bmi);
        self.instructions[0x31] = Instruction::IndexedIndirect(Self::and);
        self.instructions[0x35] = Instruction::ZeroPageX(Self::and);
        self.instructions[0x36] = Instruction::ZeroPageX(Self::rol);
        self.instructions[0x38] = Instruction::NoOperand(Self::sec);
        self.instructions[0x39] = Instruction::AbsoluteY(Self::and);
        self.instructions[0x3d] = Instruction::AbsoluteX(Self::and);
        self.instructions[0x3e] = Instruction::AbsoluteX(Self::rol);

        self.instructions[0x40] = Instruction::NoOperand(Self::rti);
        self.instructions[0x41] = Instruction::IndexedIndirect(Self::eor);
        self.instructions[0x45] = Instruction::ZeroPage(Self::eor);
        self.instructions[0x46] = Instruction::ZeroPage(Self::lsr);
        self.instructions[0x48] = Instruction::NoOperand(Self::pha);
        self.instructions[0x49] = Instruction::Immediate(Self::eor);
        self.instructions[0x4a] = Instruction::NoOperand(Self::lsra);
        self.instructions[0x4c] = Instruction::Absolute(Self::jmp);
        self.instructions[0x4d] = Instruction::Absolute(Self::eor);
        self.instructions[0x4e] = Instruction::Absolute(Self::lsr);

        self.instructions[0x50] = Instruction::Immediate(Self::bvc);
        self.instructions[0x51] = Instruction::IndirectIndexed(Self::eor);
        self.instructions[0x55] = Instruction::ZeroPageX(Self::eor);
        self.instructions[0x56] = Instruction::ZeroPageX(Self::lsr);
        self.instructions[0x58] = Instruction::NoOperand(Self::cli);
        self.instructions[0x59] = Instruction::AbsoluteY(Self::eor);
        self.instructions[0x5d] = Instruction::AbsoluteX(Self::eor);
        self.instructions[0x5e] = Instruction::AbsoluteX(Self::lsr);

        self.instructions[0x60] = Instruction::NoOperand(Self::rts);
        self.instructions[0x61] = Instruction::IndexedIndirect(Self::adc);
        self.instructions[0x65] = Instruction::ZeroPage(Self::adc);
        self.instructions[0x66] = Instruction::ZeroPage(Self::ror);
        self.instructions[0x68] = Instruction::NoOperand(Self::pla);
        self.instructions[0x69] = Instruction::Immediate(Self::adc);
        self.instructions[0x6a] = Instruction::NoOperand(Self::rora);
        self.instructions[0x6c] = Instruction::Indirect(Self::jmp);
        self.instructions[0x6d] = Instruction::Absolute(Self::adc);
        self.instructions[0x6e] = Instruction::Absolute(Self::ror);

        self.instructions[0x70] = Instruction::Immediate(Self::bvs);
        self.instructions[0x71] = Instruction::IndirectIndexed(Self::adc);
        self.instructions[0x75] = Instruction::ZeroPageX(Self::adc);
        self.instructions[0x76] = Instruction::ZeroPageX(Self::ror);
        self.instructions[0x78] = Instruction::NoOperand(Self::sei);
        self.instructions[0x79] = Instruction::AbsoluteY(Self::adc);
        self.instructions[0x7d] = Instruction::AbsoluteX(Self::adc);
        self.instructions[0x7e] = Instruction::AbsoluteX(Self::ror);

        self.instructions[0x81] = Instruction::IndexedIndirect(Self::sta);
        self.instructions[0x84] = Instruction::ZeroPage(Self::sty);
        self.instructions[0x85] = Instruction::ZeroPage(Self::sta);
        self.instructions[0x86] = Instruction::ZeroPage(Self::stx);
        self.instructions[0x88] = Instruction::NoOperand(Self::dey);
        self.instructions[0x8a] = Instruction::NoOperand(Self::txa);
        self.instructions[0x8c] = Instruction::Absolute(Self::sty);
        self.instructions[0x8d] = Instruction::Absolute(Self::sta);
        self.instructions[0x8e] = Instruction::Absolute(Self::stx);

        self.instructions[0x90] = Instruction::Immediate(Self::bcc);
        self.instructions[0x91] = Instruction::IndirectIndexed(Self::sta);
        self.instructions[0x94] = Instruction::ZeroPageX(Self::sty);
        self.instructions[0x95] = Instruction::ZeroPageX(Self::sta);
        self.instructions[0x96] = Instruction::ZeroPageY(Self::stx);
        self.instructions[0x98] = Instruction::NoOperand(Self::tya);
        self.instructions[0x99] = Instruction::AbsoluteY(Self::sta);
        self.instructions[0x9a] = Instruction::NoOperand(Self::txs);
        self.instructions[0x9d] = Instruction::AbsoluteX(Self::sta);

        self.instructions[0xa0] = Instruction::Immediate(Self::ldy);
        self.instructions[0xa1] = Instruction::IndexedIndirect(Self::lda);
        self.instructions[0xa2] = Instruction::Immediate(Self::ldx);
        self.instructions[0xa4] = Instruction::ZeroPage(Self::ldy);
        self.instructions[0xa5] = Instruction::ZeroPage(Self::lda);
        self.instructions[0xa6] = Instruction::ZeroPage(Self::ldx);
        self.instructions[0xa8] = Instruction::NoOperand(Self::tay);
        self.instructions[0xa9] = Instruction::Immediate(Self::lda);
        self.instructions[0xaa] = Instruction::NoOperand(Self::tax);
        self.instructions[0xac] = Instruction::Absolute(Self::ldy);
        self.instructions[0xad] = Instruction::Absolute(Self::lda);
        self.instructions[0xae] = Instruction::Absolute(Self::ldx);

        self.instructions[0xb0] = Instruction::Immediate(Self::bcs);
        self.instructions[0xb1] = Instruction::IndirectIndexed(Self::lda);
        self.instructions[0xb4] = Instruction::ZeroPageX(Self::ldy);
        self.instructions[0xb5] = Instruction::ZeroPageX(Self::lda);
        self.instructions[0xb6] = Instruction::ZeroPageY(Self::ldx);
        self.instructions[0xb8] = Instruction::NoOperand(Self::clv);
        self.instructions[0xb9] = Instruction::AbsoluteY(Self::lda);
        self.instructions[0xba] = Instruction::NoOperand(Self::tsx);
        self.instructions[0xbc] = Instruction::AbsoluteX(Self::ldy);
        self.instructions[0xbd] = Instruction::AbsoluteX(Self::lda);
        self.instructions[0xbe] = Instruction::AbsoluteY(Self::ldx);

        self.instructions[0xc0] = Instruction::Immediate(Self::cpy);
        self.instructions[0xc1] = Instruction::IndexedIndirect(Self::cmp);
        self.instructions[0xc4] = Instruction::ZeroPage(Self::cpy);
        self.instructions[0xc5] = Instruction::ZeroPage(Self::cmp);
        self.instructions[0xc6] = Instruction::ZeroPage(Self::dec);
        self.instructions[0xc8] = Instruction::NoOperand(Self::iny);
        self.instructions[0xc9] = Instruction::Immediate(Self::cmp);
        self.instructions[0xca] = Instruction::NoOperand(Self::dex);
        self.instructions[0xcc] = Instruction::Absolute(Self::cpy);
        self.instructions[0xcd] = Instruction::Absolute(Self::cmp);
        self.instructions[0xce] = Instruction::Absolute(Self::dec);

        self.instructions[0xd0] = Instruction::Immediate(Self::bne);
        self.instructions[0xd1] = Instruction::IndirectIndexed(Self::cmp);
        self.instructions[0xd5] = Instruction::ZeroPageX(Self::cmp);
        self.instructions[0xd6] = Instruction::ZeroPageX(Self::dec);
        self.instructions[0xd8] = Instruction::NoOperand(Self::cld);
        self.instructions[0xd9] = Instruction::AbsoluteY(Self::cmp);
        self.instructions[0xdd] = Instruction::AbsoluteX(Self::cmp);
        self.instructions[0xde] = Instruction::AbsoluteX(Self::dec);

        self.instructions[0xe0] = Instruction::Immediate(Self::cpx);
        self.instructions[0xe1] = Instruction::IndexedIndirect(Self::sbc);
        self.instructions[0xe4] = Instruction::ZeroPage(Self::cpx);
        self.instructions[0xe5] = Instruction::ZeroPage(Self::sbc);
        self.instructions[0xe6] = Instruction::ZeroPage(Self::inc);
        self.instructions[0xe8] = Instruction::NoOperand(Self::inx);
        self.instructions[0xe9] = Instruction::Immediate(Self::sbc);
        self.instructions[0xea] = Instruction::NoOperand(Self::nop);
        self.instructions[0xec] = Instruction::Absolute(Self::cpx);
        self.instructions[0xed] = Instruction::Absolute(Self::sbc);
        self.instructions[0xee] = Instruction::Absolute(Self::inc);

        self.instructions[0xf0] = Instruction::Immediate(Self::beq);
        self.instructions[0xf1] = Instruction::IndirectIndexed(Self::sbc);
        self.instructions[0xf5] = Instruction::ZeroPageX(Self::sbc);
        self.instructions[0xf6] = Instruction::ZeroPageX(Self::inc);
        self.instructions[0xf8] = Instruction::NoOperand(Self::sed);
        self.instructions[0xf9] = Instruction::AbsoluteY(Self::sbc);
        self.instructions[0xfd] = Instruction::AbsoluteX(Self::sbc);
        self.instructions[0xfe] = Instruction::AbsoluteX(Self::inc);
    }
}
