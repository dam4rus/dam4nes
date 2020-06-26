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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InstructionType {
    ADC,
    SBC,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, addressing_mode: AddressingMode) -> Self {
        Self {
            instruction_type,
            addressing_mode,
        }
    }
}

pub struct InstructionExecutor<'a> {
    cpu: &'a mut CPU,
    memory: &'a mut Memory,
}

impl<'a> InstructionExecutor<'a> {
    pub fn new(cpu: &'a mut CPU, memory: &'a mut Memory) -> Self {
        Self {
            cpu,
            memory,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction.instruction_type {
            InstructionType::ADC => {
                let old_accumulator = self.cpu.A;
                let value = self.read_8_bit_value(instruction);
                let status_flags = self.cpu.flags();
                let carried = if status_flags.carry { 1 } else { 0 };
                self.cpu.A = self.cpu.A.wrapping_add(value).wrapping_add(carried);

                self.update_flags_after_arithmetic(old_accumulator, value, self.cpu.A < old_accumulator);
            }
            InstructionType::SBC => {
                let old_accumulator = self.cpu.A;
                let value = self.read_8_bit_value(instruction);
                let status_flags = self.cpu.flags();
                let carried = if status_flags.carry { 1 } else { 0 };
                self.cpu.A = self.cpu.A.wrapping_sub(value).wrapping_sub(carried);

                self.update_flags_after_arithmetic(old_accumulator, value, self.cpu.A > old_accumulator);
            }
            InstructionType::LDA => {
                self.cpu.A = self.read_8_bit_value(instruction);
                self.update_flags_after_load(self.cpu.A);
            }
            InstructionType::LDX => {
                self.cpu.X = self.read_8_bit_value(instruction);
                self.update_flags_after_load(self.cpu.X);
            }
            InstructionType::LDY => {
                self.cpu.Y = self.read_8_bit_value(instruction);
                self.update_flags_after_load(self.cpu.Y);
            }
            InstructionType::STA => self.write_8_bit_value(instruction, self.cpu.A),
            InstructionType::STX => self.write_8_bit_value(instruction, self.cpu.X),
            InstructionType::STY => self.write_8_bit_value(instruction, self.cpu.Y),
        }
    }

    fn read_8_bit_value(&self, instruction: Instruction) -> u8 {
        self.memory.read_8_bit_value_by_mode(self.cpu, instruction.addressing_mode).expect("Failed to read value")
    }

    fn write_8_bit_value(&mut self, instruction: Instruction, value: u8) {
        self.memory.write_8_bit_value_by_mode(self.cpu, instruction.addressing_mode, value);
    }

    fn update_flags_after_arithmetic(&mut self, old_a: u8, value: u8, carry: bool) {
        let a_sign = Sign::from(self.cpu.A);
        self.cpu.set_flags(Flags {
            negative: a_sign == Sign::Negative,
            overflow: a_sign != Sign::from(old_a) && a_sign != Sign::from(value),
            zero: self.cpu.A == 0,
            carry,
            ..self.cpu.flags()
        });
    }

    fn update_flags_after_load(&mut self, register_value: u8) {
        self.cpu.set_flags(Flags {
            zero: register_value == 0,
            negative: Sign::from(register_value) == Sign::Negative,
            ..self.cpu.flags()
        })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::hardware::{CPU, Memory, AddressingMode, Instruction, InstructionType, InstructionExecutor};

    #[test]
    pub fn test_adc() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01))
        );
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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF))
        );

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

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01))
        );

        let flags = cpu.flags();
        assert_eq!(cpu.A, 0xFF);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_lda() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::LDA, AddressingMode::Immediate(0x01))
        );

        assert_eq!(cpu.A, 0x01);
    }

    #[test]
    pub fn test_ldx() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::LDX, AddressingMode::Immediate(0x01))
        );
        assert_eq!(cpu.X, 0x01);
    }

    #[test]
    pub fn test_ldy() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::LDY, AddressingMode::Immediate(0x01))
        );
        assert_eq!(cpu.Y, 0x01);
    }

    #[test]
    pub fn test_sta() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::STA, AddressingMode::Absolute(0x0200))
        );
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_stx() {
        let mut cpu = CPU::new();
        cpu.X = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::STX, AddressingMode::Absolute(0x0200))
        );
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_sty() {
        let mut cpu = CPU::new();
        cpu.Y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory).execute(
            Instruction::new(InstructionType::STY, AddressingMode::Absolute(0x0200))
        );
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }
}