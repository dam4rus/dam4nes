use super::{AddressingMode, CPU};
use std::u16;

pub struct Memory([u8; u16::MAX as usize]);

impl Memory {
    pub fn new() -> Self {
        Self([0; u16::MAX as usize])
    }

    pub fn stack<'a>(&'a mut self, cpu: &'a mut CPU) -> Stack {
        Stack::new(cpu, &mut self.0[0x0100..0x0200])
    }

    pub fn read_8_bit_value(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    pub fn write_8_bit_value(&mut self, address: u16, value: u8) {
        self.0[address as usize] = value;
    }

    pub fn read_8_bit_value_by_mode(&self, cpu: &CPU, addressing_mode: AddressingMode) -> Option<u8> {
        match addressing_mode {
            AddressingMode::Accumulator => Some(cpu.a),
            AddressingMode::Immediate(value) => Some(value),
            mode => self.address_by_mode(cpu, mode).map(|address| self.0[address as usize]),
        }
    }

    pub fn write_8_bit_value_by_mode(&mut self, cpu: &mut CPU, addressing_mode: AddressingMode, value: u8) {
        match addressing_mode {
            AddressingMode::Accumulator => cpu.a = value,
            mode => {
                self.write_8_bit_value(
                    self.address_by_mode(cpu, mode)
                        .expect("Invalid addressing mode"),
                    value,
                )
            }
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

    fn read_16_bit_value(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.0[address as usize],
            self.0[address.checked_add(1).expect("Address out of bounds") as usize],
        ])
    }
}

pub struct Stack<'a> {
    cpu: &'a mut CPU,
    slice: &'a mut [u8],
}

impl<'a> Stack<'a> {
    pub fn new(cpu: &'a mut CPU, slice: &'a mut [u8]) -> Self {
        Self {
            cpu,
            slice,
        }
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

#[cfg(test)]
pub mod tests {
    use crate::hardware::{Memory, CPU};

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
}
