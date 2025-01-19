#![allow(warnings)]
extern crate lazy_static;
pub mod cpu;
mod bus;
mod opcodes;
mod tests;
mod cartridge;
mod trace;
mod ppu;
use crate::opcodes::*;
use crate::cpu::*;
use crate::bus::*;
use crate::cartridge::*;
use crate::trace::*;
use crate::test::*;
use crate::ppu::*;


fn main() {
    let bytes: Vec<u8> = std::fs::read("src/TestRoms/nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xC000;
    cpu.execute(move |cpu| {
        println!("{}", trace(cpu));
    });

}



// fn render_memory_dump(canvas: &mut Canvas<Window>, cpu: &CPU) {
//     // Example rendering logic for memory dump
//     let memory = cpu.get_memory();
//     for (i, &byte) in memory.iter().enumerate() {
//         let x = (i % 32) as i32 * 20;
//         let y = (i / 32) as i32 * 20;
//         // canvas.set_draw_color(Color::RGB(byte, byte, byte));
//         // canvas.fill_rect(Rect::new(x, y, 20, 20)).unwrap();
//     }
// }

// fn write_memory_dump_to_file(cpu: &CPU, filename: &str) -> std::io::Result<()> {
//     let memory = cpu.get_memory();
//     let mut file = File::create(filename)?;
//     for (i, &byte) in memory.iter().take(256).enumerate() {
//         writeln!(file, "0x{:04X}: 0x{:02X}", i, byte)?;
//     }
//     Ok(())
// }
