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
use simplelog::{SimpleLogger, LevelFilter, Config};

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    let args: Vec<String> = env::args().collect();

    let mut rom_file = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .expect("Failed to read ROM file to end");
    let rom = ROM::from_slice(buffer.as_slice()).unwrap();
    let mapper = MemoryMapper::NROM(&rom.prg_rom()[0x0000..PRG_PAGE_SIZE], &rom.prg_rom()[PRG_PAGE_SIZE..]);

    let mut cpu = CPU::with_power_up_state();
    cpu.registers.pc = u16::from_le_bytes([mapper.read(0xFFFC).unwrap(), mapper.read(0xFFFD).unwrap()]);

    let mut ppu = Default::default();

    loop {
        match Instruction::from_machine_code(mapper.slice_from(cpu.registers.pc).unwrap()) {
            Ok(Some(instruction)) => {
                InstructionExecutor::new(&mut MMU::new(&mut cpu, &mut ppu, Some(&mapper))).execute(instruction);
                if instruction.instruction_type.increments_pc() {
                    cpu.registers.pc += instruction.addressing_mode.byte_length() as u16;
                }
            }
            Ok(None) => break,
            Err(err) => {
                println!("Error at offset {:#X}. {}", cpu.registers.pc, err);
                break;
            }
        }

        ppu.clock.step();
        // println!("ppu clock: {:?}", ppu.clock);
        ppu.update();
    }

    println!("End");
}
