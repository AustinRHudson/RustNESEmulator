use crate::cpu::*;
use crate::cartridge::*;

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|
pub struct Bus {
	cpu_vram: [u8; 2048],
	rom: Rom,
}

impl Bus {
	pub fn new(rom: Rom) -> Self{
		Bus {
			cpu_vram: [0; 2048],
			rom: rom,
		}
	}

	fn read_prg_rom(&self, mut addr: u16) -> u8 {
		addr -= 0x8000;
		if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
				//mirror if needed
				addr = addr % 0x4000;
		}
		self.rom.prg_rom[addr as usize]
}
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0b0001_1111_1111_1111; // 0x1FFF
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0b0011_1111_1111_1111; // 0x3FFF

impl Mem for Bus {
   fn memory_read(&self, addr: u16) -> u8 {
       match addr {
        
           RAM ..= RAM_MIRRORS_END => {
               let mirror_down_addr = addr & 0b0000_0111_1111_1111;
               self.cpu_vram[mirror_down_addr as usize]
           }

           PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => {
               let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
               todo!("PPU is not supported yet")
           }

			0x8000..= 0xFFFF => self.read_prg_rom(addr),

           _ => {
               println!("Ignoring mem access at {:x}", addr);
               0
           }
       }
   }

   fn memory_write(&mut self, addr: u16, data: u8) {
       match addr {

           RAM ..= RAM_MIRRORS_END => {
               let mirror_down_addr = addr & 0b11111111111;
               self.cpu_vram[mirror_down_addr as usize] = data;
           }

           PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => {
               let _mirror_down_addr = addr & 0b00100000_00000111;
               todo!("PPU is not supported yet");
           }

            0x8000..=0xFFFF => {
						panic!("Attempt to write to Cartridge ROM space")
			}

           _ => {
               println!("Ignoring mem write-access at {:x}", addr);
           }
       }
   }
}
