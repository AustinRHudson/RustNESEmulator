use crate::Mirroring;
use bitflags::bitflags;

pub struct ppu {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub address: address_register,
    pub control_register: ControlRegister,
    internal_buffer: u8,
}

impl ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        ppu {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
            address: address_register{value: (0,0), hi_ptr: true},
            control_register: ControlRegister{bits: 0},
            internal_buffer: 0,
        }
    }

    pub fn write_ppu_address(&mut self, data: u8){
        self.address.update(data);
    }

    pub fn write_control_register(&mut self, data: u8){
        self.control_register.update(data);
    }

    fn increment_vram_address(&mut self){
        self.address.increment(self.control_register.vram_addr_increment());
    }

   pub fn mirror_vram_address(&self, addr: u16) -> u16 {
        // Horizontal:
        //   [ A ] [ a ]
        //   [ B ] [ b ]
        
        // Vertical:
        //   [ A ] [ B ]
        //   [ a ] [ b ]

        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn read_data(&mut self) -> u8{
        let address = self.address.get();
        self.control_register.vram_addr_increment();

        match address {
            0..0x1FFF => {
                let old_result = self.internal_buffer;
                self.internal_buffer = self.chr_rom[address as usize];
                return old_result;
            }

            0x2000..0x2FFF => {
                let old_result = self.internal_buffer;
                self.internal_buffer = self.vram[self.mirror_vram_address(address) as usize];
                return old_result;
            }

            0x3000..0x3EFF => {
                panic!("writing to mirror address space");
            }

            0x3F00..0x3FFF => {
                todo!("writing to palette table");
            }

            _ => {
                panic!("writing to mirror address spaces");
            }
        }
    }

    pub fn write_data(&mut self, data: u8){
        let address = self.address.get();
        

        match address {
            0..0x1FFF => {
                panic!("attemp to write to chr_rom space");
            }

            0x2000..0x2FFF => {
                self.vram[self.mirror_vram_address(address) as usize] = data; 
            }

            0x3000..0x3EFF => {
                panic!("writing to mirror address space");
            }

            0x3F00..0x3FFF => {
                todo!("writing to palette table");
            }

            _ => {
                panic!("writing to mirror address spaces");
            }
        }
    }
}

pub struct address_register {
    value: (u8, u8),
    hi_ptr: bool,
}

impl address_register {
    pub fn new() -> Self {
        address_register {
            value: (0, 0), // high byte first, lo byte second
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16){
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0x00FF) as u8;
    }

    pub fn update(&mut self, data: u8){
        if(self.hi_ptr){
            self.value.0 = data;
        } else{
            self.value.1 = data;
        }
        if self.get() > 0x3fff { //mirror down addr above 0x3fff
            self.set(self.get() & 0b11111111111111);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8){
        let lo = self.value.1;
        self.value.1 = lo.wrapping_add(inc);
        if(lo > self.value.1){ //check for overflow
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if (self.get() > 0x3FFF){
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_ptr(&mut self){
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16{
        return ((self.value.0 as u16) << 8) | (self.value.1 as u16);
    }

}

bitflags!{
// 7  bit  0
   // ---- ----
   // VPHB SINN
   // |||| ||||
   // |||| ||++- Base nametable address
   // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   // |||| |     (0: add 1, going across; 1: add 32, going down)
   // |||| +---- Sprite pattern table address for 8x8 sprites
   // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   // |||+------ Background pattern table address (0: $0000; 1: $1000)
   // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
   // |+-------- PPU master/slave select
   // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   // +--------- Generate an NMI at the start of the
   //            vertical blanking interval (0: off; 1: on)
   pub struct ControlRegister: u8 {
    const NAMETABLE1              = 0b00000001;
    const NAMETABLE2              = 0b00000010;
    const VRAM_ADD_INCREMENT      = 0b00000100;
    const SPRITE_PATTERN_ADDR     = 0b00001000;
    const BACKROUND_PATTERN_ADDR  = 0b00010000;
    const SPRITE_SIZE             = 0b00100000;
    const MASTER_SLAVE_SELECT     = 0b01000000;
    const GENERATE_NMI            = 0b10000000;
    }
}

impl ControlRegister {
    pub fn new() -> Self{
        ControlRegister::from_bits_truncate(0b00000000)
    }

    pub fn vram_addr_increment(&self) -> u8{
        if(self.contains(ControlRegister::VRAM_ADD_INCREMENT)){
            return 32;
        }else{
            return 1
        }
    }

    pub fn update(&mut self, data: u8){
        self.bits = data;
    }
}
