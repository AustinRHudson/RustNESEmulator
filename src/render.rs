use rand::seq::index;

use crate::palette;
use crate::ppu::*;
use crate::palette::*;
use crate::frame::*;

fn bg_pallette(ppu: &ppu, tile_column: usize, tile_row : usize) -> [u8;4] {
    let attr_table_idx = tile_row / 4 * 8 +  tile_column / 4;
    let attr_byte = ppu.vram[0x3c0 + attr_table_idx];  // note: still using hardcoded first nametable
 
    let pallet_idx = match (tile_column %4 / 2, tile_row % 4 / 2) {
        (0,0) => attr_byte & 0b11,
        (1,0) => (attr_byte >> 2) & 0b11,
        (0,1) => (attr_byte >> 4) & 0b11,
        (1,1) => (attr_byte >> 6) & 0b11,
        (_,_) => panic!("should not happen"),
    };
 
    let pallete_start: usize = 1 + (pallet_idx as usize)*4;
    return [ppu.palette_table[0], ppu.palette_table[pallete_start], ppu.palette_table[pallete_start+1], ppu.palette_table[pallete_start+2]];
 }
 

 pub fn render(ppu: &ppu, frame: &mut Frame) {
    let bank = ppu.control_register.background_pattern_addr();
    //background
    for i in 0..0x3c0 {
        let tile = ppu.vram[i] as u16;
        let tile_column = i % 32;
        let tile_row = i / 32;
        let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
        let palette = bg_pallette(ppu, tile_column, tile_row);
 
        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];
 
            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => palette::SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => palette::SYSTEM_PALLETE[palette[1] as usize],
                    2 => palette::SYSTEM_PALLETE[palette[2] as usize],
                    3 => palette::SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(tile_column * 8 + x, tile_row * 8 + y, rgb)
            }
        }
    }

    //sprites
    for i in (0..ppu.oam_data.len()).step_by(4){
        let tile_y = ppu.oam_data[i] as usize;
        let tile_x = ppu.oam_data[i + 3] as usize;
        let index_number = ppu.oam_data[i + 1] as u16;
        let attributes = ppu.oam_data[i + 2];

        let mut flip_vertical = false;
        let mut flip_horizontal = false; 

        if(attributes & 0b1000_0000 == 0b1000_0000){
            flip_vertical = true;
        }

        if(attributes & 0b0100_0000 == 0b0100_0000){
            flip_horizontal = true;
        }

        let pallette_idx = ppu.oam_data[i + 2] & 0b11;
        let sprite_palette = sprite_palette(ppu, pallette_idx);

        let bank = ppu.control_register.sprite_pattern_addr();

        let tile = &ppu.chr_rom[(bank + index_number * 16) as usize..=(bank + index_number * 16 + 15) as usize];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];
            'ololo: for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => continue 'ololo, // skip coloring the pixel
                    1 => palette::SYSTEM_PALLETE[sprite_palette[1] as usize],
                    2 => palette::SYSTEM_PALLETE[sprite_palette[2] as usize],
                    3 => palette::SYSTEM_PALLETE[sprite_palette[3] as usize],
                    _ => panic!("can't be"),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                }
            }
        }
    }
 }

 fn sprite_palette(ppu: &ppu, pallete_idx: u8) -> [u8; 4] {
    let start = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}
 

