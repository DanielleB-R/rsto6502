use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Flags {
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub v: bool,
    pub n: bool,
}

impl Flags {
    pub fn set_z(&mut self, n: u8) {
        self.z = n == 0;
    }

    pub fn set_n(&mut self, n: u8) {
        self.n = n & 0x80 != 0;
    }

    pub fn get_byte(&self) -> u8 {
        (self.c as u8)
            | ((self.z as u8) << 1)
            | ((self.i as u8) << 2)
            | ((self.d as u8) << 3)
            | ((self.v as u8) << 6)
            | ((self.n as u8) << 7)
    }

    pub fn set_byte(&mut self, byte: u8) {
        self.c = byte & 0x01 != 0;
        self.z = byte & 0x02 != 0;
        self.i = byte & 0x04 != 0;
        self.d = byte & 0x08 != 0;
        self.v = byte & 0x40 != 0;
        self.n = byte & 0x80 != 0;
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.get_byte())
    }
}

#[macro_export]
macro_rules! flag {
    ( $($f:ident: $v:expr),* ) => {
        Flags {
            $($f: $v,)*
            ..Flags::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(flag! {}, Flags::default());
        assert_eq!(
            flag! {c: true},
            Flags {
                c: true,
                ..Flags::default()
            }
        );
    }

    #[test]
    fn test_set_z() {
        let mut f = Flags::default();
        assert!(!f.z);

        f.set_z(0);
        assert!(f.z);

        for n in 1..=0xff {
            f.set_z(n);
            assert!(!f.z);
            f.set_z(0);
        }
    }

    #[test]
    fn test_set_n() {
        let mut f = Flags::default();
        assert!(!f.n);

        for n in 0..=0x7f {
            f.set_n(n | 0x80);
            assert!(f.n);
            f.set_n(n);
            assert!(!f.n);
        }
    }
}
