use crate::{
    hardware::{AddressingMode, Flags, Memory, Sign, CPU},
    error::InvalidOpCode,
};

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
    RTI,
    CLC,
    SEC,
    CLD,
    SED,
    CLI,
    SEI,
    CLV,
    BRK,
    NOP,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn from_machine_code(memory: &[u8]) -> Result<Option<Self>, InvalidOpCode> {
        match memory {
            [0x00, ..] => Ok(Some(Self::new(InstructionType::BRK, AddressingMode::Implied))),
            [0x01, value, ..] => Ok(Some(Self::new(InstructionType::ORA, AddressingMode::IndexedIndirect(*value)))),
            [0x05, value, ..] => Ok(Some(Self::new(InstructionType::ORA, AddressingMode::ZeroPage(*value)))),
            [0x06, value, ..] => Ok(Some(Self::new(InstructionType::ASL, AddressingMode::ZeroPage(*value)))),
            [0x08, ..] => Ok(Some(Self::new(InstructionType::PHP, AddressingMode::Implied))),
            [0x09, value, ..] => Ok(Some(Self::new(InstructionType::ORA, AddressingMode::Immediate(*value)))),
            [0x0A, ..] => Ok(Some(Self::new(InstructionType::ASL, AddressingMode::Accumulator))),
            [0x0D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ORA,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x0E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ASL,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x10, value, ..] => Ok(Some(Self::new(InstructionType::BPL, AddressingMode::Relative(*value as i8)))),
            [0x11, value, ..] => Ok(Some(Self::new(InstructionType::ORA, AddressingMode::IndirectIndexed(*value)))),
            [0x15, value, ..] => Ok(Some(Self::new(InstructionType::ORA, AddressingMode::ZeroPageX(*value)))),
            [0x16, value, ..] => Ok(Some(Self::new(InstructionType::ASL, AddressingMode::ZeroPageX(*value)))),
            [0x18, ..] => Ok(Some(Self::new(InstructionType::CLC, AddressingMode::Implied))),
            [0x19, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ORA,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x1D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ORA,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x1E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ASL,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x20, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::JSR,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x21, value, ..] => Ok(Some(Self::new(InstructionType::AND, AddressingMode::IndexedIndirect(*value)))),
            [0x24, value, ..] => Ok(Some(Self::new(InstructionType::BIT, AddressingMode::ZeroPage(*value)))),
            [0x25, value, ..] => Ok(Some(Self::new(InstructionType::AND, AddressingMode::ZeroPage(*value)))),
            [0x26, value, ..] => Ok(Some(Self::new(InstructionType::ROL, AddressingMode::ZeroPage(*value)))),
            [0x28, ..] => Ok(Some(Self::new(InstructionType::PLP, AddressingMode::Implied))),
            [0x29, value, ..] => Ok(Some(Self::new(InstructionType::AND, AddressingMode::Immediate(*value)))),
            [0x2A, ..] => Ok(Some(Self::new(InstructionType::ROL, AddressingMode::Accumulator))),
            [0x2C, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::BIT,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x2D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::AND,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x2E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ROL,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x30, value, ..] => Ok(Some(Self::new(InstructionType::BMI, AddressingMode::Relative(*value as i8)))),
            [0x31, value, ..] => Ok(Some(Self::new(InstructionType::AND, AddressingMode::IndirectIndexed(*value)))),
            [0x35, value, ..] => Ok(Some(Self::new(InstructionType::AND, AddressingMode::ZeroPageX(*value)))),
            [0x36, value, ..] => Ok(Some(Self::new(InstructionType::ROL, AddressingMode::ZeroPageX(*value)))),
            [0x38, ..] => Ok(Some(Self::new(InstructionType::SEC, AddressingMode::Implied))),
            [0x39, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::AND,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x3D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::AND,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x3E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ROL,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x40, ..] => Ok(Some(Self::new(InstructionType::RTI, AddressingMode::Implied))),
            [0x41, value, ..] => Ok(Some(Self::new(InstructionType::EOR, AddressingMode::IndexedIndirect(*value)))),
            [0x45, value, ..] => Ok(Some(Self::new(InstructionType::EOR, AddressingMode::ZeroPage(*value)))),
            [0x46, value, ..] => Ok(Some(Self::new(InstructionType::LSR, AddressingMode::ZeroPage(*value)))),
            [0x48, ..] => Ok(Some(Self::new(InstructionType::PHA, AddressingMode::Implied))),
            [0x49, value, ..] => Ok(Some(Self::new(InstructionType::EOR, AddressingMode::Immediate(*value)))),
            [0x4A, ..] => Ok(Some(Self::new(InstructionType::LSR, AddressingMode::Accumulator))),
            [0x4C, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::JMP,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x4D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::EOR,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x4E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LSR,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x50, value, ..] => Ok(Some(Self::new(InstructionType::BVC, AddressingMode::Relative(*value as i8)))),
            [0x51, value, ..] => Ok(Some(Self::new(InstructionType::EOR, AddressingMode::IndirectIndexed(*value)))),
            [0x55, value, ..] => Ok(Some(Self::new(InstructionType::EOR, AddressingMode::ZeroPageX(*value)))),
            [0x56, value, ..] => Ok(Some(Self::new(InstructionType::LSR, AddressingMode::ZeroPageX(*value)))),
            [0x58, ..] => Ok(Some(Self::new(InstructionType::CLI, AddressingMode::Implied))),
            [0x59, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::EOR,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x5D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::EOR,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x5E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LSR,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x60, ..] => Ok(Some(Self::new(InstructionType::RTS, AddressingMode::Implied))),
            [0x61, value, ..] => Ok(Some(Self::new(InstructionType::ADC, AddressingMode::IndexedIndirect(*value)))),
            [0x65, value, ..] => Ok(Some(Self::new(InstructionType::ADC, AddressingMode::ZeroPage(*value)))),
            [0x66, value, ..] => Ok(Some(Self::new(InstructionType::ROR, AddressingMode::ZeroPage(*value)))),
            [0x68, ..] => Ok(Some(Self::new(InstructionType::PLA, AddressingMode::Implied))),
            [0x69, value, ..] => Ok(Some(Self::new(InstructionType::ADC, AddressingMode::Immediate(*value)))),
            [0x6A, ..] => Ok(Some(Self::new(InstructionType::ROR, AddressingMode::Accumulator))),
            [0x6C, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::JMP,
                AddressingMode::Indirect(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x6D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ADC,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x6E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ROR,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x70, value, ..] => Ok(Some(Self::new(InstructionType::BVS, AddressingMode::Relative(*value as i8)))),
            [0x71, value, ..] => Ok(Some(Self::new(InstructionType::ADC, AddressingMode::IndirectIndexed(*value)))),
            [0x75, value, ..] => Ok(Some(Self::new(InstructionType::ADC, AddressingMode::ZeroPageX(*value)))),
            [0x76, value, ..] => Ok(Some(Self::new(InstructionType::ROR, AddressingMode::ZeroPageX(*value)))),
            [0x78, ..] => Ok(Some(Self::new(InstructionType::SEI, AddressingMode::Implied))),
            [0x79, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ADC,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x7D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ADC,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x7E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::ROR,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x81, value, ..] => Ok(Some(Self::new(InstructionType::STA, AddressingMode::IndexedIndirect(*value)))),
            [0x84, value, ..] => Ok(Some(Self::new(InstructionType::STY, AddressingMode::ZeroPage(*value)))),
            [0x85, value, ..] => Ok(Some(Self::new(InstructionType::STA, AddressingMode::ZeroPage(*value)))),
            [0x86, value, ..] => Ok(Some(Self::new(InstructionType::STX, AddressingMode::ZeroPage(*value)))),
            [0x88, ..] => Ok(Some(Self::new(InstructionType::DEY, AddressingMode::Implied))),
            [0x8A, ..] => Ok(Some(Self::new(InstructionType::TXA, AddressingMode::Implied))),
            [0x8C, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::STY,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x8D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::STA,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x8E, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::STX,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x90, value, ..] => Ok(Some(Self::new(InstructionType::BCC, AddressingMode::Relative(*value as i8)))),
            [0x91, value, ..] => Ok(Some(Self::new(InstructionType::STA, AddressingMode::IndirectIndexed(*value)))),
            [0x94, value, ..] => Ok(Some(Self::new(InstructionType::STY, AddressingMode::ZeroPageX(*value)))),
            [0x95, value, ..] => Ok(Some(Self::new(InstructionType::STA, AddressingMode::ZeroPageX(*value)))),
            [0x96, value, ..] => Ok(Some(Self::new(InstructionType::STX, AddressingMode::ZeroPageY(*value)))),
            [0x98, ..] => Ok(Some(Self::new(InstructionType::TYA, AddressingMode::Implied))),
            [0x99, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::STA,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0x9A, ..] => Ok(Some(Self::new(InstructionType::TXS, AddressingMode::Implied))),
            [0x9D, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::STA,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xA0, value, ..] => Ok(Some(Self::new(InstructionType::LDY, AddressingMode::Immediate(*value)))),
            [0xA1, value, ..] => Ok(Some(Self::new(InstructionType::LDA, AddressingMode::IndexedIndirect(*value)))),
            [0xA2, value, ..] => Ok(Some(Self::new(InstructionType::LDX, AddressingMode::Immediate(*value)))),
            [0xA4, value, ..] => Ok(Some(Self::new(InstructionType::LDY, AddressingMode::ZeroPage(*value)))),
            [0xA5, value, ..] => Ok(Some(Self::new(InstructionType::LDA, AddressingMode::ZeroPage(*value)))),
            [0xA6, value, ..] => Ok(Some(Self::new(InstructionType::LDX, AddressingMode::ZeroPage(*value)))),
            [0xA8, ..] => Ok(Some(Self::new(InstructionType::TAY, AddressingMode::Implied))),
            [0xA9, value, ..] => Ok(Some(Self::new(InstructionType::LDA, AddressingMode::Immediate(*value)))),
            [0xAA, ..] => Ok(Some(Self::new(InstructionType::TAX, AddressingMode::Implied))),
            [0xAC, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDY,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xAD, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDA,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xAE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDX,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xB0, value, ..] => Ok(Some(Self::new(InstructionType::BCS, AddressingMode::Relative(*value as i8)))),
            [0xB1, value, ..] => Ok(Some(Self::new(InstructionType::LDA, AddressingMode::IndirectIndexed(*value)))),
            [0xB4, value, ..] => Ok(Some(Self::new(InstructionType::LDY, AddressingMode::ZeroPageX(*value)))),
            [0xB5, value, ..] => Ok(Some(Self::new(InstructionType::LDA, AddressingMode::ZeroPageX(*value)))),
            [0xB6, value, ..] => Ok(Some(Self::new(InstructionType::LDX, AddressingMode::ZeroPageY(*value)))),
            [0xB8, ..] => Ok(Some(Self::new(InstructionType::CLV, AddressingMode::Implied))),
            [0xB9, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xBA, ..] => Ok(Some(Self::new(InstructionType::TSX, AddressingMode::Implied))),
            [0xBC, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDY,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xBD, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xBE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::LDX,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xC0, value, ..] => Ok(Some(Self::new(InstructionType::CPY, AddressingMode::Immediate(*value)))),
            [0xC1, value, ..] => Ok(Some(Self::new(InstructionType::CMP, AddressingMode::IndexedIndirect(*value)))),
            [0xC4, value, ..] => Ok(Some(Self::new(InstructionType::CPY, AddressingMode::ZeroPage(*value)))),
            [0xC5, value, ..] => Ok(Some(Self::new(InstructionType::CMP, AddressingMode::ZeroPage(*value)))),
            [0xC6, value, ..] => Ok(Some(Self::new(InstructionType::DEC, AddressingMode::ZeroPage(*value)))),
            [0xC8, ..] => Ok(Some(Self::new(InstructionType::INY, AddressingMode::Implied))),
            [0xC9, value, ..] => Ok(Some(Self::new(InstructionType::CMP, AddressingMode::Immediate(*value)))),
            [0xCA, ..] => Ok(Some(Self::new(InstructionType::DEX, AddressingMode::Implied))),
            [0xCC, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::CPY,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xCD, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::CMP,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xCE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::DEC,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xD0, value, ..] => Ok(Some(Self::new(InstructionType::BNE, AddressingMode::Relative(*value as i8)))),
            [0xD1, value, ..] => Ok(Some(Self::new(InstructionType::CMP, AddressingMode::IndirectIndexed(*value)))),
            [0xD5, value, ..] => Ok(Some(Self::new(InstructionType::CMP, AddressingMode::ZeroPageX(*value)))),
            [0xD6, value, ..] => Ok(Some(Self::new(InstructionType::DEC, AddressingMode::ZeroPageX(*value)))),
            [0xD8, ..] => Ok(Some(Self::new(InstructionType::CLD, AddressingMode::Implied))),
            [0xD9, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::CMP,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xDD, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::CMP,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xDE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::DEC,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xE0, value, ..] => Ok(Some(Self::new(InstructionType::CPX, AddressingMode::Immediate(*value)))),
            [0xE1, value, ..] => Ok(Some(Self::new(InstructionType::SBC, AddressingMode::IndexedIndirect(*value)))),
            [0xE4, value, ..] => Ok(Some(Self::new(InstructionType::CPX, AddressingMode::ZeroPage(*value)))),
            [0xE5, value, ..] => Ok(Some(Self::new(InstructionType::SBC, AddressingMode::ZeroPage(*value)))),
            [0xE6, value, ..] => Ok(Some(Self::new(InstructionType::INC, AddressingMode::ZeroPage(*value)))),
            [0xE8, ..] => Ok(Some(Self::new(InstructionType::INX, AddressingMode::Implied))),
            [0xE9, value, ..] => Ok(Some(Self::new(InstructionType::SBC, AddressingMode::Immediate(*value)))),
            [0xEA, ..] => Ok(Some(Self::new(InstructionType::NOP, AddressingMode::Implied))),
            [0xEC, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::CPX,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xED, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::SBC,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xEE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::INC,
                AddressingMode::Absolute(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xF0, value, ..] => Ok(Some(Self::new(InstructionType::BEQ, AddressingMode::Relative(*value as i8)))),
            [0xF1, value, ..] => Ok(Some(Self::new(InstructionType::SBC, AddressingMode::IndirectIndexed(*value)))),
            [0xF5, value, ..] => Ok(Some(Self::new(InstructionType::SBC, AddressingMode::ZeroPageX(*value)))),
            [0xF6, value, ..] => Ok(Some(Self::new(InstructionType::INC, AddressingMode::ZeroPageX(*value)))),
            [0xF8, ..] => Ok(Some(Self::new(InstructionType::SED, AddressingMode::Implied))),
            [0xF9, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::SBC,
                AddressingMode::AbsoluteY(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xFD, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::SBC,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [0xFE, fst, snd, ..] => Ok(Some(Self::new(
                InstructionType::INC,
                AddressingMode::AbsoluteX(u16::from_le_bytes([*fst, *snd])),
            ))),
            [op_code, ..] => Err(InvalidOpCode::new(*op_code)),
            _ => Ok(None),
        }
    }

    fn new(instruction_type: InstructionType, addressing_mode: AddressingMode) -> Self {
        Self {
            instruction_type,
            addressing_mode,
        }
    }

    pub fn byte_length(&self) -> u32 {
        self.addressing_mode.byte_length()
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
                self.cpu.pc = self
                    .memory
                    .address_by_mode(self.cpu, instruction.addressing_mode)
                    .expect("Invalid addressing mode");
            }
            InstructionType::JSR => match instruction.addressing_mode {
                AddressingMode::Absolute(address) => {
                    let return_address = self.cpu.pc.wrapping_sub(1).to_le_bytes();
                    let mut stack = self.memory.stack(self.cpu);
                    stack.push(return_address[1]);
                    stack.push(return_address[0]);
                    self.cpu.pc = address;
                }
                _ => panic!("Invalid addressing mode for JSR. JSR only support absolute addressing"),
            },
            InstructionType::RTS => {
                let mut stack = self.memory.stack(self.cpu);
                let lo_word = stack.pop();
                let hi_word = stack.pop();
                self.cpu.pc = u16::from_le_bytes([lo_word, hi_word]).wrapping_add(1);
            }
            InstructionType::RTI => {
                let mut stack = self.memory.stack(self.cpu);
                let flags = Flags::from(stack.pop());
                let lo_word = stack.pop();
                let hi_word = stack.pop();
                self.cpu.set_flags(flags);
                self.cpu.pc = u16::from_le_bytes([lo_word, hi_word]);
            }
            InstructionType::CLC => self.cpu.set_flags(Flags {
                carry: false,
                ..self.cpu.flags()
            }),
            InstructionType::SEC => self.cpu.set_flags(Flags {
                carry: true,
                ..self.cpu.flags()
            }),
            InstructionType::CLD => self.cpu.set_flags(Flags {
                decimal: false,
                ..self.cpu.flags()
            }),
            InstructionType::SED => self.cpu.set_flags(Flags {
                decimal: true,
                ..self.cpu.flags()
            }),
            InstructionType::CLI => self.cpu.set_flags(Flags {
                interrupt_disable: false,
                ..self.cpu.flags()
            }),
            InstructionType::SEI => self.cpu.set_flags(Flags {
                interrupt_disable: true,
                ..self.cpu.flags()
            }),
            InstructionType::CLV => self.cpu.set_flags(Flags {
                overflow: false,
                ..self.cpu.flags()
            }),
            InstructionType::BRK => {
                let pc_address = self.cpu.pc.to_le_bytes();
                let status = self.cpu.p;
                let mut stack = self.memory.stack(self.cpu);
                stack.push(pc_address[1]);
                stack.push(pc_address[0]);
                stack.push(status);
                self.cpu.set_flags(Flags {
                    break_command: true,
                    ..self.cpu.flags()
                });
            }
            InstructionType::NOP => (),
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
    use crate::{
        hardware::{AddressingMode, Flags, Memory, CPU},
        error::InvalidOpCode,
    };

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
        cpu.set_flags(Flags {
            carry: true,
            ..Default::default()
        });
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
        cpu.set_flags(Flags {
            carry: true,
            ..Default::default()
        });
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
        InstructionExecutor::new(&mut cpu, &mut memory).execute(Instruction::new(
            InstructionType::AND,
            AddressingMode::Immediate(0b10101010),
        ));

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
        InstructionExecutor::new(&mut cpu, &mut memory).execute(Instruction::new(
            InstructionType::ORA,
            AddressingMode::Immediate(0b10101010),
        ));

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
        InstructionExecutor::new(&mut cpu, &mut memory).execute(Instruction::new(
            InstructionType::EOR,
            AddressingMode::Immediate(0b01010101),
        ));

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
        cpu.set_flags(Flags {
            carry: true,
            ..Default::default()
        });
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
        cpu.set_flags(Flags {
            zero: true,
            ..Default::default()
        });
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
        cpu.set_flags(Flags {
            negative: true,
            ..Default::default()
        });
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
        cpu.set_flags(Flags {
            overflow: true,
            ..Default::default()
        });
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
            .execute(Instruction::new(InstructionType::TYA, AddressingMode::Implied));

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
    pub fn test_rti() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut stack = memory.stack(&mut cpu);
        stack.push(0x06);
        stack.push(0x00);
        stack.push(0b00000011);
        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::RTI, AddressingMode::Implied));

        let flags = cpu.flags();
        assert_eq!(cpu.pc, 0x0600);
        assert!(flags.carry);
        assert!(flags.zero);
        assert!(!flags.interrupt_disable);
        assert!(!flags.decimal);
        assert!(!flags.overflow);
        assert!(!flags.negative);
    }

    #[test]
    pub fn test_clc() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CLC, AddressingMode::Implied));

        assert!(!cpu.flags().carry);
    }

    #[test]
    pub fn test_sec() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SEC, AddressingMode::Implied));

        assert!(cpu.flags().carry);
    }

    #[test]
    pub fn test_cld() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_flags(Flags {
            decimal: true,
            ..Default::default()
        });

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CLD, AddressingMode::Implied));

        assert!(!cpu.flags().decimal);
    }

    #[test]
    pub fn test_sed() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SED, AddressingMode::Implied));

        assert!(cpu.flags().decimal);
    }

    #[test]
    pub fn test_cli() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_flags(Flags {
            interrupt_disable: true,
            ..Default::default()
        });

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CLI, AddressingMode::Implied));

        assert!(!cpu.flags().interrupt_disable);
    }

    #[test]
    pub fn test_sei() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::SEI, AddressingMode::Implied));

        assert!(cpu.flags().interrupt_disable);
    }

    #[test]
    pub fn test_clv() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_flags(Flags {
            overflow: true,
            ..Default::default()
        });

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::CLV, AddressingMode::Implied));

        assert!(!cpu.flags().overflow);
    }

    #[test]
    pub fn test_brk() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.set_flags(Flags {
            carry: true,
            ..Default::default()
        });
        let mut memory = Memory::new();

        InstructionExecutor::new(&mut cpu, &mut memory)
            .execute(Instruction::new(InstructionType::BRK, AddressingMode::Implied));

        let mut stack = memory.stack(&mut cpu);
        assert_eq!(
            stack.pop(),
            Flags {
                carry: true,
                ..Default::default()
            }
            .into()
        );
        assert_eq!(stack.pop(), 0x00);
        assert_eq!(stack.pop(), 0x06);
        let flags = cpu.flags();
        assert!(flags.break_command);
    }

    #[test]
    pub fn test_lda_absolute_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xAD, 0x10, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::Absolute(0xD010)))
        );
    }

    #[test]
    pub fn test_lda_absolute_x_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xBD, 0x10, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::AbsoluteX(0xD010)))
        );
    }

    #[test]
    pub fn test_lda_absolute_y_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB9, 0x10, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::AbsoluteY(0xD010)))
        );
    }

    #[test]
    pub fn test_lda_immediate_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA9, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::Immediate(0xD0)))
        );
    }

    #[test]
    pub fn test_lda_zero_page_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA5, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::ZeroPage(0xD0)))
        );
    }

    #[test]
    pub fn test_lda_indexed_indirect_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xA1, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::IndexedIndirect(0xD0)))
        );
    }

    #[test]
    pub fn test_lda_zero_page_x_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB5, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::ZeroPageX(0xD0)))
        );
    }

    #[test]
    pub fn test_lda_indirect_indexed_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB1, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(InstructionType::LDA, AddressingMode::IndirectIndexed(0xD0)))
        );
    }

    #[test]
    pub fn test_invalid_machine_code() {
        let error = Instruction::from_machine_code(&[0xFF]).unwrap_err();
        assert_eq!(error, InvalidOpCode::new(0xFF));
    }

    #[test]
    pub fn test_empty_machine_code() {
        let empty = Instruction::from_machine_code(&[]).unwrap();
        assert_eq!(empty, None);
    }
}
