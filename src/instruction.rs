use crate::hardware::{AddressingMode, Flags, Memory, Sign, CPU};

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
    pub fn from_machine_code(memory: &[u8]) -> Option<Self> {
        match memory {
            [0xAD, fst, snd, ..] => Some(Self::new(
                InstructionType::LDA,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            )),
            [0xBD, fst, snd, ..] => Some(Self::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            )),
            [0xB9, fst, snd, ..] => Some(Self::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            )),
            [0xA9, value, ..] => Some(Self::new(InstructionType::LDA, AddressingMode::Immediate(*value))),
            [0xA5, value, ..] => Some(Self::new(InstructionType::LDA, AddressingMode::ZeroPage(*value))),
            [0xA1, value, ..] => Some(Self::new(InstructionType::LDA, AddressingMode::IndexedIndirect(*value))),
            [0xB5, value, ..] => Some(Self::new(InstructionType::LDA, AddressingMode::ZeroPageX(*value))),
            [0xB1, value, ..] => Some(Self::new(InstructionType::LDA, AddressingMode::IndirectIndexed(*value))),
            _ => None,
        }
    }

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
        Self { cpu, memory }
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
        self.memory
            .read_8_bit_value_by_mode(self.cpu, instruction.addressing_mode)
            .expect("Failed to read value")
    }

    fn write_8_bit_value(&mut self, instruction: Instruction, value: u8) {
        self.memory
            .write_8_bit_value_by_mode(self.cpu, instruction.addressing_mode, value);
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
    use super::{Instruction, InstructionExecutor, InstructionType};
    use crate::hardware::{AddressingMode, Memory, CPU};

    #[test]
    pub fn test_adc() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)));
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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)));

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

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LDA, AddressingMode::Immediate(0x01)));

        assert_eq!(cpu.A, 0x01);
    }

    #[test]
    pub fn test_ldx() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LDX, AddressingMode::Immediate(0x01)));
        assert_eq!(cpu.X, 0x01);
    }

    #[test]
    pub fn test_ldy() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LDY, AddressingMode::Immediate(0x01)));
        assert_eq!(cpu.Y, 0x01);
    }

    #[test]
    pub fn test_sta() {
        let mut cpu = CPU::new();
        cpu.A = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STA, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_stx() {
        let mut cpu = CPU::new();
        cpu.X = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STX, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_sty() {
        let mut cpu = CPU::new();
        cpu.Y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STY, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_lda_absolute_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xAD, 0x10, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::Absolute(0xD010))
        );
    }

    #[test]
    pub fn test_lda_absolute_x_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xBD, 0x10, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::AbsoluteX(0xD010))
        );
    }

    #[test]
    pub fn test_lda_absolute_y_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB9, 0x10, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::AbsoluteY(0xD010))
        );
    }

    #[test]
    pub fn test_lda_immediate_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA9, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::Immediate(0xD0))
        );
    }

    #[test]
    pub fn test_lda_zero_page_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA5, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::ZeroPage(0xD0))
        );
    }

    #[test]
    pub fn test_lda_indexed_indirect_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA1, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::IndexedIndirect(0xD0))
        );
    }

    #[test]
    pub fn test_lda_zero_page_x_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB5, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::ZeroPageX(0xD0))
        );
    }

    #[test]
    pub fn test_lda_indirect_indexed_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB1, 0xD0]).expect("Invalid machine code");
        assert_eq!(
            lda,
            Instruction::new(InstructionType::LDA, AddressingMode::IndirectIndexed(0xD0))
        );
    }
}
