use super::cpu::CPU;
use std::u16;

pub trait Memory {
    fn read(&self, address: u16) -> Option<u8>;
    fn write(&mut self, address: u16, value: u8);

    fn read_16_bit_value(&self, address: u16) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.read(address)?,
            self.read(address.checked_add(1).expect("Address out of bounds"))?,
        ]))
    }
}

pub struct Stack<'a> {
    cpu: &'a mut CPU,
}

impl<'a> Stack<'a> {
    pub fn new(cpu: &'a mut CPU) -> Self {
        Self { cpu }
    }

    pub fn push(&mut self, value: u8) {
        self.cpu.internal_memory[self.cpu.registers.s as usize + 0x0100] = value;
        self.cpu.registers.s = self.cpu.registers.s.wrapping_sub(0x01);
    }

    pub fn pop(&mut self) -> u8 {
        self.cpu.registers.s = self.cpu.registers.s.wrapping_add(0x01);
        self.cpu.internal_memory[self.cpu.registers.s as usize + 0x0100]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryMapper<'a> {
    NROM(&'a [u8], &'a [u8]),
}

impl<'a> MemoryMapper<'a> {
    pub fn read(&self, address: u16) -> Option<u8> {
        self.slice_from(address).and_then(|slice| slice.first()).copied()
    }

    pub fn slice_from(&self, address: u16) -> Option<&[u8]> {
        match self {
            MemoryMapper::NROM(bank1, bank2) => match address {
                0x8000..=0xBFFF => Some(&bank1[address as usize - 0x8000..]),
                0xC000..=0xFFFF => Some(&bank2[address as usize - 0xC000..]),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        hardware::{
            cpu::CPU,
            memory::{MemoryMapper, Stack},
        },
        rom::PRG_PAGE_SIZE,
    };

    #[test]
    pub fn test_stack() {
        let mut cpu = CPU::new();
        let mut stack = Stack::new(&mut cpu);
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
        let mut prg_rom = [0u8; PRG_PAGE_SIZE * 2];
        prg_rom[0] = 0x01;
        prg_rom[PRG_PAGE_SIZE as usize] = 0x02;
        let mapper = MemoryMapper::NROM(&prg_rom[0x0000..PRG_PAGE_SIZE], &prg_rom[PRG_PAGE_SIZE..]);
        assert_eq!(mapper.read(0x8000), Some(0x01));
        assert_eq!(mapper.read(0xC000), Some(0x02));
    }
}
