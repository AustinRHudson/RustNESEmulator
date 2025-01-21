use crate::cpu::*;
use crate::cartridge::*;
use crate::joypad;
use crate::ppu::*;
use crate::render::*;
use crate::frame::*;
use crate::joypad::*;

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
pub struct Bus <'call>{
	cpu_vram: [u8; 2048],
	prg_rom: Vec<u8>,
    ppu: ppu,
    cycles: usize,
    gameloop_callback: Box<dyn FnMut(&ppu, &mut Joypad) + 'call>,
    joypad: Joypad,
}

impl <'a>Bus<'a> {
	pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&ppu, &mut Joypad) + 'call,
    {
        let ppu = ppu::new(rom.chr_rom, rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
            joypad: Joypad::new()
        }
    }

	fn read_prg_rom(&self, mut addr: u16) -> u8 {
		addr -= 0x8000;
		if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
				//mirror if needed
				addr = addr % 0x4000;
		}
		self.prg_rom[addr as usize]
    }

    pub fn tick(&mut self, ticks: u8){
        self.cycles += ticks as usize;
        let nmi_before = self.ppu.nmi_interrupt.is_some();
        self.ppu.tick(ticks * 3);
        let nmi_after = self.ppu.nmi_interrupt.is_some();
        if(!nmi_before && nmi_after){
            (self.gameloop_callback)(&self.ppu, &mut self.joypad);
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8>{
        return self.ppu.nmi_interrupt.take();
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF; // 0x1FFF
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF; // 0x3FFF

impl Mem for Bus<'_> {
   fn memory_read(&mut self, addr: u16) -> u8 {
    //println!("{:x}", addr);
       match addr {
        
           RAM ..= RAM_MIRRORS_END => {
               let mirror_down_addr = addr & 0b0000_0111_1111_1111;
               self.cpu_vram[mirror_down_addr as usize]
           }

           0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
            panic!("Attempt to read from write-only PPU address {:x}", addr);
            0
            }

            0x2002 => {
                return self.ppu.read_status_register();
            }

            0x2004 => {
                return self.ppu.read_oam_data();
            }

            0x2007 => self.ppu.read_data(),

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.memory_read(mirror_down_addr)
            }

            0x4016 => {
                return self.joypad.read_joypad();
            }

            0x4017 => {
                0
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

           0x2000 => {
                self.ppu.write_control_register(data);
           }

           0x2001 => {
            self.ppu.write_mask_register(data);
           }

           0x2002 => {
            panic!("attempting to write to read only register");
           }

           0x2003 => {
            self.ppu.write_oam_address(data);
           }

           0x2004 => {
            self.ppu.write_oam_data(data);
           }

           0x2005 => {
            self.ppu.write_scroll_register(data);
           }

           0x2006 => {
                self.ppu.write_ppu_address(data);
           }

           0x2007 => {
                self.ppu.write_data(data);
           }

           0x4000..=0x4013 | 0x4015 => {
            //ignore APU 
        }

        0x4016 => {
            self.joypad.write_joypad(data);
        }

        0x4017 => {
            // ignore joypad 2
        }

           0x2008 ..= PPU_REGISTERS_MIRRORS_END => {
               let _mirror_down_addr = addr & 0b00100000_00000111;
               self.memory_write(_mirror_down_addr, data);
               //todo!("PPU is not supported yet");
           }

           0x4014 => {
            let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.memory_read(hi + i);
                }

                self.ppu.write_oam_dma(&buffer);

           }

            0x8000..=0xFFFF => {
						//panic!("Attempt to write to Cartridge ROM space")
			}

           _ => {
               println!("Ignoring mem write-access at {:x}", addr);
           }
       }
   }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cartridge::test;

    // #[test]
    // fn test_mem_read_write_to_ram() {
    //     let mut frame = Frame::new();
    //     let mut bus = Bus::new(test::test_rom(vec![]);
    //     bus.memory_write(0x01, 0x55);
    //     assert_eq!(bus.memory_read(0x01), 0x55);
    // }
}