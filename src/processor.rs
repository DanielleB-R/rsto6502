use crate::flags::Flags;

#[derive(Default, Clone)]
pub struct Processor {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub f: Flags,
}

impl Processor {
    pub fn new() -> Processor {
        Processor::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_new() {
        let cpu = Processor::new();

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.f, Flags::default());
    }
}
