use super::{AddressingMode, CPU};
use crate::{error::InvalidOpCode, instruction::Instruction};
use std::u16;

pub struct Memory<'a> {
    pool: [u8; u16::MAX as usize],
    mapper: Option<MemoryMapper<'a>>,
}

impl<'a> Memory<'a> {
    pub fn new() -> Self {
        Self {
            pool: [0; u16::MAX as usize],
            mapper: None,
        }
    }

    pub fn stack<'b>(&'b mut self, cpu: &'b mut CPU) -> Stack<'b> {
        Stack::new(cpu, &mut self.pool[0x0100..0x0200])
    }

    pub fn set_mapper(&mut self, mapper: Option<MemoryMapper<'a>>) {
        self.mapper = mapper;
    }

    pub fn read_8_bit_value(&self, address: u16) -> u8 {
        self.mapper
            .and_then(|mapper| mapper.map(address))
            .unwrap_or(self.pool[address as usize])
    }

    pub fn write_8_bit_value(&mut self, address: u16, value: u8) {
        self.pool[address as usize] = value;
    }

    pub fn read_8_bit_value_by_mode(&self, cpu: &CPU, addressing_mode: AddressingMode) -> Option<u8> {
        match addressing_mode {
            AddressingMode::Accumulator => Some(cpu.a),
            AddressingMode::Immediate(value) => Some(value),
            mode => self
                .address_by_mode(cpu, mode)
                .map(|address| self.pool[address as usize]),
        }
    }

    pub fn write_8_bit_value_by_mode(&mut self, cpu: &mut CPU, addressing_mode: AddressingMode, value: u8) {
        match addressing_mode {
            AddressingMode::Accumulator => cpu.a = value,
            mode => self.write_8_bit_value(self.address_by_mode(cpu, mode).expect("Invalid addressing mode"), value),
        }
    }

    pub fn address_by_mode(&self, cpu: &CPU, addressing_mode: AddressingMode) -> Option<u16> {
        match addressing_mode {
            AddressingMode::ZeroPage(address) => Some(address as u16),
            AddressingMode::ZeroPageX(address) => Some(address.wrapping_add(cpu.x) as u16),
            AddressingMode::ZeroPageY(address) => Some(address.wrapping_add(cpu.y) as u16),
            AddressingMode::Absolute(address) => Some(address),
            AddressingMode::AbsoluteX(address) => Some(address.wrapping_add(cpu.x as u16)),
            AddressingMode::AbsoluteY(address) => Some(address.wrapping_add(cpu.y as u16)),
            AddressingMode::IndexedIndirect(address) => {
                Some(self.read_16_bit_value(address.wrapping_add(cpu.x) as u16))
            }
            AddressingMode::IndirectIndexed(address) => {
                Some(self.read_16_bit_value(address as u16).wrapping_add(cpu.y as u16))
            }
            _ => None,
        }
    }

    pub fn read_instruction(&self, start_address: u16) -> Result<Instruction, InvalidOpCode> {
        let start_byte = self.read_8_bit_value(start_address);
        let instruction = match start_byte {
            0x00 | 0x08 | 0x0A | 0x18 | 0x28 | 0x2A | 0x38 | 0x40 | 0x48 | 0x4A | 0x58 | 0x60 | 0x68 | 0x6A | 0x78
            | 0x88 | 0x8A | 0x98 | 0x9A | 0xA8 | 0xAA | 0xB8 | 0xBA | 0xC8 | 0xCA | 0xD8 | 0xE8 | 0xEA | 0xF8 => {
                Instruction::from_machine_code(&[start_byte])
            }
            0x01 | 0x05 | 0x06 | 0x09 | 0x10 | 0x11 | 0x15 | 0x16 | 0x21 | 0x24 | 0x25 | 0x26 | 0x29 | 0x30 | 0x31
            | 0x35 | 0x36 | 0x41 | 0x45 | 0x46 | 0x49 | 0x50 | 0x51 | 0x55 | 0x56 | 0x61 | 0x65 | 0x66 | 0x69
            | 0x70 | 0x71 | 0x75 | 0x76 | 0x81 | 0x84 | 0x85 | 0x86 | 0x90 | 0x91 | 0x94 | 0x95 | 0x96 | 0xA0
            | 0xA1 | 0xA2 | 0xA4 | 0xA5 | 0xA6 | 0xA9 | 0xB0 | 0xB1 | 0xB4 | 0xB5 | 0xB6 | 0xC0 | 0xC1 | 0xC4
            | 0xC5 | 0xC6 | 0xC9 | 0xD0 | 0xD1 | 0xD5 | 0xD6 | 0xE0 | 0xE1 | 0xE4 | 0xE5 | 0xE6 | 0xE9 | 0xF0
            | 0xF1 | 0xF5 | 0xF6 => {
                Instruction::from_machine_code(&[start_byte, self.read_8_bit_value(start_address.wrapping_add(1))])
            }
            0x0D | 0x0E | 0x19 | 0x1D | 0x1E | 0x20 | 0x2C | 0x2D | 0x2E | 0x39 | 0x3D | 0x3E | 0x4C | 0x4D | 0x4E
            | 0x59 | 0x5D | 0x5E | 0x6C | 0x6D | 0x6E | 0x79 | 0x7D | 0x7E | 0x8C | 0x8D | 0x8E | 0x99 | 0x9D
            | 0xAC | 0xAD | 0xAE | 0xB9 | 0xBC | 0xBD | 0xBE | 0xCC | 0xCD | 0xCE | 0xD9 | 0xDD | 0xDE | 0xEC
            | 0xED | 0xEE | 0xF9 | 0xFD | 0xFE => Instruction::from_machine_code(&[
                start_byte,
                self.read_8_bit_value(start_address.wrapping_add(1)),
                self.read_8_bit_value(start_address.wrapping_add(2)),
            ]),
            op_code => return Err(InvalidOpCode::new(op_code)),
        };

        match instruction {
            Ok(Some(instruction)) => Ok(instruction),
            Ok(None) => unreachable!("Instruction op code array can't be empty"),
            Err(err) => Err(err),
        }
    }

    fn read_16_bit_value(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.pool[address as usize],
            self.pool[address.checked_add(1).expect("Address out of bounds") as usize],
        ])
    }
}

pub struct Stack<'a> {
    cpu: &'a mut CPU,
    slice: &'a mut [u8],
}

impl<'a> Stack<'a> {
    pub fn new(cpu: &'a mut CPU, slice: &'a mut [u8]) -> Self {
        Self { cpu, slice }
    }

    pub fn push(&mut self, value: u8) {
        self.slice[self.cpu.s as usize] = value;
        self.cpu.s = self.cpu.s.wrapping_sub(0x01);
    }

    pub fn pop(&mut self) -> u8 {
        self.cpu.s = self.cpu.s.wrapping_add(0x01);
        self.slice[self.cpu.s as usize]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryMapper<'a> {
    NROM(&'a [u8], &'a [u8]),
}

impl<'a> MemoryMapper<'a> {
    pub fn map(&self, address: u16) -> Option<u8> {
        match self {
            MemoryMapper::NROM(bank1, bank2) => match address {
                0x8000..=0xBFFF => Some(bank1[address as usize - 0x8000]),
                0xC000..=0xFFFF => Some(bank2[address as usize - 0xC000]),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        hardware::{Memory, MemoryMapper, CPU},
        rom::PRG_PAGE_SIZE,
    };

    #[test]
    pub fn test_stack() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut stack = memory.stack(&mut cpu);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        assert_eq!(stack.pop(), 5);
        assert_eq!(stack.pop(), 4);
        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    pub fn test_nrom_mapper() {
        let mut memory = Memory::new();
        let mut prg_rom = [0u8; PRG_PAGE_SIZE * 2];
        prg_rom[0] = 0x01;
        prg_rom[PRG_PAGE_SIZE as usize] = 0x02;
        memory.set_mapper(Some(MemoryMapper::NROM(
            &prg_rom[0x0000..PRG_PAGE_SIZE],
            &prg_rom[PRG_PAGE_SIZE..],
        )));
        assert_eq!(memory.read_8_bit_value(0x8000), 0x01);
        assert_eq!(memory.read_8_bit_value(0xC000), 0x02);
    }
}
