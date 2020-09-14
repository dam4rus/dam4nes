mod error;
mod hardware;
mod instruction;
mod rom;

use hardware::{
    cpu::{CPU, MMU as CPUMMU},
    memory::MemoryMapper,
    ppu::{PPU, MMU as PPUMMU, NameTables, NameTable, PatternTables, PATTERN_TABLE_SECTION_SIZE},
};
use instruction::{Instruction, InstructionExecutor};
use rom::{PRG_PAGE_SIZE, ROM};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
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
    let rect_scale: i32 = 3;
    let window = video_subsystem
        .window("dam4nes", (256 * rect_scale + 30) as u32, (256 * rect_scale + 30) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut bitmap = [[0u8; 256]; 256];

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
                InstructionExecutor::new(&mut CPUMMU::new(&mut cpu, &mut ppu, Some(&mapper))).execute(instruction);
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
        let pattern_tables = PatternTables::new(
            &rom.chr_rom()[..PATTERN_TABLE_SECTION_SIZE],
            &rom.chr_rom()[PATTERN_TABLE_SECTION_SIZE..(PATTERN_TABLE_SECTION_SIZE * 2)],
        ).unwrap();

        let old_vblank = ppu.registers.status_flags().vblank;
        ppu.update(pattern_tables, &mut bitmap);
        if !old_vblank && ppu.registers.status_flags().vblank {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();
            for y in 0..256 {
                for x in 0..256 {
                    canvas.set_draw_color(match bitmap[y][x] {
                        0 => Color::RGB(0, 0, 0),
                        1 => Color::RGB(255, 0, 0),
                        2 => Color::RGB(0, 255, 0),
                        3 => Color::RGB(0, 0, 255),
                        _ => Color::RGB(255, 255, 255),
                    });
                    canvas.fill_rect(Rect::new(x as i32 * rect_scale, y as i32 * rect_scale, rect_scale as u32, rect_scale as u32)).unwrap();
                }
            }

            canvas.present();
        }


        //println!("");
    }
}
