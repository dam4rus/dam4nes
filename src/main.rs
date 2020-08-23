mod error;
mod hardware;
mod instruction;
mod rom;

use hardware::{
    cpu::{CPU, MMU},
    memory::MemoryMapper,
    ppu::PPU,
};
use instruction::{Instruction, InstructionExecutor};
use rom::{PRG_PAGE_SIZE, ROM};
use sdl2::{event::Event, keyboard::Keycode};
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::{env, fs::File, io::Read};

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    let args: Vec<String> = env::args().collect();

    let mut rom_file = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .expect("Failed to read ROM file to end");
    let rom = ROM::with_content(buffer).unwrap();
    let mapper = MemoryMapper::NROM(&rom.prg_rom()[0x0000..PRG_PAGE_SIZE], &rom.prg_rom()[PRG_PAGE_SIZE..]);

    let mut cpu = CPU::with_power_up_state();
    cpu.registers.pc = u16::from_le_bytes([mapper.read(0xFFFC).unwrap(), mapper.read(0xFFFD).unwrap()]);

    let mut ppu = PPU::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem
        .window("dam4nes", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }

        match Instruction::from_machine_code(mapper.slice_from(cpu.registers.pc).unwrap()) {
            Ok(Some(instruction)) => {
                InstructionExecutor::new(&mut MMU::new(&mut cpu, &mut ppu, Some(&mapper))).execute(instruction);
                if instruction.instruction_type.increments_pc() {
                    cpu.registers.pc += instruction.addressing_mode.byte_length() as u16;
                }
            }
            Ok(None) => break 'running,
            Err(err) => {
                println!("Error at offset {:#X}. {}", cpu.registers.pc, err);
                break 'running;
            }
        }

        ppu.clock.step();
        // println!("ppu clock: {:?}", ppu.clock);
        ppu.update();
    }
}
