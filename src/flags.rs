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
}

#[cfg(test)]
mod tests {
    use super::*;

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
