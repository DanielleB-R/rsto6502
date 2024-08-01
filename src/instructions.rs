#[macro_export]
macro_rules! decode {
    (
        $opcode:expr;
        $self:ident,
        $($code:literal => ($operation:ident, $($addressing:ident)?)),+
    ) => {
        match $opcode {
            $($code => {
                $self.$operation($($self.$addressing())?);
                1 $( - 1 + $crate::instructions::length::$addressing)?
            })+
            _ => panic!("invalid opcode 0x{:x}", $opcode)
        }
    }
}

#[macro_export]
macro_rules! decode_6502 {
    ($opcode:expr; $self:ident) => {
        $crate::decode! {
            $opcode;
            $self,

            0x00 => (brk,),
            0x01 => (ora, indexed_indirect),
            0x03 => (slo, indexed_indirect),
            0x04 => (nop_addr, zero_page), // UNDOCUMENTED
            0x05 => (ora, zero_page),
            0x06 => (asl, zero_page),
            0x07 => (slo, zero_page),
            0x08 => (php,),
            0x09 => (ora, immediate),
            0x0a => (asla, ),
            0x0c => (nop_addr, absolute), // UNDOCUMENTED
            0x0d => (ora, absolute),
            0x0e => (asl, absolute),
            0x0f => (slo, absolute),

            0x10 => (bpl, immediate),
            0x11 => (ora, indirect_indexed),
            0x13 => (slo, indirect_indexed),
            0x14 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0x15 => (ora, zero_page_x),
            0x16 => (asl, zero_page_x),
            0x17 => (slo, zero_page_x),
            0x18 => (clc, ),
            0x19 => (ora, absolute_y),
            0x1a => (nop, ), // UNDOCUMENTED
            0x1b => (slo, absolute_y),
            0x1c => (nop_addr, absolute_x), // UNDOCUMENTED
            0x1d => (ora, absolute_x),
            0x1e => (asl, absolute_x),
            0x1f => (slo, absolute_x),

            0x20 => (jsr, absolute),
            0x21 => (and, indexed_indirect),
            0x23 => (rla, indexed_indirect),
            0x24 => (bit, zero_page),
            0x25 => (and, zero_page),
            0x26 => (rol, zero_page),
            0x27 => (rla, zero_page),
            0x28 => (plp, ),
            0x29 => (and, immediate),
            0x2a => (rola, ),
            0x2c => (bit, absolute),
            0x2d => (and, absolute),
            0x2e => (rol, absolute),
            0x2f => (rla, absolute),

            0x30 => (bmi, immediate),
            0x31 => (and, indirect_indexed),
            0x33 => (rla, indirect_indexed),
            0x34 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0x35 => (and, zero_page_x),
            0x36 => (rol, zero_page_x),
            0x37 => (rla, zero_page_x),
            0x38 => (sec, ),
            0x39 => (and, absolute_y),
            0x3a => (nop, ), // UNDOCUMENTED
            0x3b => (rla, absolute_y),
            0x3c => (nop_addr, absolute_x), // UNDOCUMENTED
            0x3d => (and, absolute_x),
            0x3e => (rol, absolute_x),
            0x3f => (rla, absolute_x),

            0x40 => (rti, ),
            0x41 => (eor, indexed_indirect),
            0x43 => (sre, indexed_indirect),
            0x44 => (nop_addr, zero_page), // UNDOCUMENTED
            0x45 => (eor, zero_page),
            0x46 => (lsr, zero_page),
            0x47 => (sre, zero_page),
            0x48 => (pha, ),
            0x49 => (eor, immediate),
            0x4a => (lsra, ),
            0x4c => (jmp, absolute),
            0x4d => (eor, absolute),
            0x4e => (lsr, absolute),
            0x4f => (sre, absolute),

            0x50 => (bvc, immediate),
            0x51 => (eor, indirect_indexed),
            0x53 => (sre, indirect_indexed),
            0x54 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0x55 => (eor, zero_page_x),
            0x56 => (lsr, zero_page_x),
            0x57 => (sre, zero_page_x),
            0x58 => (cli, ),
            0x59 => (eor, absolute_y),
            0x5a => (nop, ), // UNDOCUMENTED
            0x5b => (sre, absolute_y),
            0x5c => (nop_addr, absolute_x), // UNDOCUMENTED
            0x5d => (eor, absolute_x),
            0x5e => (lsr, absolute_x),
            0x5f => (sre, absolute_x),

            0x60 => (rts, ),
            0x61 => (adc, indexed_indirect),
            0x63 => (rra, indexed_indirect),
            0x64 => (nop_addr, zero_page), // UNDOCUMENTED
            0x65 => (adc, zero_page),
            0x66 => (ror, zero_page),
            0x67 => (rra, zero_page),
            0x68 => (pla,),
            0x69 => (adc, immediate),
            0x6a => (rora, ),
            0x6c => (jmp, indirect),
            0x6d => (adc, absolute),
            0x6e => (ror, absolute),
            0x6f => (rra, absolute),

            0x70 => (bvs, immediate),
            0x71 => (adc, indirect_indexed),
            0x73 => (rra, indirect_indexed),
            0x74 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0x75 => (adc, zero_page_x),
            0x76 => (ror, zero_page_x),
            0x77 => (rra, zero_page_x),
            0x78 => (sei, ),
            0x79 => (adc, absolute_y),
            0x7a => (nop, ), // UNDOCUMENTED
            0x7b => (rra, absolute_y),
            0x7c => (nop_addr, absolute_x), // UNDOCUMENTED
            0x7d => (adc, absolute_x),
            0x7e => (ror, absolute_x),
            0x7f => (rra, absolute_x),

            0x80 => (nop_addr, immediate), // UNDOCUMENTED
            0x81 => (sta, indexed_indirect),
            0x83 => (sax, indexed_indirect),
            0x84 => (sty, zero_page),
            0x85 => (sta, zero_page),
            0x86 => (stx, zero_page),
            0x87 => (sax, zero_page),
            0x88 => (dey, ),
            0x8a => (txa, ),
            0x8c => (sty, absolute),
            0x8d => (sta, absolute),
            0x8e => (stx, absolute),
            0x8f => (sax, absolute),

            0x90 => (bcc, immediate),
            0x91 => (sta, indirect_indexed),
            0x94 => (sty, zero_page_x),
            0x95 => (sta, zero_page_x),
            0x96 => (stx, zero_page_y),
            0x97 => (sax, zero_page_y),
            0x98 => (tya, ),
            0x99 => (sta, absolute_y),
            0x9a => (txs, ),
            0x9d => (sta, absolute_x),

            0xa0 => (ldy, immediate),
            0xa1 => (lda, indexed_indirect),
            0xa2 => (ldx, immediate),
            0xa3 => (lax, indexed_indirect),
            0xa4 => (ldy, zero_page),
            0xa5 => (lda, zero_page),
            0xa6 => (ldx, zero_page),
            0xa7 => (lax, zero_page),
            0xa8 => (tay, ),
            0xa9 => (lda, immediate),
            0xaa => (tax, ),
            0xac => (ldy, absolute),
            0xad => (lda, absolute),
            0xae => (ldx, absolute),
            0xaf => (lax, absolute),

            0xb0 => (bcs, immediate),
            0xb1 => (lda, indirect_indexed),
            0xb3 => (lax, indirect_indexed),
            0xb4 => (ldy, zero_page_x),
            0xb5 => (lda, zero_page_x),
            0xb6 => (ldx, zero_page_y),
            0xb7 => (lax, zero_page_y),
            0xb8 => (clv, ),
            0xb9 => (lda, absolute_y),
            0xba => (tsx, ),
            0xbc => (ldy, absolute_x),
            0xbd => (lda, absolute_x),
            0xbe => (ldx, absolute_y),
            0xbf => (lax, absolute_y),

            0xc0 => (cpy, immediate),
            0xc1 => (cmp, indexed_indirect),
            0xc3 => (dcp, indexed_indirect),
            0xc4 => (cpy, zero_page),
            0xc5 => (cmp, zero_page),
            0xc6 => (dec, zero_page),
            0xc7 => (dcp, zero_page),
            0xc8 => (iny, ),
            0xc9 => (cmp, immediate),
            0xca => (dex, ),
            0xcc => (cpy, absolute),
            0xcd => (cmp, absolute),
            0xce => (dec, absolute),
            0xcf => (dcp, absolute),

            0xd0 => (bne, immediate),
            0xd1 => (cmp, indirect_indexed),
            0xd3 => (dcp, indirect_indexed),
            0xd4 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0xd5 => (cmp, zero_page_x),
            0xd6 => (dec, zero_page_x),
            0xd7 => (dcp, zero_page_x),
            0xd8 => (cld, ),
            0xd9 => (cmp, absolute_y),
            0xda => (nop, ), // UNDOCUMENTED
            0xdb => (dcp, absolute_y),
            0xdc => (nop_addr, absolute_x), // UNDOCUMENTED
            0xdd => (cmp, absolute_x),
            0xde => (dec, absolute_x),
            0xdf => (dcp, absolute_x),

            0xe0 => (cpx, immediate),
            0xe1 => (sbc, indexed_indirect),
            0xe3 => (isc, indexed_indirect),
            0xe4 => (cpx, zero_page),
            0xe5 => (sbc, zero_page),
            0xe6 => (inc, zero_page),
            0xe7 => (isc, zero_page),
            0xe8 => (inx, ),
            0xe9 => (sbc, immediate),
            0xea => (nop, ),
            0xeb => (sbc, immediate), // UNDOCUMENTED
            0xec => (cpx, absolute),
            0xed => (sbc, absolute),
            0xee => (inc, absolute),
            0xef => (isc, absolute),

            0xf0 => (beq, immediate),
            0xf1 => (sbc, indirect_indexed),
            0xf3 => (isc, indirect_indexed),
            0xf4 => (nop_addr, zero_page_x), // UNDOCUMENTED
            0xf5 => (sbc, zero_page_x),
            0xf6 => (inc, zero_page_x),
            0xf7 => (isc, zero_page_x),
            0xf8 => (sed, ),
            0xf9 => (sbc, absolute_y),
            0xfa => (nop, ), // UNDOCUMENTED
            0xfb => (isc, absolute_y),
            0xfc => (nop_addr, absolute_x), // UNDOCUMENTED
            0xfd => (sbc, absolute_x),
            0xfe => (inc, absolute_x),
            0xff => (isc, absolute_x)
        }
    };
}

#[allow(non_upper_case_globals)]
pub mod length {
    pub const immediate: u16 = 2;
    pub const absolute: u16 = 3;
    pub const absolute_x: u16 = 3;
    pub const absolute_y: u16 = 3;
    pub const zero_page: u16 = 2;
    pub const zero_page_x: u16 = 2;
    pub const zero_page_y: u16 = 2;
    pub const indirect: u16 = 3;
    pub const indexed_indirect: u16 = 2;
    pub const indirect_indexed: u16 = 2;
}
