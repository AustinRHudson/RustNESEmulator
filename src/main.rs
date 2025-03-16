#![allow(warnings)]
extern crate lazy_static;
pub mod cpu;
mod bus;
mod opcodes;
mod tests;
mod cartridge;
mod trace;
mod ppu;
mod tile_viewer;
mod render;
mod frame;
mod palette;
mod joypad;
mod apu;
use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::KeyCode;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use crate::opcodes::*;
use crate::cpu::*;
use crate::bus::*;
use crate::cartridge::*;
use crate::trace::*;
use crate::test::*;
use crate::ppu::ppu as NesPPU;
use crate::tile_viewer::*;
use crate::render::*;
use crate::frame::*;
use crate::joypad::*;
use crate::apu::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    //load game
    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, joypad::JoypadButtons::DOWN);
    key_map.insert(Keycode::Up, joypad::JoypadButtons::UP);
    key_map.insert(Keycode::Right, joypad::JoypadButtons::RIGHT);
    key_map.insert(Keycode::Left, joypad::JoypadButtons::LEFT);
    key_map.insert(Keycode::Space, joypad::JoypadButtons::SELECT);
    key_map.insert(Keycode::Return, joypad::JoypadButtons::START);
    key_map.insert(Keycode::X, joypad::JoypadButtons::BUTTON_A);
    key_map.insert(Keycode::Z, joypad::JoypadButtons::BUTTON_B);

    let bytes: Vec<u8> = std::fs::read("src/TestRoms/pacman.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut Joypad| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => std::process::exit(0),

              Event::KeyDown { keycode, .. } => {
                if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    joypad.set_button_pressed_status(*key, true);
                    //println!("{:?}", *key);
                }
            }
            Event::KeyUp { keycode, .. } => {
                if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    joypad.set_button_pressed_status(*key, false);
                }
            }

              _ => { /* do nothing */ }
            }
         }
    });
    let mut cpu = CPU::new(bus);
    cpu.reset();
    //cpu.program_counter = 0xc000;
    cpu.execute(move |cpu| {
        //println!("{}", trace(cpu));
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
