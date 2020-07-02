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
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    ASL,
    LSR,
    ROL,
    ROR,
    AND,
    ORA,
    EOR,
    CMP,
    CPX,
    CPY,
    BIT,
    BCC,
    BCS,
    BNE,
    BEQ,
    BPL,
    BMI,
    BVC,
    BVS,
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    JMP,
    JSR,
    RTS,
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
                let old_accumulator = self.cpu.a;
                let value = self.read_8_bit_value(instruction);
                let status_flags = self.cpu.flags();
                let carried = if status_flags.carry { 1 } else { 0 };
                self.cpu.a = self.cpu.a.wrapping_add(value).wrapping_add(carried);

                self.update_flags_after_arithmetic(old_accumulator, value, self.cpu.a < old_accumulator);
            }
            InstructionType::SBC => {
                let old_accumulator = self.cpu.a;
                let value = self.read_8_bit_value(instruction);
                let status_flags = self.cpu.flags();
                let carried = if status_flags.carry { 1 } else { 0 };
                self.cpu.a = self.cpu.a.wrapping_sub(value).wrapping_sub(carried);

                self.update_flags_after_arithmetic(old_accumulator, value, self.cpu.a > old_accumulator);
            }
            InstructionType::LDA => {
                self.cpu.a = self.read_8_bit_value(instruction);
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::LDX => {
                self.cpu.x = self.read_8_bit_value(instruction);
                self.update_zero_and_negative_flags(self.cpu.x);
            }
            InstructionType::LDY => {
                self.cpu.y = self.read_8_bit_value(instruction);
                self.update_zero_and_negative_flags(self.cpu.y);
            }
            InstructionType::STA => self.write_8_bit_value(instruction, self.cpu.a),
            InstructionType::STX => self.write_8_bit_value(instruction, self.cpu.x),
            InstructionType::STY => self.write_8_bit_value(instruction, self.cpu.y),
            InstructionType::INC => {
                let value = self.read_8_bit_value(instruction).wrapping_add(1);
                self.write_8_bit_value(instruction, value);
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::INX => {
                self.cpu.x = self.cpu.x.wrapping_add(1);
                self.update_zero_and_negative_flags(self.cpu.x);
            }
            InstructionType::INY => {
                self.cpu.y = self.cpu.y.wrapping_add(1);
                self.update_zero_and_negative_flags(self.cpu.y);
            }
            InstructionType::DEC => {
                let value = self.read_8_bit_value(instruction).wrapping_sub(1);
                self.write_8_bit_value(instruction, value);
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::DEX => {
                self.cpu.x = self.cpu.x.wrapping_sub(1);
                self.update_zero_and_negative_flags(self.cpu.x);
            }
            InstructionType::DEY => {
                self.cpu.y = self.cpu.y.wrapping_sub(1);
                self.update_zero_and_negative_flags(self.cpu.y);
            }
            InstructionType::ASL => {
                let old_value = self.read_8_bit_value(instruction);
                let value = old_value << 1;
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b10000000) != 0);
            }
            InstructionType::LSR => {
                let old_value = self.read_8_bit_value(instruction);
                let value = old_value >> 1;
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b00000001) != 0);
            }
            InstructionType::ROL => {
                let old_value = self.read_8_bit_value(instruction);
                let value = if self.cpu.flags().carry {
                    (old_value << 1) | 0b00000001
                } else {
                    old_value << 1
                };
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b10000000) != 0);
            }
            InstructionType::ROR => {
                let old_value = self.read_8_bit_value(instruction);
                let value = if self.cpu.flags().carry {
                    (old_value >> 1) | 0b10000000
                } else {
                    old_value >> 1
                };
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b00000001) != 0);
            }
            InstructionType::AND => {
                let value = self.read_8_bit_value(instruction);
                self.cpu.a = self.cpu.a & value;
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::ORA => {
                let value = self.read_8_bit_value(instruction);
                self.cpu.a = self.cpu.a | value;
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::EOR => {
                let value = self.read_8_bit_value(instruction);
                self.cpu.a = self.cpu.a ^ value;
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::CMP => {
                let value = self.read_8_bit_value(instruction);
                let subtracted = self.cpu.a.wrapping_sub(value);
                self.update_flags_after_compare(self.cpu.a, value, subtracted);
            }
            InstructionType::CPX => {
                let value = self.read_8_bit_value(instruction);
                let subtracted = self.cpu.x.wrapping_sub(value);
                self.update_flags_after_compare(self.cpu.x, value, subtracted);
            }
            InstructionType::CPY => {
                let value = self.read_8_bit_value(instruction);
                let subtracted = self.cpu.y.wrapping_sub(value);
                self.update_flags_after_compare(self.cpu.y, value, subtracted);
            }
            InstructionType::BIT => {
                let value = self.read_8_bit_value(instruction);
                let result = self.cpu.a & value;
                self.cpu.set_flags(Flags {
                    negative: (value & 0b10000000) != 0,
                    overflow: (value & 0b01000000) != 0,
                    zero: result == 0,
                    ..self.cpu.flags()
                })
            }
            InstructionType::BCC => {
                if !self.cpu.flags().carry {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BCS => {
                if self.cpu.flags().carry {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BNE => {
                if !self.cpu.flags().zero {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BEQ => {
                if self.cpu.flags().zero {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BPL => {
                if !self.cpu.flags().negative {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BMI => {
                if self.cpu.flags().negative {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BVC => {
                if !self.cpu.flags().overflow {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BVS => {
                if self.cpu.flags().overflow {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::TAX => {
                self.cpu.x = self.cpu.a;
                self.update_zero_and_negative_flags(self.cpu.x);
            }
            InstructionType::TXA => {
                self.cpu.a = self.cpu.x;
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::TAY => {
                self.cpu.y = self.cpu.a;
                self.update_zero_and_negative_flags(self.cpu.y);
            }
            InstructionType::TYA => {
                self.cpu.a = self.cpu.y;
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::TSX => {
                self.cpu.x = self.cpu.s;
                self.update_zero_and_negative_flags(self.cpu.x);
            }
            InstructionType::TXS => {
                self.cpu.s = self.cpu.x;
                self.update_zero_and_negative_flags(self.cpu.s);
            }
            InstructionType::PHA => {
                let value = self.cpu.a;
                self.memory.stack(self.cpu).push(value);
            }
            InstructionType::PLA => {
                self.cpu.a = self.memory.stack(self.cpu).pop();
                self.update_zero_and_negative_flags(self.cpu.a);
            }
            InstructionType::PHP => {
                let value = self.cpu.p;
                self.memory.stack(self.cpu).push(value);
            }
            InstructionType::PLP => {
                self.cpu.p = self.memory.stack(self.cpu).pop();
            }
            InstructionType::JMP => {
                self.cpu.pc = self.memory.address_by_mode(self.cpu, instruction.addressing_mode).expect("Invalid addressing mode");
            }
            InstructionType::JSR => {
                match instruction.addressing_mode {
                    AddressingMode::Absolute(address) => {
                        let return_address = self.cpu.pc.wrapping_sub(1).to_le_bytes();
                        let mut stack = self.memory.stack(self.cpu);
                        stack.push(return_address[1]);
                        stack.push(return_address[0]);
                        self.cpu.pc = address;
                    }
                    _ => panic!("Invalid addressing mode for JSR. JSR only support absolute addressing"),
                }
            }
            InstructionType::RTS => {
                let mut stack = self.memory.stack(self.cpu);
                let lo_word = stack.pop();
                let hi_word = stack.pop();
                self.cpu.pc = u16::from_le_bytes([lo_word, hi_word]).wrapping_add(1);
            }
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
        let a_sign = Sign::from(self.cpu.a);
        self.cpu.set_flags(Flags {
            negative: a_sign == Sign::Negative,
            overflow: a_sign != Sign::from(old_a) && a_sign != Sign::from(value),
            zero: self.cpu.a == 0,
            carry,
            ..self.cpu.flags()
        });
    }

    fn update_flags_after_shift(&mut self, value: u8, carry: bool) {
        self.cpu.set_flags(Flags {
            negative: Sign::from(value) == Sign::Negative,
            zero: value == 0,
            carry,
            ..self.cpu.flags()
        })
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.cpu.set_flags(Flags {
            zero: value == 0,
            negative: Sign::from(value) == Sign::Negative,
            ..self.cpu.flags()
        })
    }

    fn update_flags_after_compare(&mut self, register_value: u8, memory_value: u8, result: u8) {
        self.cpu.set_flags(Flags {
            negative: (result & 0b10000000) != 0,
            zero: register_value == memory_value,
            carry: register_value >= memory_value,
            ..self.cpu.flags()
        })
    }

    fn jump(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Relative(jump_offset) => {
                self.cpu.pc = match jump_offset.is_positive() {
                    true => self.cpu.pc.wrapping_add(jump_offset as u16),
                    false => self.cpu.pc.wrapping_sub(jump_offset.abs() as u16),
                }
            }
            _ => panic!("Invalid addressing mode for jump instruction. They can only use relative addressing"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Instruction, InstructionExecutor, InstructionType};
    use crate::hardware::{AddressingMode, Memory, CPU, Flags};

    #[test]
    pub fn test_adc() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x02);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x00);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.a = 0x7F;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)));
        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x80);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.a = 0x80;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x7F);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x00);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x02);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc_overflow() {
        let mut cpu = CPU::new();
        cpu.a = 0xFF;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x00);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.a = 0x00;
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0xFF);
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

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ldx() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LDX, AddressingMode::Immediate(0x01)));
        
        let flags = cpu.flags();
        assert_eq!(cpu.x, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ldy() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LDY, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert_eq!(cpu.y, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sta() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STA, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_stx() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STX, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_sty() {
        let mut cpu = CPU::new();
        cpu.y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::STY, AddressingMode::Absolute(0x0200)));
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
    }

    #[test]
    pub fn test_inc() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::INC, AddressingMode::Absolute(0x0200)));

        let flags = cpu.flags();
        assert_eq!(memory.read_8_bit_value(0x0200), 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_inx() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::INX, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.x, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_iny() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::INY, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.y, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dec() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::DEC, AddressingMode::Absolute(0x0200)));

        let flags = cpu.flags();
        assert_eq!(memory.read_8_bit_value(0x0200), 0xFF);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dex() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::DEX, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.x, 0xFF);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dey() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::DEY, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.y, 0xFF);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_asl() {
        let mut cpu = CPU::new();
        cpu.a = 0b01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ASL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_asl_saturating() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ASL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b11111110);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }
    
    #[test]
    pub fn test_asl_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0b10000000;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ASL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b0);
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }


    #[test]
    pub fn test_lsr() {
        let mut cpu = CPU::new();
        cpu.a = 0b10;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LSR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_lsr_saturating() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LSR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b01111111);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }
    
    #[test]
    pub fn test_lsr_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0b00000001;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::LSR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b00);
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_rol() {
        let mut cpu = CPU::new();
        cpu.a = 0b01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_rol_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b11111110);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_rol_carry_over() {
        let mut cpu = CPU::new();
        cpu.a = 0b00;
        cpu.set_flags(Flags { carry: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROL, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_ror() {
        let mut cpu = CPU::new();
        cpu.a = 0b10;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_ror_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b01111111);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_ror_carry_over() {
        let mut cpu = CPU::new();
        cpu.a = 0b00;
        cpu.set_flags(Flags { carry: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ROR, AddressingMode::Accumulator));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10000000);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_and() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::AND, AddressingMode::Immediate(0b10101010)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ora() {
        let mut cpu = CPU::new();
        cpu.a = 0b00000000;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::ORA, AddressingMode::Immediate(0b10101010)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_eor() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::EOR, AddressingMode::Immediate(0b01010101)));

        let flags = cpu.flags();
        assert_eq!(cpu.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_cmp_equals() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cmp_greater() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x00)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cmp_less() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x02)));

        let flags = cpu.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_cpx_equals() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpx_greater() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x00)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpx_less() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x02)));

        let flags = cpu.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_cpy_equals() {
        let mut cpu = CPU::new();
        cpu.y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x01)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpy_greater() {
        let mut cpu = CPU::new();
        cpu.y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x00)));

        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpy_less() {
        let mut cpu = CPU::new();
        cpu.y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x02)));

        let flags = cpu.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_bit_zero() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        memory.write_8_bit_value(0x0000, 0b00000000);
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BIT, AddressingMode::ZeroPage(0x00)));
        
        let flags = cpu.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(!flags.overflow);
    }

    #[test]
    pub fn test_bit_negative_and_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0b11111111;
        let mut memory = Memory::new();
        memory.write_8_bit_value(0x0000, 0b11000000);
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BIT, AddressingMode::ZeroPage(0x00)));
        
        let flags = cpu.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.overflow);
    }

    #[test]
    pub fn test_bcc() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BCC, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }


    #[test]
    pub fn test_bcs() {
        let mut cpu = CPU::new();
        cpu.set_flags(Flags{ carry: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BCS, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    pub fn test_bne() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BNE, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }


    #[test]
    pub fn test_beq() {
        let mut cpu = CPU::new();
        cpu.set_flags(Flags{ zero: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BEQ, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    pub fn test_bpl() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BPL, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    pub fn test_bmi() {
        let mut cpu = CPU::new();
        cpu.set_flags(Flags{ negative: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BMI, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    pub fn test_bvc() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BVC, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }


    #[test]
    pub fn test_bvs() {
        let mut cpu = CPU::new();
        cpu.set_flags(Flags{ overflow: true, ..Default::default() });
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BVS, AddressingMode::Relative(2)));

        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    pub fn test_tax() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TAX, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.x, cpu.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_txa() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TXA, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.x, cpu.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tay() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TAY, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.y, cpu.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tya() {
        let mut cpu = CPU::new();
        cpu.y = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TAY, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.y, cpu.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.s = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TSX, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.x, cpu.s);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_txs() {
        let mut cpu = CPU::new();
        cpu.x = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::TXS, AddressingMode::Implied));
        
        let flags = cpu.flags();
        assert_eq!(cpu.x, cpu.s);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_pha() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::PHA, AddressingMode::Implied));

        assert_eq!(cpu.s, 0xFE);
        assert_eq!(memory.read_8_bit_value(0x01FF), cpu.a);
    }


    #[test]
    pub fn test_pla() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        memory.stack(&mut cpu).push(0x01);
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::PLA, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.s, 0xFF);
        assert_eq!(memory.read_8_bit_value(0x01FF), cpu.a);
        assert!(!flags.zero);
        assert!(!flags.negative);
    }

    #[test]
    pub fn test_php() {
        let mut cpu = CPU::new();
        cpu.p = 0x01;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::PHP, AddressingMode::Implied));

        assert_eq!(cpu.s, 0xFE);
        assert_eq!(memory.read_8_bit_value(0x01FF), cpu.p);
    }


    #[test]
    pub fn test_plp() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        memory.stack(&mut cpu).push(0x01);
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::PLP, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.s, 0xFF);
        assert_eq!(memory.read_8_bit_value(0x01FF), cpu.p);
        assert!(!flags.zero);
        assert!(!flags.negative);
    }

    #[test]
    pub fn test_jmp() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::JMP, AddressingMode::Absolute(0x0600)));

        assert_eq!(cpu.pc, 0x0600);
    }

    #[test]
    pub fn test_jsr() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0601;
        let mut memory = Memory::new();
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::JSR, AddressingMode::Absolute(0x1000)));

        let mut stack = memory.stack(&mut cpu);
        assert_eq!(stack.pop(), 0x00);
        assert_eq!(stack.pop(), 0x06);
        assert_eq!(cpu.pc, 0x1000);
    }

    #[test]
    pub fn test_rts() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut stack = memory.stack(&mut cpu);
        stack.push(0x06);
        stack.push(0x00);

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::RTS, AddressingMode::Implied));

        assert_eq!(cpu.pc, 0x0601);
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
