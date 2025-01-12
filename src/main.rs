#![allow(warnings)]
extern crate lazy_static;
pub mod cpu;
mod bus;
mod opcodes;
mod test;
mod cartridge;
use crate::opcodes::*;
use crate::cpu::Mem;
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
                cpu.memory_write(0xff, 0x77);

            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cpu.memory_write(0xff, 0x73);
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.memory_write(0xff, 0x61);
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.memory_write(0xff, 0x64);
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
        JSR_ABS, 0x06, 0x06, // jmp init
        JSR_ABS, 0x38, 0x06, // jmp loop
        //init:
        JSR_ABS, 0x0d, 0x06, // jmp initSnake
        JSR_ABS, 0x2a, 0x06, // jmp generateApple
        RTS_IMP, //return
        // initSnake:
        LDA_IMM, 0x02, // move direction
        STA_0PGE, 0x02, 
        LDA_IMM, 0x04, // snake length
        STA_0PGE, 0x03, 
        LDA_IMM, 0x11, 
        STA_0PGE, 0x10, // initial snake head location, least significant bytes (0x10 is lo for head)
        LDA_IMM, 0x10, 
        STA_0PGE, 0x12, // initial snake body location, least significant bytes
        LDA_IMM, 0x0f, 
        STA_0PGE, 0x14, // also snake body, least significant bytes
        LDA_IMM, 0x04,  
        STA_0PGE, 0x11, // most signifcant bytes of the three things described above
        STX_0PGE, 0x13, 
        STA_0PGE, 0x15, 
        RTS_IMP, // return
        // generateApple:
        //The least significant byte of the apple position will determine where
        //in a 8x32 strip the apple is placed. This number can be any one byte value because
        //the size of one 8x32 strip fits exactly in one out of 256 bytes
        LDA_0PGE, 0xfe, // 0xfe contains a rand number
        STA_0PGE, 0x00, // new rng 
        LDA_0PGE, 0xfe, // this random determines which strip of 8x32 the apple will be placed.
        AND_IMM, 0x03, // take the last two values of the random number to determine a value between 0-3
        CLC_IMP, 
        ADC_IMM, 0x02, // add 2 to get a number between 2-5
        STA_0PGE, 0x01, //store y coordinate
        RTS_IMP, // return to "jmp generateApple"
        //loop:
        JSR_ABS, 0x4d, 0x06, //jmp readKeypress
        JSR_ABS, 0x8d, 0x06, //jmp checkCollision
        JSR_ABS, 0xc3, 0x06, //jmp updateSanke
        JSR_ABS, 0x19, 0x07, //jmp drawApple
        JSR_ABS, 0x20, 0x07, //jmp drawSnake
        JSR_ABS, 0x2d, 0x07, //jmp spinWheels
        JMP_ABS, 0x38, 0x06, //jmp loop
        //readKepress:
        LDA_0PGE, 0xff, // loads latest movement press
        CMP_IMM, 0x77, // w
        BEQ_REL, 0x0d, 
        CMP_IMM, 0x64, // a
        BEQ_REL, 0x14, 
        CMP_IMM, 0x73, // s 
        BEQ_REL, 0x1b, 
        CMP_IMM, 0x61, // d
        BEQ_REL, 0x22, 
        RTS_IMP, 
        //   0b01 // 1
        // & 0b10 // 2
        // -------
        //   0000 // 0, zero flag = 1
        // BNE, is zero flag false? then branch

        //   0b10 // 2
        // & 0b10 // 2
        // -------
        //   0010 // 2, zero flag = 0
        // BNE, is zero flag false? then branch
        // sure. they are equal. so instruction say
        // do not branch. why? because they are equal
        // LIES...BRANCH. zero flag is false, so we must branch

        LDA_IMM, 0x04, // loads the movement direction for down into the accumlator and compares
        //it to the current movement direction at 0x02. If the current direction is down and you
        //tried to go up that would be an illegal move in snake because you can not got down from up
        //so it branches to an illegal move part of the code. Otherwise set the movement direction to up
            BIT_0PGE, 0x02, 
            BNE_REL, 0x26, //branches to illegalMove
            LDA_IMM, 0x01, //set movement direction to up
            STA_0PGE, 0x02, 
            RTS_IMP, //return
        LDA_IMM, 0x08, //loads left and makes sure that you're not going left so you can go right
            BIT_0PGE, 0x02, 
            BNE_REL, 0x1b, 
            LDA_IMM, 0x02, 
            STA_0PGE, 0x02, 
            RTS_IMP, //return
        LDA_IMM, 0x01, //loads up and makes sure that you're not going up so you can go down
            BIT_0PGE, 0x02, 
            BNE_REL, 0x10, 
            LDA_IMM, 0x04, 
            STA_0PGE, 0x02, 
            RTS_IMP, //return
        LDA_IMM, 0x02, //loads right and makes sure that you're not going right so you can go left
            BIT_0PGE, 0x02, 
            BNE_REL, 0x05, // <-- fake!!! really a BEQ
            LDA_IMM, 0x08, 
            STA_0PGE, 0x02, 
            RTS_IMP, //return
        //illegalMove:
        //just returns if the move is illegal and does nothing
        RTS_IMP, //return
        //checkCollision:
        JSR_ABS, 0x94, 0x06, //jmp checkAppleCollision 
        JSR_ABS, 0xa8, 0x06, //jmp checkSnakeCollision
        RTS_IMP, //return
        //checkAppleCollision:
        LDA_0PGE, 0x00, //loads least significant byte of apples location
        CMP_0PGE, 0x10, //compares least significant byte of apples location to least significant byte o    f the snakes head 
        BNE_REL, 0x0d, //if not equal then branch to doneAppleCollision
        LDA_0PGE, 0x01, //loads most significant byte of apples location
        CMP_0PGE, 0x11, //compares most significant byte of apples location to most significant byte of the snakes head
        BNE_REL, 0x07, //if not equal then branch to doneAppleCollision
        INC_0PGE, 0x03, //if it made it here then increase length of snake twice
        INC_0PGE, 0x03, 
        JSR_ABS, 0x2a, 0x06, //jmp generateApple
        //doneAppleCollision:
        RTS_IMP, //return
        //checkSnakeCollision:
        LDX_IMM, 0x02, //loads 2 into x register because we start with the head
        //snakeCollisionLoop:
        LDA_0PGE_X, 0x10, //load the value at address 0x10 (the least significant byte of the snakes head) plus x to get the least significant byte in the next snake segment
        CMP_0PGE, 0x10, //compare the lesat significant bytes of the snake segment just loaded and the snakes head
        BNE_REL, 0x06, //branch if they are not equal to continueCollisionLoop
        //maybeCollided:
        LDA_0PGE_X, 0x11, //now we load the most significant byte of the snakes segment we are checking
        CMP_0PGE, 0x11, //compare it to the most significant byte of the snakes head
        BEQ_REL, 0x09, //branch if they are equal to didCollide
        //continueCollisionLoop:
        INX_IMP, //increments X to continue looking through snake segments
        INX_IMP, 
        CPX_0PGE, 0x03, //compare x register to the value stores at addres 0x03 (snake length)
        BEQ_REL, 0x06,  //if equal that means we have looked at each segment so we are done, branch to didntCollide
        JMP_ABS, 0xaa, 0x06, //else we have more to check so jump back to the start of the loop, jmp snakeCollisionLoop
        //didCollide:
        JMP_ABS, 0x35, 0x07, //jmp gameOver 
        //didntCollide:
        RTS_IMP, //return
        //updateSnake:
        LDX_0PGE, 0x03, //loads value of snake length into X
        DEX_IMP, //decreases length by 1 in x
        TXA_IMP, //transgers x to a, not sure why honestly
        //updateLoop:
        //shifts all segmets of snake forward 2 places in memory (2 bytes for each segmemt)
        LDA_0PGE_X, 0x10, //load the first segment of the snake 
        STA_0PGE_X, 0x12, //store a register into register 2 addresses down (move the segment)
        DEX_IMP,  
        BPL_REL, 0xf9, //if positive branch back to updateLoop to move more segments
        LDA_0PGE, 0x02, //load direction of snake into A
        LSR_ACC, 
        BCS_REL, 0x09, //check if right shift cut off the 1 bit, if so branch to up
        LSR_ACC, 
        BCS_REL, 0x19, //check if right shift cut off the 1 bit, if so branch to right
        LSR_ACC, 
        BCS_REL, 0x1f, //check if right shift cut off the 1 bit, if so branch to down
        LSR_ACC, 
        BCS_REL, 0x2f, //check if right shift cut off the 1 bit, if so branch to left
        //up:
        LDA_0PGE, 0x10, 
        SEC_IMP, 
        SBC_IMM, 0x20,
        STA_0PGE, 0x10, 
        BCC_REL, 0x01, //branch if carry cleared from sbc due to overflow, to upup
        RTS_IMP, 
        //upup:
        DEC_0PGE, 0x11, 
        LDA_IMM, 0x01, 
        CMP_0PGE, 0x11, 
        BEQ_REL, 0x28, 
        RTS_IMP, 
        //right:
        INC_0PGE, 0x10, 
        LDA_IMM, 0x1f, 
        BIT_0PGE, 0x10, 
        BEQ_REL, 0x1f, //branch to collision
        RTS_IMP,
        //down: 
        LDA_0PGE, 0x10, 
        CLC_IMP, 
        ADC_IMM, 0x20, 
        STA_0PGE, 0x10,
        BCS_REL, 0x01, //branch to downdown
        RTS_IMP, 
        //downdown:
        INC_0PGE, 0x11, 
        LDA_IMM, 0x06, 
        CMP_0PGE, 0x11, 
        BEQ_REL, 0x0c, //branch to collision
        RTS_IMP, 
        //left:
        DEC_0PGE, 0x10, 
        LDA_0PGE, 0x10, 
        AND_IMM, 0x1f, 
        CMP_IMM, 0x1f, 
        BEQ_REL, 0x01, //branch collision
        RTS_IMP, 
        //collision:
        JMP_ABS, 0x35, 0x07, //jmp gameOver
        //drawApple:
        LDY_IMM, 0x00, 
        LDA_0PGE, 0xfe,
        STA_IND_Y, 0x00, 
        RTS_IMP, 
        //drawSnake:
        LDX_0PGE, 0x03, 
        LDA_IMM, 0x00, 
        STA_IND_X, 0x10, 
        LDX_IMM, 0x00, 
        LDA_IMM, 0x01, 
        STA_IND_X, 0x10,
        RTS_IMP, 
        //spinWheels:
        LDX_0PGE, 0xff, 
        //spinLoop
        NOP, 
        NOP, 
        DEX_IMP, 
        BNE_REL, 0xfb, //branch spinLoop
        RTS_IMP,
        //gameOver:
    ];
    let mut iters = 0;

    //load the game
    let mut cpu = CPU::new();
    cpu.load(game_code, 0x0600);
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
        // if(iters % 1000 == 0) {
        //     write_memory_dump_to_file(&cpu, "memory_dump.txt").unwrap();
        // }
        // mem_dump_canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 70_000));
        iters+=1;
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
