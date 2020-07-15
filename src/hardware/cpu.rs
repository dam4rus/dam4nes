use super::memory::{Memory, MemoryMapper};

pub struct CPU {
    pub registers: Registers,
    pub internal_memory: [u8; 0x0800],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            internal_memory: [0; 0x0800],
        }
    }
}

pub struct MMU<'a, 'b> {
    cpu: &'a mut CPU,
    mapper: Option<&'a MemoryMapper<'b>>,
}

impl<'a, 'b> MMU<'a, 'b> {
    pub fn new(cpu: &'a mut CPU, mapper: Option<&'a MemoryMapper<'b>>) -> Self {
        Self { cpu, mapper }
    }

    pub fn cpu(&self) -> &CPU {
        self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut CPU {
        self.cpu
    }

    pub fn read_by_mode(&self, addressing_mode: AddressingMode) -> Option<u8> {
        match addressing_mode {
            AddressingMode::Accumulator => Some(self.cpu.registers.a),
            AddressingMode::Immediate(value) => Some(value),
            mode => self.address_by_mode(mode).and_then(|address| self.read(address)),
        }
    }

    pub fn write_8_bit_value_by_mode(&mut self, addressing_mode: AddressingMode, value: u8) {
        match addressing_mode {
            AddressingMode::Accumulator => self.cpu.registers.a = value,
            mode => self.write(self.address_by_mode(mode).expect("Invalid addressing mode"), value),
        }
    }

    pub fn address_by_mode(&self, addressing_mode: AddressingMode) -> Option<u16> {
        match addressing_mode {
            AddressingMode::ZeroPage(address) => Some(address as u16),
            AddressingMode::ZeroPageX(address) => Some(address.wrapping_add(self.cpu.registers.x) as u16),
            AddressingMode::ZeroPageY(address) => Some(address.wrapping_add(self.cpu.registers.y) as u16),
            AddressingMode::Absolute(address) => Some(address),
            AddressingMode::AbsoluteX(address) => Some(address.wrapping_add(self.cpu.registers.x as u16)),
            AddressingMode::AbsoluteY(address) => Some(address.wrapping_add(self.cpu.registers.y as u16)),
            AddressingMode::IndexedIndirect(address) => {
                self.read_16_bit_value(address.wrapping_add(self.cpu.registers.x) as u16)
            }
            AddressingMode::IndirectIndexed(address) => self
                .read_16_bit_value(address as u16)
                .map(|value| value.wrapping_add(self.cpu.registers.y as u16)),
            _ => None,
        }
    }
}

impl<'a, 'b> Memory for MMU<'a, 'b> {
    fn read(&self, address: u16) -> Option<u8> {
        match address {
            0x0000..=0x1FFF => Some(self.cpu.internal_memory[(address % 0x0800) as usize]),
            0x2000..=0x3FFF => unimplemented!("Reads ppu registers"),
            0x4000..=0x4017 => unimplemented!("APU and IO registers"),
            0x4018..=0x401F => unimplemented!("APU and IO functionality"),
            0x4020..=0xFFFF => self.mapper.and_then(|mapper| mapper.read(address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.cpu.internal_memory[(address % 0x0800) as usize] = value,
            _ => panic!("Access violation. Trying to write to read only address {:#X}", address),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub s: u8,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: 0x00,
            s: 0xFF,
            pc: 0x0000,
        }
    }

    pub fn flags(&self) -> Flags {
        Flags::from(self.p)
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.p = flags.into();
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub break_command: bool,
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
            break_command: (value & 0x10) != 0,
            overflow: (value & 0x40) != 0,
            negative: (value & 0x80) != 0,
        }
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut value = 0;
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
        if self.break_command {
            value |= 0x10;
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
    Relative(i8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

impl AddressingMode {
    pub fn byte_length(&self) -> u32 {
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
