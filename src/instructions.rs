#[macro_export]
macro_rules! decode {
    (
        $opcode:expr;
        $self:ident,
        $($code:literal => ($cycles:literal, $operation:ident, $($addressing:ident)?)),+
    ) => {
        match $opcode {
            $($code => {
                $(let $addressing = $self.$addressing();)?
                $self.$operation($($addressing)?);
                let length = 1 $( - 1 + $crate::instructions::length::$addressing)?;
                (length, $cycles)
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

            0x00 => (7, brk,),
            0x01 => (6, ora, indexed_indirect),
            0x03 => (8, slo, indexed_indirect),
            0x04 => (3, nop_addr, zero_page), // UNDOCUMENTED
            0x05 => (3, ora, zero_page),
            0x06 => (5, asl, zero_page),
            0x07 => (5, slo, zero_page),
            0x08 => (3, php,),
            0x09 => (2, ora, immediate),
            0x0a => (2, asla, ),
            0x0c => (4, nop_addr, absolute), // UNDOCUMENTED
            0x0d => (4, ora, absolute),
            0x0e => (6, asl, absolute),
            0x0f => (6, slo, absolute),

            0x10 => (2, bpl, immediate),
            0x11 => (5, ora, indirect_indexed),
            0x13 => (8, slo, indirect_indexed_for_store),
            0x14 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0x15 => (4, ora, zero_page_x),
            0x16 => (6, asl, zero_page_x),
            0x17 => (6, slo, zero_page_x),
            0x18 => (2, clc, ),
            0x19 => (4, ora, absolute_y),
            0x1a => (2, nop, ), // UNDOCUMENTED
            0x1b => (7, slo, absolute_y_for_store),
            0x1c => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0x1d => (4, ora, absolute_x),
            0x1e => (7, asl, absolute_x),
            0x1f => (7, slo, absolute_x_for_store),

            0x20 => (6, jsr, absolute),
            0x21 => (6, and, indexed_indirect),
            0x23 => (8, rla, indexed_indirect),
            0x24 => (3, bit, zero_page),
            0x25 => (3, and, zero_page),
            0x26 => (5, rol, zero_page),
            0x27 => (5, rla, zero_page),
            0x28 => (4, plp, ),
            0x29 => (2, and, immediate),
            0x2a => (2, rola, ),
            0x2c => (4, bit, absolute),
            0x2d => (4, and, absolute),
            0x2e => (6, rol, absolute),
            0x2f => (6, rla, absolute),

            0x30 => (2, bmi, immediate),
            0x31 => (5, and, indirect_indexed),
            0x33 => (8, rla, indirect_indexed_for_store),
            0x34 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0x35 => (4, and, zero_page_x),
            0x36 => (6, rol, zero_page_x),
            0x37 => (6, rla, zero_page_x),
            0x38 => (2, sec, ),
            0x39 => (4, and, absolute_y),
            0x3a => (2, nop, ), // UNDOCUMENTED
            0x3b => (7, rla, absolute_y_for_store),
            0x3c => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0x3d => (4, and, absolute_x),
            0x3e => (7, rol, absolute_x),
            0x3f => (7, rla, absolute_x_for_store),

            0x40 => (6, rti, ),
            0x41 => (6, eor, indexed_indirect),
            0x43 => (8, sre, indexed_indirect),
            0x44 => (3, nop_addr, zero_page), // UNDOCUMENTED
            0x45 => (3, eor, zero_page),
            0x46 => (5, lsr, zero_page),
            0x47 => (5, sre, zero_page),
            0x48 => (3, pha, ),
            0x49 => (2, eor, immediate),
            0x4a => (2, lsra, ),
            0x4c => (3, jmp, absolute),
            0x4d => (4, eor, absolute),
            0x4e => (6, lsr, absolute),
            0x4f => (6, sre, absolute),

            0x50 => (2, bvc, immediate),
            0x51 => (5, eor, indirect_indexed),
            0x53 => (8, sre, indirect_indexed),
            0x54 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0x55 => (4, eor, zero_page_x),
            0x56 => (6, lsr, zero_page_x),
            0x57 => (6, sre, zero_page_x),
            0x58 => (2, cli, ),
            0x59 => (4, eor, absolute_y),
            0x5a => (2, nop, ), // UNDOCUMENTED
            0x5b => (7, sre, absolute_y),
            0x5c => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0x5d => (4, eor, absolute_x),
            0x5e => (7, lsr, absolute_x),
            0x5f => (7, sre, absolute_x),

            0x60 => (6, rts, ),
            0x61 => (6, adc, indexed_indirect),
            0x63 => (8, rra, indexed_indirect),
            0x64 => (3, nop_addr, zero_page), // UNDOCUMENTED
            0x65 => (3, adc, zero_page),
            0x66 => (5, ror, zero_page),
            0x67 => (5, rra, zero_page),
            0x68 => (4, pla,),
            0x69 => (2, adc, immediate),
            0x6a => (2, rora, ),
            0x6c => (5, jmp, indirect),
            0x6d => (4, adc, absolute),
            0x6e => (6, ror, absolute),
            0x6f => (6, rra, absolute),

            0x70 => (2, bvs, immediate),
            0x71 => (5, adc, indirect_indexed),
            0x73 => (8, rra, indirect_indexed),
            0x74 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0x75 => (4, adc, zero_page_x),
            0x76 => (6, ror, zero_page_x),
            0x77 => (6, rra, zero_page_x),
            0x78 => (2, sei, ),
            0x79 => (4, adc, absolute_y),
            0x7a => (2, nop, ), // UNDOCUMENTED
            0x7b => (7, rra, absolute_y),
            0x7c => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0x7d => (4, adc, absolute_x),
            0x7e => (7, ror, absolute_x),
            0x7f => (7, rra, absolute_x),

            0x80 => (2, nop_addr, immediate), // UNDOCUMENTED
            0x81 => (6, sta, indexed_indirect),
            0x83 => (6, sax, indexed_indirect),
            0x84 => (3, sty, zero_page),
            0x85 => (3, sta, zero_page),
            0x86 => (3, stx, zero_page),
            0x87 => (3, sax, zero_page),
            0x88 => (2, dey, ),
            0x8a => (2, txa, ),
            0x8c => (4, sty, absolute),
            0x8d => (4, sta, absolute),
            0x8e => (4, stx, absolute),
            0x8f => (4, sax, absolute),

            0x90 => (2, bcc, immediate),
            0x91 => (6, sta, indirect_indexed),
            0x94 => (4, sty, zero_page_x),
            0x95 => (4, sta, zero_page_x),
            0x96 => (4, stx, zero_page_y),
            0x97 => (4, sax, zero_page_y),
            0x98 => (2, tya, ),
            0x99 => (5, sta, absolute_y_for_store),
            0x9a => (2, txs, ),
            0x9d => (5, sta, absolute_x_for_store),

            0xa0 => (2, ldy, immediate),
            0xa1 => (6, lda, indexed_indirect),
            0xa2 => (2, ldx, immediate),
            0xa3 => (6, lax, indexed_indirect),
            0xa4 => (3, ldy, zero_page),
            0xa5 => (3, lda, zero_page),
            0xa6 => (3, ldx, zero_page),
            0xa7 => (3, lax, zero_page),
            0xa8 => (2, tay, ),
            0xa9 => (2, lda, immediate),
            0xaa => (2, tax, ),
            0xac => (4, ldy, absolute),
            0xad => (4, lda, absolute),
            0xae => (4, ldx, absolute),
            0xaf => (4, lax, absolute),

            0xb0 => (2, bcs, immediate),
            0xb1 => (5, lda, indirect_indexed),
            0xb3 => (5, lax, indirect_indexed),
            0xb4 => (4, ldy, zero_page_x),
            0xb5 => (4, lda, zero_page_x),
            0xb6 => (4, ldx, zero_page_y),
            0xb7 => (4, lax, zero_page_y),
            0xb8 => (2, clv, ),
            0xb9 => (4, lda, absolute_y),
            0xba => (2, tsx, ),
            0xbc => (4, ldy, absolute_x),
            0xbd => (4, lda, absolute_x),
            0xbe => (4, ldx, absolute_y),
            0xbf => (4, lax, absolute_y),

            0xc0 => (2, cpy, immediate),
            0xc1 => (6, cmp, indexed_indirect),
            0xc3 => (8, dcp, indexed_indirect),
            0xc4 => (3, cpy, zero_page),
            0xc5 => (3, cmp, zero_page),
            0xc6 => (5, dec, zero_page),
            0xc7 => (5, dcp, zero_page),
            0xc8 => (2, iny, ),
            0xc9 => (2, cmp, immediate),
            0xca => (2, dex, ),
            0xcc => (4, cpy, absolute),
            0xcd => (4, cmp, absolute),
            0xce => (6, dec, absolute),
            0xcf => (6, dcp, absolute),

            0xd0 => (2, bne, immediate),
            0xd1 => (5, cmp, indirect_indexed),
            0xd3 => (8, dcp, indirect_indexed_for_store),
            0xd4 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0xd5 => (4, cmp, zero_page_x),
            0xd6 => (6, dec, zero_page_x),
            0xd7 => (6, dcp, zero_page_x),
            0xd8 => (2, cld, ),
            0xd9 => (4, cmp, absolute_y),
            0xda => (2, nop, ), // UNDOCUMENTED
            0xdb => (7, dcp, absolute_y_for_store),
            0xdc => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0xdd => (4, cmp, absolute_x),
            0xde => (7, dec, absolute_x),
            0xdf => (7, dcp, absolute_x_for_store),

            0xe0 => (2, cpx, immediate),
            0xe1 => (6, sbc, indexed_indirect),
            0xe3 => (8, isc, indexed_indirect),
            0xe4 => (3, cpx, zero_page),
            0xe5 => (3, sbc, zero_page),
            0xe6 => (5, inc, zero_page),
            0xe7 => (5, isc, zero_page),
            0xe8 => (2, inx, ),
            0xe9 => (2, sbc, immediate),
            0xea => (2, nop, ),
            0xeb => (2, sbc, immediate), // UNDOCUMENTED
            0xec => (4, cpx, absolute),
            0xed => (4, sbc, absolute),
            0xee => (6, inc, absolute),
            0xef => (6, isc, absolute),

            0xf0 => (2, beq, immediate),
            0xf1 => (5, sbc, indirect_indexed),
            0xf3 => (8, isc, indirect_indexed_for_store),
            0xf4 => (4, nop_addr, zero_page_x), // UNDOCUMENTED
            0xf5 => (4, sbc, zero_page_x),
            0xf6 => (6, inc, zero_page_x),
            0xf7 => (6, isc, zero_page_x),
            0xf8 => (2, sed, ),
            0xf9 => (4, sbc, absolute_y),
            0xfa => (2, nop, ), // UNDOCUMENTED
            0xfb => (7, isc, absolute_y_for_store),
            0xfc => (4, nop_addr, absolute_x), // UNDOCUMENTED
            0xfd => (4, sbc, absolute_x),
            0xfe => (7, inc, absolute_x),
            0xff => (7, isc, absolute_x_for_store)
        }
    };
}

#[allow(non_upper_case_globals)]
pub mod length {
    pub const immediate: u16 = 2;
    pub const absolute: u16 = 3;
    pub const absolute_x: u16 = 3;
    pub const absolute_y: u16 = 3;
    pub const absolute_x_for_store: u16 = 3;
    pub const absolute_y_for_store: u16 = 3;
    pub const zero_page: u16 = 2;
    pub const zero_page_x: u16 = 2;
    pub const zero_page_y: u16 = 2;
    pub const indirect: u16 = 3;
    pub const indexed_indirect: u16 = 2;
    pub const indirect_indexed: u16 = 2;
    pub const indirect_indexed_for_store: u16 = 2;
}
