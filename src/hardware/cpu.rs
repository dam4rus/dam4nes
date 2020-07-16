use super::{
    memory::{Memory, MemoryMapper},
    ppu::PPU,
};
use std::fmt::{Formatter, Display};

const INTERNAL_MEMORY_SIZE: usize = 2048;

#[derive(Copy, Clone)]
pub struct CPU {
    pub registers: Registers,
    pub internal_memory: [u8; INTERNAL_MEMORY_SIZE],
}

impl CPU {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
            internal_memory: [0; INTERNAL_MEMORY_SIZE],
        }
    }

    pub fn with_power_up_state() -> Self {
        Self {
            registers: Registers {
                a: 0x00,
                x: 0x00,
                y: 0x00,
                p: 0x34,
                s: 0xFD,
                pc: 0x0000,
            },
            internal_memory: [0; INTERNAL_MEMORY_SIZE],
        }
    }
}

pub struct MMU<'a, 'b> {
    cpu: &'a mut CPU,
    ppu: &'a mut PPU,
    mapper: Option<&'a MemoryMapper<'b>>,
}

impl<'a, 'b> MMU<'a, 'b> {
    pub fn new(cpu: &'a mut CPU, ppu: &'a mut PPU, mapper: Option<&'a MemoryMapper<'b>>) -> Self {
        Self { cpu, ppu, mapper }
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
            0x0000..=0x1FFF => Some(self.cpu.internal_memory[address as usize % INTERNAL_MEMORY_SIZE]),
            0x2000..=0x3FFF => {
                let registers = &self.ppu.registers;
                Some(match address % 0x08 {
                    0 => registers.ppuctrl,
                    1 => registers.ppumask,
                    2 => registers.ppustatus,
                    3 => registers.oamaddr,
                    4 => registers.oamdata,
                    5 => registers.ppuscroll,
                    6 => registers.ppudata,
                    7 => registers.oamdma,
                    _ => unreachable!(),
                })
            }
            0x4000..=0x4017 => unimplemented!("APU and IO registers"),
            0x4018..=0x401F => unimplemented!("APU and IO functionality"),
            0x4020..=0xFFFF => self.mapper.and_then(|mapper| mapper.read(address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.cpu.internal_memory[address as usize % INTERNAL_MEMORY_SIZE] = value,
            0x2000..=0x3FFF => {
                let registers = &mut self.ppu.registers;
                match address % 0x08 {
                    0 => registers.ppuctrl = value,
                    1 => registers.ppumask = value,
                    2 => registers.ppustatus = value,
                    3 => registers.oamaddr = value,
                    4 => registers.oamdata = value,
                    5 => registers.ppuscroll = value,
                    6 => registers.ppudata = value,
                    7 => registers.oamdata = value,
                    _ => unreachable!(),
                }
            }
            _ => panic!("Access violation. Trying to write to read only address {:#X}", address),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub s: u8,
    pub pc: u16,
}

impl Registers {
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

impl Display for AddressingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::Implied => write!(f, "Implied"),
            AddressingMode::Accumulator => write!(f, "Accumulator"),
            AddressingMode::Immediate(value) => write!(f, "Immediate({:#X})", *value),
            AddressingMode::ZeroPage(address) => write!(f, "ZeroPage({:#X})", *address),
            AddressingMode::ZeroPageX(address) => write!(f, "ZeroPageX({:#X})", *address),
            AddressingMode::ZeroPageY(address) => write!(f, "ZeroPageY({:#X})", *address),
            AddressingMode::Relative(offset) => write!(f, "Relative({})", *offset),
            AddressingMode::Absolute(address) => write!(f, "Absolute({:#X})", *address),
            AddressingMode::AbsoluteX(address) => write!(f, "AbsoluteX({:#X})", *address),
            AddressingMode::AbsoluteY(address) => write!(f, "AbsoluteY({:#X})", *address),
            AddressingMode::Indirect(address) => write!(f, "Indirect({:#X})", *address),
            AddressingMode::IndexedIndirect(address) => write!(f, "IndexedIndirect({:#X})", *address),
            AddressingMode::IndirectIndexed(address) => write!(f, "IndirectIndexed({:#X})", *address),
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
