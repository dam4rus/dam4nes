mod error;
mod hardware;
mod instruction;
mod rom;

use hardware::{
    cpu::{CPU, MMU},
    memory::MemoryMapper,
};
use instruction::{Instruction, InstructionExecutor};
use rom::{PRG_PAGE_SIZE, ROM};
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rom_file = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .expect("Failed to read ROM file to end");
    let rom = ROM::from_slice(buffer.as_slice()).unwrap();
    let mapper = MemoryMapper::NROM(&rom.prg_rom()[0x0000..PRG_PAGE_SIZE], &rom.prg_rom()[PRG_PAGE_SIZE..]);

    let mut cpu = CPU::new();
    cpu.registers.p = 0x34;
    cpu.registers.s = 0xFD;
    cpu.registers.pc = u16::from_le_bytes([mapper.read(0xFFFC).unwrap(), mapper.read(0xFFFD).unwrap()]);

    loop {
        match Instruction::from_machine_code(mapper.slice_from(cpu.registers.pc).unwrap()) {
            Ok(Some(instruction)) => {
                InstructionExecutor::new(&mut MMU::new(&mut cpu, Some(&mapper))).execute(instruction)
            }
            Ok(None) => break,
            Err(err) => {
                println!("Error at offset {:#X}. {}", cpu.registers.pc, err);
                break;
            }
        }
    }

    println!("End");
}
