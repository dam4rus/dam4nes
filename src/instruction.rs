use crate::{
    error::InvalidOpCode,
    hardware::{
        cpu::{AddressingMode, Flags, Sign, MMU},
        memory::Stack,
    },
};
use log::debug;
use std::fmt::{Display, Formatter};

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

impl InstructionType {
    pub fn increments_pc(&self) -> bool {
        match self {
            InstructionType::JMP | InstructionType::JSR | InstructionType::RTS | InstructionType::RTI => false,
            _ => true,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn from_machine_code(memory: &[u8]) -> Result<Option<Self>, InvalidOpCode> {
        match memory {
            [0x00, ..] => Ok(Some(Self::new(InstructionType::BRK, AddressingMode::Implied))),
            [0x01, value, ..] => Ok(Some(Self::new(
                InstructionType::ORA,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0x10, value, ..] => Ok(Some(Self::new(
                InstructionType::BPL,
                AddressingMode::Relative(*value as i8),
            ))),
            [0x11, value, ..] => Ok(Some(Self::new(
                InstructionType::ORA,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0x21, value, ..] => Ok(Some(Self::new(
                InstructionType::AND,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0x30, value, ..] => Ok(Some(Self::new(
                InstructionType::BMI,
                AddressingMode::Relative(*value as i8),
            ))),
            [0x31, value, ..] => Ok(Some(Self::new(
                InstructionType::AND,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0x41, value, ..] => Ok(Some(Self::new(
                InstructionType::EOR,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0x50, value, ..] => Ok(Some(Self::new(
                InstructionType::BVC,
                AddressingMode::Relative(*value as i8),
            ))),
            [0x51, value, ..] => Ok(Some(Self::new(
                InstructionType::EOR,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0x61, value, ..] => Ok(Some(Self::new(
                InstructionType::ADC,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0x70, value, ..] => Ok(Some(Self::new(
                InstructionType::BVS,
                AddressingMode::Relative(*value as i8),
            ))),
            [0x71, value, ..] => Ok(Some(Self::new(
                InstructionType::ADC,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0x81, value, ..] => Ok(Some(Self::new(
                InstructionType::STA,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0x90, value, ..] => Ok(Some(Self::new(
                InstructionType::BCC,
                AddressingMode::Relative(*value as i8),
            ))),
            [0x91, value, ..] => Ok(Some(Self::new(
                InstructionType::STA,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0xA1, value, ..] => Ok(Some(Self::new(
                InstructionType::LDA,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0xB0, value, ..] => Ok(Some(Self::new(
                InstructionType::BCS,
                AddressingMode::Relative(*value as i8),
            ))),
            [0xB1, value, ..] => Ok(Some(Self::new(
                InstructionType::LDA,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0xC1, value, ..] => Ok(Some(Self::new(
                InstructionType::CMP,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0xD0, value, ..] => Ok(Some(Self::new(
                InstructionType::BNE,
                AddressingMode::Relative(*value as i8),
            ))),
            [0xD1, value, ..] => Ok(Some(Self::new(
                InstructionType::CMP,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
            [0xE1, value, ..] => Ok(Some(Self::new(
                InstructionType::SBC,
                AddressingMode::IndexedIndirect(*value),
            ))),
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
            [0xF0, value, ..] => Ok(Some(Self::new(
                InstructionType::BEQ,
                AddressingMode::Relative(*value as i8),
            ))),
            [0xF1, value, ..] => Ok(Some(Self::new(
                InstructionType::SBC,
                AddressingMode::IndirectIndexed(*value),
            ))),
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
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction: {{ type: {:?}, addressing_mode: {} }}",
            self.instruction_type, self.addressing_mode
        )
    }
}

pub struct InstructionExecutor<'a, 'mmu, 'mapped> {
    mmu: &'a mut MMU<'mmu, 'mapped>,
}

impl<'a, 'mmu, 'mapped> InstructionExecutor<'a, 'mmu, 'mapped> {
    pub fn new(mmu: &'a mut MMU<'mmu, 'mapped>) -> Self {
        Self { mmu }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        //debug!("Executing {}", instruction);
        match instruction.instruction_type {
            InstructionType::ADC => {
                let value = self.read_8_bit_value(instruction);
                let registers = &self.mmu.cpu().registers;
                let old_accumulator = registers.a;
                let carried = if registers.flags().carry { 1 } else { 0 };
                let new_accumulator = old_accumulator.wrapping_add(value).wrapping_add(carried);
                self.mmu.cpu_mut().registers.a = new_accumulator;

                self.update_flags_after_arithmetic(old_accumulator, value, new_accumulator < old_accumulator);
            }
            InstructionType::SBC => {
                let value = self.read_8_bit_value(instruction);
                let registers = &self.mmu.cpu().registers;
                let old_accumulator = registers.a;
                let carried = if registers.flags().carry { 1 } else { 0 };
                let new_accumulator = old_accumulator.wrapping_sub(value).wrapping_sub(carried);
                self.mmu.cpu_mut().registers.a = new_accumulator;

                self.update_flags_after_arithmetic(old_accumulator, value, new_accumulator > old_accumulator);
            }
            InstructionType::LDA => {
                let value = self.read_8_bit_value(instruction);
                self.mmu.cpu_mut().registers.a = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::LDX => {
                let value = self.read_8_bit_value(instruction);
                self.mmu.cpu_mut().registers.x = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::LDY => {
                let value = self.read_8_bit_value(instruction);
                self.mmu.cpu_mut().registers.y = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::STA => self.write_8_bit_value(instruction, self.mmu.cpu().registers.a),
            InstructionType::STX => self.write_8_bit_value(instruction, self.mmu.cpu().registers.x),
            InstructionType::STY => self.write_8_bit_value(instruction, self.mmu.cpu().registers.y),
            InstructionType::INC => {
                let value = self.read_8_bit_value(instruction).wrapping_add(1);
                self.write_8_bit_value(instruction, value);
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::INX => {
                let value = self.mmu.cpu().registers.x.wrapping_add(1);
                self.mmu.cpu_mut().registers.x = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::INY => {
                let value = self.mmu.cpu().registers.y.wrapping_add(1);
                self.mmu.cpu_mut().registers.y = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::DEC => {
                let value = self.read_8_bit_value(instruction).wrapping_sub(1);
                self.write_8_bit_value(instruction, value);
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::DEX => {
                let value = self.mmu.cpu().registers.x.wrapping_sub(1);
                self.mmu.cpu_mut().registers.x = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::DEY => {
                let value = self.mmu.cpu().registers.y.wrapping_sub(1);
                self.mmu.cpu_mut().registers.y = value;
                self.update_zero_and_negative_flags(value);
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
                let value = if self.mmu.cpu().registers.flags().carry {
                    (old_value << 1) | 0b00000001
                } else {
                    old_value << 1
                };
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b10000000) != 0);
            }
            InstructionType::ROR => {
                let old_value = self.read_8_bit_value(instruction);
                let value = if self.mmu.cpu().registers.flags().carry {
                    (old_value >> 1) | 0b10000000
                } else {
                    old_value >> 1
                };
                self.write_8_bit_value(instruction, value);
                self.update_flags_after_shift(value, (old_value & 0b00000001) != 0);
            }
            InstructionType::AND => {
                let value = self.read_8_bit_value(instruction);
                let new_accumulator = self.mmu.cpu().registers.a & value;
                self.mmu.cpu_mut().registers.a = new_accumulator;
                self.update_zero_and_negative_flags(new_accumulator);
            }
            InstructionType::ORA => {
                let value = self.read_8_bit_value(instruction);
                let new_accumulator = self.mmu.cpu().registers.a | value;
                self.mmu.cpu_mut().registers.a = new_accumulator;
                self.update_zero_and_negative_flags(new_accumulator);
            }
            InstructionType::EOR => {
                let value = self.read_8_bit_value(instruction);
                let new_accumulator = self.mmu.cpu().registers.a ^ value;
                self.mmu.cpu_mut().registers.a = new_accumulator;
                self.update_zero_and_negative_flags(new_accumulator);
            }
            InstructionType::CMP => {
                let value = self.read_8_bit_value(instruction);
                let accumulator = self.mmu.cpu().registers.a;
                let subtracted = accumulator.wrapping_sub(value);
                self.update_flags_after_compare(accumulator, value, subtracted);
            }
            InstructionType::CPX => {
                let value = self.read_8_bit_value(instruction);
                let x = self.mmu.cpu().registers.x;
                let subtracted = x.wrapping_sub(value);
                self.update_flags_after_compare(x, value, subtracted);
            }
            InstructionType::CPY => {
                let value = self.read_8_bit_value(instruction);
                let y = self.mmu.cpu().registers.y;
                let subtracted = y.wrapping_sub(value);
                self.update_flags_after_compare(y, value, subtracted);
            }
            InstructionType::BIT => {
                let value = self.read_8_bit_value(instruction);
                let registers = &mut self.mmu.cpu_mut().registers;
                let result = registers.a & value;
                registers.set_flags(Flags {
                    negative: (value & 0b10000000) != 0,
                    overflow: (value & 0b01000000) != 0,
                    zero: result == 0,
                    ..registers.flags()
                })
            }
            InstructionType::BCC => {
                if !self.mmu.cpu().registers.flags().carry {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BCS => {
                if self.mmu.cpu().registers.flags().carry {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BNE => {
                if !self.mmu.cpu().registers.flags().zero {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BEQ => {
                if self.mmu.cpu().registers.flags().zero {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BPL => {
                if !self.mmu.cpu().registers.flags().negative {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BMI => {
                if self.mmu.cpu().registers.flags().negative {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BVC => {
                if !self.mmu.cpu().registers.flags().overflow {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::BVS => {
                if self.mmu.cpu().registers.flags().overflow {
                    self.jump(instruction.addressing_mode);
                }
            }
            InstructionType::TAX => {
                let value = self.mmu.cpu().registers.a;
                self.mmu.cpu_mut().registers.x = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::TXA => {
                let value = self.mmu.cpu().registers.x;
                self.mmu.cpu_mut().registers.a = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::TAY => {
                let value = self.mmu.cpu().registers.a;
                self.mmu.cpu_mut().registers.y = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::TYA => {
                let value = self.mmu.cpu().registers.y;
                self.mmu.cpu_mut().registers.a = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::TSX => {
                let value = self.mmu.cpu().registers.s;
                self.mmu.cpu_mut().registers.x = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::TXS => {
                let value = self.mmu.cpu().registers.x;
                self.mmu.cpu_mut().registers.s = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::PHA => {
                let registers = &mut self.mmu.cpu_mut().registers;
                let value = registers.a;
                Stack::new(self.mmu.cpu_mut()).push(value);
            }
            InstructionType::PLA => {
                let value = Stack::new(self.mmu.cpu_mut()).pop();
                self.mmu.cpu_mut().registers.a = value;
                self.update_zero_and_negative_flags(value);
            }
            InstructionType::PHP => {
                let value = self.mmu.cpu().registers.p;
                Stack::new(self.mmu.cpu_mut()).push(value);
            }
            InstructionType::PLP => {
                let value = Stack::new(self.mmu.cpu_mut()).pop();
                self.mmu.cpu_mut().registers.p = value;
            }
            InstructionType::JMP => {
                let jump_address = self
                    .mmu
                    .address_by_mode(instruction.addressing_mode)
                    .expect("Invalid addressing mode");

                self.mmu.cpu_mut().registers.pc = jump_address;
            }
            InstructionType::JSR => match instruction.addressing_mode {
                AddressingMode::Absolute(address) => {
                    let return_address = self.mmu
                        .cpu()
                        .registers
                        .pc
                        .wrapping_add(instruction.addressing_mode.byte_length() as u16)
                        .wrapping_sub(1)
                        .to_le_bytes();
                    let mut stack = Stack::new(self.mmu.cpu_mut());
                    stack.push(return_address[1]);
                    stack.push(return_address[0]);
                    self.mmu.cpu_mut().registers.pc = address;
                }
                _ => panic!("Invalid addressing mode for JSR. JSR only support absolute addressing"),
            },
            InstructionType::RTS => {
                let mut stack = Stack::new(self.mmu.cpu_mut());
                let lo_word = stack.pop();
                let hi_word = stack.pop();
                self.mmu.cpu_mut().registers.pc = u16::from_le_bytes([lo_word, hi_word]).wrapping_add(1);
            }
            InstructionType::RTI => {
                let mut stack = Stack::new(self.mmu.cpu_mut());
                let flags = Flags::from(stack.pop());
                let lo_word = stack.pop();
                let hi_word = stack.pop();

                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(flags);
                registers.pc = u16::from_le_bytes([lo_word, hi_word]);
            }
            InstructionType::CLC => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    carry: false,
                    ..registers.flags()
                })
            }
            InstructionType::SEC => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    carry: true,
                    ..registers.flags()
                })
            }
            InstructionType::CLD => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    decimal: false,
                    ..registers.flags()
                })
            }
            InstructionType::SED => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    decimal: true,
                    ..registers.flags()
                })
            }
            InstructionType::CLI => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    interrupt_disable: false,
                    ..registers.flags()
                })
            }
            InstructionType::SEI => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    interrupt_disable: true,
                    ..registers.flags()
                })
            }
            InstructionType::CLV => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.set_flags(Flags {
                    overflow: false,
                    ..registers.flags()
                })
            }
            InstructionType::BRK => {
                let registers = &self.mmu.cpu().registers;
                let pc_address = registers.pc.to_le_bytes();
                let flags = registers.flags();

                let mut stack = Stack::new(self.mmu.cpu_mut());
                stack.push(pc_address[1]);
                stack.push(pc_address[0]);
                stack.push(flags.into());

                self.mmu.cpu_mut().registers.set_flags(Flags {
                    break_command: true,
                    ..flags
                });
            }
            InstructionType::NOP => (),
        }
    }

    fn read_8_bit_value(&self, instruction: Instruction) -> u8 {
        self.mmu
            .read_by_mode(instruction.addressing_mode)
            .expect("Failed to read by mode")
    }

    fn write_8_bit_value(&mut self, instruction: Instruction, value: u8) {
        self.mmu.write_8_bit_value_by_mode(instruction.addressing_mode, value);
    }

    fn update_flags_after_arithmetic(&mut self, old_a: u8, value: u8, carry: bool) {
        let registers = &mut self.mmu.cpu_mut().registers;
        let a_sign = Sign::from(registers.a);
        registers.set_flags(Flags {
            negative: a_sign == Sign::Negative,
            overflow: a_sign != Sign::from(old_a) && a_sign != Sign::from(value),
            zero: registers.a == 0,
            carry,
            ..registers.flags()
        });
    }

    fn update_flags_after_shift(&mut self, value: u8, carry: bool) {
        let registers = &mut self.mmu.cpu_mut().registers;
        registers.set_flags(Flags {
            negative: Sign::from(value) == Sign::Negative,
            zero: value == 0,
            carry,
            ..registers.flags()
        })
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        let registers = &mut self.mmu.cpu_mut().registers;
        registers.set_flags(Flags {
            zero: value == 0,
            negative: Sign::from(value) == Sign::Negative,
            ..registers.flags()
        })
    }

    fn update_flags_after_compare(&mut self, register_value: u8, memory_value: u8, result: u8) {
        let registers = &mut self.mmu.cpu_mut().registers;
        registers.set_flags(Flags {
            negative: (result & 0b10000000) != 0,
            zero: register_value == memory_value,
            carry: register_value >= memory_value,
            ..registers.flags()
        })
    }

    fn jump(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Relative(jump_offset) => {
                let registers = &mut self.mmu.cpu_mut().registers;
                registers.pc = match jump_offset.is_positive() {
                    true => registers.pc.wrapping_add(jump_offset as u16),
                    false => registers.pc.wrapping_sub(jump_offset.abs() as u16),
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
        error::InvalidOpCode,
        hardware::{
            cpu::{AddressingMode, Flags, CPU, MMU},
            memory::{Memory, Stack},
            ppu::PPU,
        },
    };

    fn execute_with_cpu(cpu: &mut CPU, instruction: Instruction) {
        InstructionExecutor::new(&mut MMU::new(cpu, &mut PPU::new(), None)).execute(instruction);
    }

    #[test]
    pub fn test_adc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x02);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x00);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x7F;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x80);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_adc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x80;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ADC, AddressingMode::Immediate(0xFF)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x7F);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x00);
        assert!(!flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x02);
        assert!(flags.carry);
        assert!(!flags.overflow);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sbc_overflow() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0xFF;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0xFF)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x00);
        assert!(!flags.carry);
        assert!(flags.overflow);
        assert!(!flags.negative);
        assert!(flags.zero);
    }

    #[test]
    pub fn test_sbc_carry_overflow() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x00;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SBC, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0xFF);
        assert!(flags.carry);
        assert!(flags.overflow);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_lda() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LDA, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ldx() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LDX, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ldy() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LDY, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.y, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_sta() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu)
            .execute(Instruction::new(InstructionType::STA, AddressingMode::Absolute(0x0200)));
        assert_eq!(mmu.read(0x0200), Some(0x01));
    }

    #[test]
    pub fn test_stx() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu)
            .execute(Instruction::new(InstructionType::STX, AddressingMode::Absolute(0x0200)));
        assert_eq!(mmu.read(0x0200), Some(0x01));
    }

    #[test]
    pub fn test_sty() {
        let mut cpu = CPU::new();
        cpu.registers.y = 0x01;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu)
            .execute(Instruction::new(InstructionType::STY, AddressingMode::Absolute(0x0200)));
        assert_eq!(mmu.read(0x0200), Some(0x01));
    }

    #[test]
    pub fn test_inc() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::INC, AddressingMode::Absolute(0x0200)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(MMU::new(&mut cpu, &mut PPU::new(), None).read(0x0200), Some(0x01));
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_inx() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::INX, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_iny() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::INY, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.y, 0x01);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dec() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::DEC, AddressingMode::Absolute(0x0200)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(MMU::new(&mut cpu, &mut PPU::new(), None).read(0x0200), Some(0xFF));
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dex() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::DEX, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, 0xFF);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_dey() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::DEY, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.y, 0xFF);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_asl() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ASL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_asl_saturating() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ASL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b11111110);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_asl_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b10000000;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ASL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b0);
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_lsr() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b10;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LSR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_lsr_saturating() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LSR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b01111111);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_lsr_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b00000001;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::LSR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b00);
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_rol() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_rol_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b11111110);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_rol_carry_over() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b00;
        cpu.registers.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROL, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_ror() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b10;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b01);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_ror_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b01111111);
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_ror_carry_over() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b00;
        cpu.registers.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ROR, AddressingMode::Accumulator),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10000000);
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_and() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::AND, AddressingMode::Immediate(0b10101010)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_ora() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b00000000;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::ORA, AddressingMode::Immediate(0b10101010)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_eor() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::EOR, AddressingMode::Immediate(0b01010101)),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.a, 0b10101010);
        assert!(flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_cmp_equals() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cmp_greater() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x00)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cmp_less() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CMP, AddressingMode::Immediate(0x02)),
        );

        let flags = cpu.registers.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_cpx_equals() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpx_greater() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x00)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpx_less() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPX, AddressingMode::Immediate(0x02)),
        );

        let flags = cpu.registers.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_cpy_equals() {
        let mut cpu = CPU::new();
        cpu.registers.y = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x01)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpy_greater() {
        let mut cpu = CPU::new();
        cpu.registers.y = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x00)),
        );

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(!flags.zero);
        assert!(flags.carry);
    }

    #[test]
    pub fn test_cpy_less() {
        let mut cpu = CPU::new();
        cpu.registers.y = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CPY, AddressingMode::Immediate(0x02)),
        );

        let flags = cpu.registers.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(!flags.carry);
    }

    #[test]
    pub fn test_bit_zero() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        mmu.write(0x0000, 0b00000000);
        InstructionExecutor::new(&mut mmu)
            .execute(Instruction::new(InstructionType::BIT, AddressingMode::ZeroPage(0x00)));

        let flags = cpu.registers.flags();
        assert!(!flags.negative);
        assert!(flags.zero);
        assert!(!flags.overflow);
    }

    #[test]
    pub fn test_bit_negative_and_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11111111;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        mmu.write(0x0000, 0b11000000);
        InstructionExecutor::new(&mut mmu)
            .execute(Instruction::new(InstructionType::BIT, AddressingMode::ZeroPage(0x00)));

        let flags = cpu.registers.flags();
        assert!(flags.negative);
        assert!(!flags.zero);
        assert!(flags.overflow);
    }

    #[test]
    pub fn test_bcc() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BCC, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bcs() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BCS, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bne() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BNE, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_beq() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            zero: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BEQ, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bpl() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BPL, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bmi() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            negative: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BMI, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bvc() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BVC, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_bvs() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            overflow: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BVS, AddressingMode::Relative(2)),
        );

        assert_eq!(cpu.registers.pc, 0x02);
    }

    #[test]
    pub fn test_tax() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TAX, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, cpu.registers.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_txa() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TXA, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, cpu.registers.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tay() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TAY, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.y, cpu.registers.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tya() {
        let mut cpu = CPU::new();
        cpu.registers.y = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TYA, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.y, cpu.registers.a);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.registers.s = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TSX, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, cpu.registers.s);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_txs() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0x01;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::TXS, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.x, cpu.registers.s);
        assert!(!flags.negative);
        assert!(!flags.zero);
    }

    #[test]
    pub fn test_pha() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.s = 0xFF;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu).execute(Instruction::new(InstructionType::PHA, AddressingMode::Implied));

        assert_eq!(mmu.cpu().registers.s, 0xFE);
        assert_eq!(mmu.read(0x01FF), Some(cpu.registers.a));
    }

    #[test]
    pub fn test_pla() {
        let mut cpu = CPU::new();
        cpu.registers.s = 0xFF;
        let mut ppu = PPU::new();

        Stack::new(&mut cpu).push(0x01);
        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu).execute(Instruction::new(InstructionType::PLA, AddressingMode::Implied));

        let flags = mmu.cpu().registers.flags();
        assert_eq!(mmu.cpu().registers.s, 0xFF);
        assert_eq!(mmu.read(0x01FF), Some(cpu.registers.a));
        assert!(!flags.zero);
        assert!(!flags.negative);
    }

    #[test]
    pub fn test_php() {
        let mut cpu = CPU::new();
        cpu.registers.s = 0xFF;
        cpu.registers.p = 0x01;
        let mut ppu = PPU::new();

        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu).execute(Instruction::new(InstructionType::PHP, AddressingMode::Implied));

        assert_eq!(mmu.cpu().registers.s, 0xFE);
        assert_eq!(mmu.read(0x01FF), Some(cpu.registers.p));
    }

    #[test]
    pub fn test_plp() {
        let mut cpu = CPU::new();
        cpu.registers.s = 0xFF;
        let mut ppu = PPU::new();

        Stack::new(&mut cpu).push(0x01);
        let mut mmu = MMU::new(&mut cpu, &mut ppu, None);
        InstructionExecutor::new(&mut mmu).execute(Instruction::new(InstructionType::PLP, AddressingMode::Implied));

        let flags = mmu.cpu().registers.flags();
        assert_eq!(mmu.cpu().registers.s, 0xFF);
        assert_eq!(mmu.read(0x01FF), Some(cpu.registers.p));
        assert!(!flags.zero);
        assert!(!flags.negative);
    }

    #[test]
    pub fn test_jmp() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::JMP, AddressingMode::Absolute(0x0600)),
        );

        assert_eq!(cpu.registers.pc, 0x0600);
    }

    #[test]
    pub fn test_jsr() {
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x0601;

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::JSR, AddressingMode::Absolute(0x1000)),
        );

        let mut stack = Stack::new(&mut cpu);
        assert_eq!(stack.pop(), 0x03);
        assert_eq!(stack.pop(), 0x06);
        assert_eq!(cpu.registers.pc, 0x1000);
    }

    #[test]
    pub fn test_rts() {
        let mut cpu = CPU::new();

        let mut stack = Stack::new(&mut cpu);
        stack.push(0x06);
        stack.push(0x00);

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::RTS, AddressingMode::Implied),
        );

        assert_eq!(cpu.registers.pc, 0x0601);
    }

    #[test]
    pub fn test_rti() {
        let mut cpu = CPU::new();

        let mut stack = Stack::new(&mut cpu);
        stack.push(0x06);
        stack.push(0x00);
        stack.push(0b00000011);
        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::RTI, AddressingMode::Implied),
        );

        let flags = cpu.registers.flags();
        assert_eq!(cpu.registers.pc, 0x0600);
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

        cpu.registers.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CLC, AddressingMode::Implied),
        );

        assert!(!cpu.registers.flags().carry);
    }

    #[test]
    pub fn test_sec() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SEC, AddressingMode::Implied),
        );

        assert!(cpu.registers.flags().carry);
    }

    #[test]
    pub fn test_cld() {
        let mut cpu = CPU::new();

        cpu.registers.set_flags(Flags {
            decimal: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CLD, AddressingMode::Implied),
        );

        assert!(!cpu.registers.flags().decimal);
    }

    #[test]
    pub fn test_sed() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SED, AddressingMode::Implied),
        );

        assert!(cpu.registers.flags().decimal);
    }

    #[test]
    pub fn test_cli() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            interrupt_disable: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CLI, AddressingMode::Implied),
        );

        assert!(!cpu.registers.flags().interrupt_disable);
    }

    #[test]
    pub fn test_sei() {
        let mut cpu = CPU::new();

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::SEI, AddressingMode::Implied),
        );

        assert!(cpu.registers.flags().interrupt_disable);
    }

    #[test]
    pub fn test_clv() {
        let mut cpu = CPU::new();
        cpu.registers.set_flags(Flags {
            overflow: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::CLV, AddressingMode::Implied),
        );

        assert!(!cpu.registers.flags().overflow);
    }

    #[test]
    pub fn test_brk() {
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x0600;
        cpu.registers.set_flags(Flags {
            carry: true,
            ..Default::default()
        });

        execute_with_cpu(
            &mut cpu,
            Instruction::new(InstructionType::BRK, AddressingMode::Implied),
        );

        let mut stack = Stack::new(&mut cpu);
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
        let flags = cpu.registers.flags();
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
            Some(Instruction::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteX(0xD010)
            ))
        );
    }

    #[test]
    pub fn test_lda_absolute_y_from_machine_node() {
        let lda = Instruction::from_machine_code(&[0xB9, 0x10, 0xD0]).unwrap();
        assert_eq!(
            lda,
            Some(Instruction::new(
                InstructionType::LDA,
                AddressingMode::AbsoluteY(0xD010)
            ))
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
            Some(Instruction::new(
                InstructionType::LDA,
                AddressingMode::IndexedIndirect(0xD0)
            ))
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
            Some(Instruction::new(
                InstructionType::LDA,
                AddressingMode::IndirectIndexed(0xD0)
            ))
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
