use crate::decode_6502;

macro_rules! decompile_mnemonic {
    ($mnemonic:ident) => {
        pub(crate) fn $mnemonic(&mut self) {
            println!("{}", stringify!($mnemonic));
        }
    };
    ($mnemonic:ident, A) => {
        pub(crate) fn $mnemonic(&mut self) {
            println!("{} A", stringify!($mnemonic));
        }
    };
    ($mnemonic:ident, arg) => {
        pub(crate) fn $mnemonic(&mut self, arg: String) {
            println!("{} {}", stringify!($mnemonic), arg);
        }
    };
}

#[derive(Debug, Clone)]
pub struct Decompiler<'a> {
    pc: usize,
    program: &'a [u8],
}

impl<'a> Decompiler<'a> {
    pub fn new(program: &'a [u8]) -> Self {
        Self { pc: 0, program }
    }

    fn arg_u8(&self) -> u8 {
        self.program[self.pc + 1]
    }

    fn arg_u16(&self) -> u16 {
        u16::from_le_bytes([self.program[self.pc + 1], self.program[self.pc + 2]])
    }

    pub(crate) fn immediate(&self) -> String {
        format!("#${:02x}", self.arg_u8())
    }

    pub(crate) fn zero_page(&self) -> String {
        format!("${:02x}", self.arg_u8())
    }

    pub(crate) fn zero_page_x(&self) -> String {
        format!("${:02x},X", self.arg_u8())
    }

    pub(crate) fn zero_page_y(&self) -> String {
        format!("${:02x},Y", self.arg_u8())
    }

    pub(crate) fn absolute(&self) -> String {
        format!("${:04x}", self.arg_u16())
    }

    pub(crate) fn absolute_x(&self) -> String {
        format!("${:04x},X", self.arg_u16())
    }

    pub(crate) fn absolute_y(&self) -> String {
        format!("${:04x},Y", self.arg_u16())
    }

    pub(crate) fn absolute_x_for_store(&self) -> String {
        format!("${:04x},X", self.arg_u16())
    }

    pub(crate) fn absolute_y_for_store(&self) -> String {
        format!("${:04x},Y", self.arg_u16())
    }

    pub(crate) fn indirect(&self) -> String {
        format!("(${:04x})", self.arg_u16())
    }

    pub(crate) fn indexed_indirect(&self) -> String {
        format!("(${:02x},X)", self.arg_u8())
    }

    pub(crate) fn indirect_indexed(&self) -> String {
        format!("(${:02x}),Y", self.arg_u8())
    }

    pub(crate) fn indirect_indexed_for_store(&self) -> String {
        format!("(${:02x}),Y", self.arg_u8())
    }

    decompile_mnemonic!(adc, arg);
    decompile_mnemonic!(and, arg);
    decompile_mnemonic!(asla, A);
    decompile_mnemonic!(asl, arg);
    decompile_mnemonic!(bcc, arg);
    decompile_mnemonic!(bcs, arg);
    decompile_mnemonic!(beq, arg);
    decompile_mnemonic!(bit, arg);
    decompile_mnemonic!(bmi, arg);
    decompile_mnemonic!(bne, arg);
    decompile_mnemonic!(bpl, arg);
    decompile_mnemonic!(brk);
    decompile_mnemonic!(bvc, arg);
    decompile_mnemonic!(bvs, arg);
    decompile_mnemonic!(clc);
    decompile_mnemonic!(cld);
    decompile_mnemonic!(cli);
    decompile_mnemonic!(clv);
    decompile_mnemonic!(cmp, arg);
    decompile_mnemonic!(cpx, arg);
    decompile_mnemonic!(cpy, arg);
    decompile_mnemonic!(dcp, arg);
    decompile_mnemonic!(dec, arg);
    decompile_mnemonic!(dex);
    decompile_mnemonic!(dey);
    decompile_mnemonic!(eor, arg);
    decompile_mnemonic!(inc, arg);
    decompile_mnemonic!(inx);
    decompile_mnemonic!(iny);
    decompile_mnemonic!(isc, arg);
    decompile_mnemonic!(jmp, arg);
    decompile_mnemonic!(jsr, arg);
    decompile_mnemonic!(lax, arg);
    decompile_mnemonic!(lda, arg);
    decompile_mnemonic!(ldx, arg);
    decompile_mnemonic!(ldy, arg);
    decompile_mnemonic!(lsr, arg);
    decompile_mnemonic!(lsra, A);
    decompile_mnemonic!(nop);
    decompile_mnemonic!(nop_addr, arg);
    decompile_mnemonic!(ora, arg);
    decompile_mnemonic!(pha);
    decompile_mnemonic!(php);
    decompile_mnemonic!(pla);
    decompile_mnemonic!(plp);
    decompile_mnemonic!(rla, arg);
    decompile_mnemonic!(rol, arg);
    decompile_mnemonic!(rola, A);
    decompile_mnemonic!(ror, arg);
    decompile_mnemonic!(rora, A);
    decompile_mnemonic!(rra, arg);
    decompile_mnemonic!(rti);
    decompile_mnemonic!(rts);
    decompile_mnemonic!(sax, arg);
    decompile_mnemonic!(sbc, arg);
    decompile_mnemonic!(sec);
    decompile_mnemonic!(sed);
    decompile_mnemonic!(sei);
    decompile_mnemonic!(slo, arg);
    decompile_mnemonic!(sre, arg);
    decompile_mnemonic!(sta, arg);
    decompile_mnemonic!(stx, arg);
    decompile_mnemonic!(sty, arg);
    decompile_mnemonic!(tax);
    decompile_mnemonic!(tay);
    decompile_mnemonic!(tsx);
    decompile_mnemonic!(txa);
    decompile_mnemonic!(txs);
    decompile_mnemonic!(tya);

    pub fn decompile(mut self) {
        while self.pc < self.program.len() {
            self.pc += decode_6502!(self.program[self.pc]; self).0 as usize;
        }
    }
}
