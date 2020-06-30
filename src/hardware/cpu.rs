#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CPU {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub P: u8,
    pub S: u8,
    pub PC: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            P: 0x00,
            S: 0xFF,
            PC: 0x0000,
        }
    }

    pub fn flags(&self) -> Flags {
        Flags::from(self.P)
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.P = flags.into();
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            carry: (value & 0x01) != 0,
            zero: (value & 0x02) != 0,
            interrupt_disable: (value & 0x04) != 0,
            decimal: (value & 0x08) != 0,
            overflow: (value & 0x40) != 0,
            negative: (value & 0x80) != 0,
        }
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.carry {
            value |= 0x01;
        }
        if self.zero {
            value |= 0x02;
        }
        if self.interrupt_disable {
            value |= 0x04;
        }
        if self.decimal {
            value |= 0x08;
        }
        if self.overflow {
            value |= 0x40;
        }
        if self.negative {
            value |= 0x80;
        }

        value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Relative(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

impl AddressingMode {
    pub fn byte_length(&self) -> u8 {
        match self {
            AddressingMode::Implied => 1,
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate(_) => 2,
            AddressingMode::ZeroPage(_) => 2,
            AddressingMode::ZeroPageX(_) => 2,
            AddressingMode::ZeroPageY(_) => 2,
            AddressingMode::Relative(_) => 2,
            AddressingMode::Absolute(_) => 3,
            AddressingMode::AbsoluteX(_) => 3,
            AddressingMode::AbsoluteY(_) => 3,
            AddressingMode::Indirect(_) => 3,
            AddressingMode::IndexedIndirect(_) => 2,
            AddressingMode::IndirectIndexed(_) => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

impl From<u8> for Sign {
    fn from(value: u8) -> Self {
        match (value & 0x80) == 0 {
            true => Sign::Positive,
            false => Sign::Negative,
        }
    }
}
