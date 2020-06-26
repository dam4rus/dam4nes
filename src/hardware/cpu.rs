use super::Memory;

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

#[derive(Debug, Copy, Clone)]
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
    Implicit,
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
    pub fn byte_length(&self) -> u8 {
        match self {
            AddressingMode::Implicit => 1,
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
enum Sign {
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

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory);
    fn cycles(&self) -> u8;
}

pub struct ADC(pub AddressingMode);

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let old_accumulator = cpu.A;
        let value_at_address = memory.read_value(cpu, self.0).expect("Failed to read memory address");
        let status_flags = cpu.flags();
        let carried = if status_flags.carry { 1 } else { 0 };
        cpu.A = cpu.A.wrapping_add(value_at_address).wrapping_add(carried);

        let a_sign = Sign::from(cpu.A);
        cpu.set_flags(Flags {
            negative: a_sign == Sign::Negative,
            overflow: a_sign != Sign::from(old_accumulator) && a_sign != Sign::from(value_at_address),
            zero: cpu.A == 0,
            carry: cpu.A < old_accumulator,
            ..status_flags
        });
    }

    fn cycles(&self) -> u8 {
        match self.0 {
            AddressingMode::Immediate(_) => 2,
            AddressingMode::ZeroPage(_) => 3,
            AddressingMode::ZeroPageX(_) => 4,
            AddressingMode::Absolute(_) => 4,
            AddressingMode::AbsoluteX(_) => 4,
            AddressingMode::AbsoluteY(_) => 4,
            AddressingMode::IndexedIndirect(_) => 6,
            AddressingMode::IndirectIndexed(_) => 5,
            _ => unreachable!("Invalid addressing mode for ADC"),
        }
    }
}

pub struct SBC(pub AddressingMode);

impl Instruction for SBC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let old_accumulator = cpu.A;
        let value_at_address = memory.read_value(cpu, self.0).expect("Failed to read memory address");
        let status_flags = cpu.flags();
        let carried = if status_flags.carry { 1 } else { 0 };
        cpu.A = cpu.A.wrapping_sub(value_at_address).wrapping_sub(carried);

        let a_sign = Sign::from(cpu.A);
        cpu.set_flags(Flags {
            negative: a_sign == Sign::Negative,
            overflow: a_sign != Sign::from(old_accumulator) && a_sign != Sign::from(value_at_address),
            zero: cpu.A == 0,
            carry: cpu.A > old_accumulator,
            ..status_flags
        });
    }

    fn cycles(&self) -> u8 {
        match self.0 {
            AddressingMode::Immediate(_) => 2,
            AddressingMode::ZeroPage(_) => 3,
            AddressingMode::ZeroPageX(_) => 4,
            AddressingMode::Absolute(_) => 4,
            AddressingMode::AbsoluteX(_) => 4,
            AddressingMode::AbsoluteY(_) => 4,
            AddressingMode::IndexedIndirect(_) => 6,
            AddressingMode::IndirectIndexed(_) => 5,
            _ => unreachable!("Invalid addressing mode for SBC"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::hardware::{CPU, Memory, ADC, SBC, AddressingMode, Instruction};

    #[test]
    pub fn test_adc() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        ADC(AddressingMode::Immediate(0x01)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x02);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        ADC(AddressingMode::Immediate(0xFF)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x00);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.A = 0x7F;
        let mut memory = Memory::new();
        ADC(AddressingMode::Immediate(0x01)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x80);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.A = 0x80;
        let mut memory = Memory::new();
        ADC(AddressingMode::Immediate(0xFF)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x7F);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        SBC(AddressingMode::Immediate(0x01)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x00);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        SBC(AddressingMode::Immediate(0xFF)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x02);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc_overflow() {
        let mut cpu = CPU::new();
        cpu.A = 0xFF;
        let mut memory = Memory::new();
        SBC(AddressingMode::Immediate(0xFF)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0x00);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.A = 0x00;
        let mut memory = Memory::new();
        SBC(AddressingMode::Immediate(0x01)).execute(&mut cpu, &mut memory);
        let flags = cpu.flags();
        assert_eq!(cpu.A, 0xFF);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }
}