use std::u16;
use super::{CPU, AddressingMode};

pub struct Memory([u8; u16::MAX as usize]);

impl Memory {
    pub fn new() -> Self {
        Self([0; u16::MAX as usize])
    }

    pub fn stack(&mut self) -> Stack {
        Stack(&mut self.0[0x0100 .. 0x0200])
    }

    pub fn read_value(&self, cpu: &CPU, addressing_mode: AddressingMode) -> Option<u8> {
        match addressing_mode {
            AddressingMode::Implicit => None,
            AddressingMode::Accumulator => Some(cpu.A),
            AddressingMode::Immediate(value) => Some(value),
            AddressingMode::ZeroPage(address) => Some(self.0[address as usize]),
            AddressingMode::ZeroPageX(address) => Some(self.0[address.wrapping_add(cpu.X) as usize]),
            AddressingMode::ZeroPageY(address) => Some(self.0[address.wrapping_add(cpu.Y) as usize]),
            AddressingMode::Relative(_) => None,
            AddressingMode::Absolute(address) => Some(self.0[address as usize]),
            AddressingMode::AbsoluteX(address) => Some(self.0[(address + cpu.X as u16) as usize]),
            AddressingMode::AbsoluteY(address) => Some(self.0[(address + cpu.Y as u16) as usize]),
            AddressingMode::Indirect(_) => None,
            AddressingMode::IndexedIndirect(address) => {
                let indirect_address = self.read_16_bit_value(address.wrapping_add(cpu.X) as u16);
                Some(self.0[indirect_address as usize])
            }
            AddressingMode::IndirectIndexed(address) => {
                let indirect_address = self.read_16_bit_value(address as u16).wrapping_add(cpu.Y as u16);
                Some(self.0[indirect_address as usize])
            }
        }
    }

    fn read_16_bit_value(&self, address: u16) -> u16 {
        let low_byte = self.0[address.checked_add(1).expect("Address out of bounds") as usize];
        let high_byte = self.0[address as usize];
        low_byte as u16 | ((high_byte as u16) << 8)
    }
}

pub struct Stack<'a>(&'a mut [u8]);

impl<'a> Stack<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self(slice)
    }

    pub fn push(&mut self, stack_pointer: &mut u8, value: u8) {
        self.0[*stack_pointer as usize] = value;
        *stack_pointer = stack_pointer.checked_sub(0x01).expect("Stack pointer overflow");
    }

    pub fn pop(&mut self, stack_pointer: &mut u8) -> u8 {
        *stack_pointer = stack_pointer.checked_add(0x01).expect("Stack pointer underflow");
        self.0[*stack_pointer as usize]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::hardware::{CPU, Memory};

    #[test]
    pub fn test_stack() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut stack = memory.stack();
        stack.push(&mut cpu.S, 1);
        stack.push(&mut cpu.S, 2);
        stack.push(&mut cpu.S, 3);
        stack.push(&mut cpu.S, 4);
        stack.push(&mut cpu.S, 5);
        assert_eq!(stack.pop(&mut cpu.S), 5);
        assert_eq!(stack.pop(&mut cpu.S), 4);
        assert_eq!(stack.pop(&mut cpu.S), 3);
        assert_eq!(stack.pop(&mut cpu.S), 2);
        assert_eq!(stack.pop(&mut cpu.S), 1);
    }
}