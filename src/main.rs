#![allow(warnings)]
extern crate lazy_static;
pub mod cpu;
mod opcodes;
mod test;

use crate::opcodes::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use crate::cpu::CPU;

#[macro_use]
extern crate bitflags;

fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.memory_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                println!("in here bitch");
                cpu.memory_write(0xff, 0x77);
                println!("memory: ");
                println!("{:x}", cpu.memory_read(0xff));

            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                println!("in here bitch 2");
                cpu.memory_write(0xff, 0x73);
                println!("memory: ");
                println!("{:x}", cpu.memory_read(0xff));
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                println!("in here bitch 3");
                cpu.memory_write(0xff, 0x61);
                println!("memory: ");
                println!("{:x}", cpu.memory_read(0xff));
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                println!("in here bitch 4");
                cpu.memory_write(0xff, 0x64);
                println!("memory: ");
                println!("{:x}", cpu.memory_read(0xff));
            }
            _ => {/* do nothing */}
        }
    }
}

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();
    
    // let mem_dump_window = video_subsystem
        // .window("Memory Dump", 640, 480)
        // .position_centered()
        // .build().unwrap();

    // let mut mem_dump_canvas = mem_dump_window.into_canvas().present_vsync().build().unwrap();
    
    let game_code = vec![
        JSR_ABS, 0x06, 0x06, // JMP(123) fn_1
        JSR_ABS, 0x38, 0x06, // fn_2
        JSR_ABS, 0x0d, 0x06, // fn_3 JMP(126)
        JSR_ABS, 0x2a, 0x06, // fn_4
        RTS_IMP, 
        LDA_IMM, 0x02,
        STA_0PGE, 0x02, 
        LDA_IMM, 0x04, 
        STA_0PGE, 0x03, 
        LDA_IMM, 0x11, 
        STA_0PGE, 0x10, 
        LDA_IMM, 0x10, 
        STA_0PGE, 0x12, 
        LDA_IMM, 0x0f, 
        STA_0PGE, 0x14, 
        LDA_IMM, 0x04, 
        STA_0PGE, 0x11, 
        STA_0PGE, 0x13, 
        STA_0PGE, 0x15, 
        RTS_IMP, 
        LDA_0PGE, 0xfe, 
        STA_0PGE, 0x00, 
        LDA_0PGE, 0xfe, 
        AND_IMM, 0x03, 
        CLC_IMP, 
        ADC_IMM, 0x02, 
        STA_0PGE, 0x01, 
        RTS_IMP, 
        JSR_ABS, 0x4d, 0x06, 
        JSR_ABS, 0x8d, 0x06,
        JSR_ABS, 0xc3, 0x06, 
        JSR_ABS, 0x19, 0x07, 
        JSR_ABS, 0x20, 0x07, 
        JSR_ABS, 0x2d, 0x07, 
        JMP_ABS, 0x38, 0x06, 
        LDA_0PGE, 0xff,
        CMP_IMM, 0x77, 
        BEQ_REL, 0x0d, 
        CMP_IMM, 0x64, 
        BEQ_REL, 0x14, 
        CMP_IMM, 0x73, 
        BEQ_REL, 0x1b, 
        CMP_IMM, 0x61, 
        BEQ_REL, 0x22, 
        RTS_IMP, 
        LDA_IMM, 0x04, 
        BIT_0PGE, 0x02, 
        BNE_REL, 0x26, 
        LDA_IMM, 0x01, 
        STA_0PGE, 0x02, 
        RTS_IMP, 
        LDA_IMM, 0x08, 
        BIT_0PGE, 0x02, 
        BNE_REL, 0x1b, 
        LDA_IMM, 0x02, 
        STA_0PGE, 0x02, 
        RTS_IMP, 
        LDA_IMM, 0x01,
        BIT_0PGE, 0x02, 
        BNE_REL, BPL_REL, 
        LDA_IMM, 0x04, 
        STA_0PGE, 0x02, 
        RTS_IMP, 
        LDA_IMM, 0x02, 
        BIT_0PGE, 0x02, 
        BNE_REL, 0x05,
        LDA_IMM, 0x08, 
        STA_0PGE, 0x02, 
        RTS_IMP, 
        RTS_IMP, 
        JSR_ABS, 0x94, 0x06, 
        JSR_ABS, 0xa8, 0x06, 
        RTS_IMP, 
        LDA_0PGE, 0x00,
        CMP_0PGE, BPL_REL, 
        BNE_REL, 0x0d, 
        LDA_0PGE, 0x01, 
        CMP_0PGE, 0x11, 
        BNE_REL, 0x07, 
        INC_0PGE, 0x03, 
        INC_0PGE, 0x03, 
        JSR_ABS, 0x2a, 0x06, 
        RTS_IMP, 
        LDX_IMM, 0x02, 
        LDA_0PGE_X, BPL_REL, 
        CMP_0PGE, BPL_REL, 
        BNE_REL, 0x06, 
        LDA_0PGE_X, 0x11, 
        CMP_0PGE, 0x11,
        BEQ_REL, 0x09, 
        INX_IMP, 
        INX_IMP, 
        CPX_0PGE, 0x03, 
        BEQ_REL, 0x06, 
        JMP_ABS, 0xaa, 0x06, 
        JMP_ABS, 0x35, 0x07, 
        RTS_IMP,
        LDX_0PGE, 0x03, 
        DEX_IMP, 
        TXA_IMP, 
        LDA_0PGE_X, BPL_REL, 
        STA_0PGE_X, 0x12, 
        DEX_IMP, 
        BPL_REL, 0xf9, 
        LDA_0PGE, 0x02, 
        LSR_ACC, 
        BCS_REL, 0x09, 
        LSR_ACC, 
        BCS_REL, 0x19, 
        LSR_ACC, 
        BCS_REL, 0x1f, 
        LSR_ACC, 
        BCS_REL, 0x2f, 
        LDA_0PGE, 0x10, 
        SEC_IMP, 
        SBC_IMM, 0x20,
        STA_0PGE, 0x10, 
        BCC_REL, 0x01, 
        RTS_IMP, 
        DEC_0PGE, 0x11, 
        LDA_IMM, 0x01, 
        CMP_0PGE, 0x11, 
        BEQ_REL, 0x28, 
        RTS_IMP, 
        INC_0PGE, 0x10, 
        LDA_IMM, 0x1f, 
        BIT_0PGE, 0x10, 
        BEQ_REL, 0x1f, 
        RTS_IMP, 
        LDA_0PGE, 0x10, 
        CLC_IMP, 
        ADC_IMM, 0x20, 
        STA_0PGE, 0x10,
        BCS_REL, 0x01, 
        RTS_IMP, 
        INC_0PGE, 0x11, 
        LDA_IMM, 0x06, 
        CMP_0PGE, 0x11, 
        BEQ_REL, 0x0c, 
        RTS_IMP, 
        DEC_0PGE, 0x10, 
        LDA_0PGE, 0x10, 
        AND_IMM, 0x1f, 
        CMP_IMM, 0x1f, 
        BEQ_REL, 0x01, 
        RTS_IMP, 
        JMP_ABS, 0x35, 0x07, 
        LDY_IMM, 0x00, 
        LDA_0PGE, 0xfe,
        STA_IND_Y, 0x00, 
        RTS_IMP, 
        LDX_0PGE, 0x03, 
        LDA_IMM, 0x00, 
        STA_IND_X, 0x10, 
        LDX_IMM, 0x00, 
        LDA_IMM, 0x01, 
        STA_IND_X, 0x10,
        RTS_IMP, 
        LDX_0PGE, 0xff, 
        NOP, 
        NOP, 
        DEX_IMP, 
        BNE_REL, 0xfb, 
        RTS_IMP,
    ];
    let mut iters = 0;

    //load the game
    let mut cpu = CPU::new();
    cpu.load(game_code);
    cpu.reset();

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    // run the game cycle
    cpu.execute(move |cpu| {
        handle_user_input(cpu, &mut event_pump);

        cpu.memory_write(0xfe, rng.gen_range(1, 16));

        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();

            canvas.copy(&texture, None, None).unwrap();

            canvas.present();
        }

        // mem_dump_canvas.clear();
        // render_memory_dump(&mut mem_dump_canvas, &cpu);
        if(iters % 1000 == 0) {
            write_memory_dump_to_file(&cpu, "memory_dump.txt").unwrap();
        }
        // mem_dump_canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 7_000));
        iters+=1;
    });

    
}

fn render_memory_dump(canvas: &mut Canvas<Window>, cpu: &CPU) {
    // Example rendering logic for memory dump
    let memory = cpu.get_memory();
    for (i, &byte) in memory.iter().enumerate() {
        let x = (i % 32) as i32 * 20;
        let y = (i / 32) as i32 * 20;
        // canvas.set_draw_color(Color::RGB(byte, byte, byte));
        // canvas.fill_rect(Rect::new(x, y, 20, 20)).unwrap();
    }
}

fn write_memory_dump_to_file(cpu: &CPU, filename: &str) -> std::io::Result<()> {
    let memory = cpu.get_memory();
    let mut file = File::create(filename)?;
    for (i, &byte) in memory.iter().take(256).enumerate() {
        writeln!(file, "0x{:04X}: 0x{:02X}", i, byte)?;
    }
    Ok(())
}
