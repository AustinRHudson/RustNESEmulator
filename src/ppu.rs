use crate::Mirroring;
use bitflags::bitflags;
use crate::cartridge::*;

pub struct ppu {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub address: address_register,
    pub control_register: ControlRegister,
    internal_buffer: u8,
    pub mask_register: MaskRegister,
    pub status_register: StatusRegister,
    pub oam_address: u8,
    pub scroll_register: ScrollRegister,
    scanline: u16,
    cycles: usize,
    pub nmi_interrupt: Option<u8>,
}

impl ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        ppu {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
            address: address_register::new(),
            control_register: ControlRegister::new(),
            internal_buffer: 0,
            mask_register: MaskRegister::new(),
            status_register: StatusRegister::new(),
            oam_address: 0,
            scroll_register: ScrollRegister::new(),
            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
        }
    }

    pub fn tick(&mut self, ticks: u8) -> bool{
        self.cycles += ticks as usize;
        if(self.cycles >= 341){
            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status_register.set_vblank(true);
                //self.status.set_sprite_zero_hit(false);
                if self.control_register.generate_nmi() {
                    self.nmi_interrupt = Some(1);
                }
            }

            if(self.scanline >= 262){
                self.scanline = 0;
                self.nmi_interrupt = None;
                self.status_register.clear_vblank();
                return true;
            }
        }
        return false;

    }

    pub fn write_ppu_address(&mut self, data: u8){
        self.address.update(data);
    }

    pub fn write_control_register(&mut self, data: u8){
        let before_nmi_status = self.control_register.generate_nmi();
        self.control_register.update(data);
        if !before_nmi_status && self.control_register.generate_nmi() && self.status_register.check_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn write_mask_register(&mut self, data: u8){
        self.mask_register.update(data);
    }

    pub fn read_status_register(&mut self) -> u8{
        let data = self.status_register.get();
        self.status_register.clear_vblank();
        self.address.reset_ptr();
        self.scroll_register.reset_ptr();
        return data;
    }

    pub fn read_oam_data(&self) -> u8{
        return self.oam_data[self.oam_address as usize];
    }

    pub fn write_oam_data(&mut self, data: u8){
        self.oam_data[self.oam_address as usize] = data;
        self.oam_address = self.oam_address.wrapping_add(1);
    }

    pub fn write_oam_address(&mut self, address: u8){
        self.oam_address = address;
    }

    pub fn write_scroll_register(&mut self, data: u8){
        self.write_scroll_register(data);
    }

    fn increment_vram_address(&mut self){
        self.address.increment(self.control_register.vram_addr_increment());
    }

    pub fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for i in data.iter() {
            self.oam_data[self.oam_address as usize] = *i;
            self.oam_address = self.oam_address.wrapping_add(1);
        }
    }

    //test function
    pub fn new_empty_rom() -> Self {
        return ppu::new(vec![0; 2048], Mirroring::HORIZONTAL);
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
        self.increment_vram_address();

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

            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = address - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff =>
            {
                self.palette_table[(address - 0x3f00) as usize]
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

            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = address - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = data;
            }

            0x3f00..=0x3fff =>
            {
                self.palette_table[(address - 0x3f00) as usize] = data;
            }

            _ => {
                panic!("writing to mirror address spaces");
            }
        }
        self.increment_vram_address();
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

//Control Register

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

        pub fn generate_nmi(&mut self) -> bool{
            return self.contains(ControlRegister::GENERATE_NMI);
        }
        
        pub fn background_pattern_addr(&self) -> u16 {
            if !self.contains(ControlRegister::BACKROUND_PATTERN_ADDR) {
                0
            } else {
                0x1000
            }
        }
    
        pub fn update(&mut self, data: u8){
            self.bits = data;
        }
    }

    //Mask Register

    bitflags!{
        // 7  bit  0
        // ---- ----
        // VSOx xxxx
        // |||| ||||
        // |||+-++++- (PPU open bus or 2C05 PPU identifier)
        // ||+------- Sprite overflow flag
        // |+-------- Sprite 0 hit flag
        // +--------- Vblank flag, cleared on read. Unreliable; see below.
           pub struct MaskRegister: u8 {
            const GREYSCALE                = 0b00000001;
            const SHOW_BACKGROUND_LEFTMOST = 0b00000010;
            const SHOW_SPRITES_LEFTMOST    = 0b00000100;
            const ENABLE_BACKGROUND_RENDER = 0b00001000;
            const ENABLE_SPRITE_RENDER     = 0b00010000;
            const EMPHASIZE_RED            = 0b00100000;
            const EMPHASIZE_GREEN          = 0b01000000;
            const EMPHASIZE_BLUE           = 0b10000000;
            }
        }
        
        impl MaskRegister {
            pub fn new() -> Self{
                MaskRegister::from_bits_truncate(0b00000000)
            }
        
            pub fn update(&mut self, data: u8){
                self.bits = data;
            }
        }

    //Status Register
    
    bitflags!{
        // 7  bit  0
        // ---- ----
        // VSOx xxxx
        // |||| ||||
        // |||+-++++- (PPU open bus or 2C05 PPU identifier)
        // ||+------- Sprite overflow flag
        // |+-------- Sprite 0 hit flag
        // +--------- Vblank flag, cleared on read. Unreliable; see below.
            pub struct StatusRegister: u8 {
            const UNUSED1         = 0b00000001;
            const UNUSED2         = 0b00000010;
            const UNUSED3         = 0b00000100;
            const UNUSED4         = 0b00001000;
            const UNUSED5         = 0b00010000;
            const SPRITE_OVERFLOW = 0b00100000;
            const SPRITE_0_HIT    = 0b01000000;
            const VBLANK          = 0b10000000;
            }
        }
        
        impl StatusRegister {
            pub fn new() -> Self{
                StatusRegister::from_bits_truncate(0b00000000)
            }

            pub fn set_sprite_overflow(&mut self, flag: bool){
                self.set(StatusRegister::SPRITE_OVERFLOW, flag);
            }

            pub fn set_sprite_0(&mut self, flag: bool){
                self.set(StatusRegister::SPRITE_0_HIT, flag);
            }

            pub fn set_vblank(&mut self, flag: bool){
                self.set(StatusRegister::VBLANK, flag);
            }

            pub fn clear_vblank(&mut self){
                self.set(StatusRegister::VBLANK, false);
            }

            pub fn check_vblank(&self) -> bool{
                return self.contains(StatusRegister::VBLANK);
            }

            pub fn get(&self) -> u8 {
                return self.bits;
            }
        }

    pub struct ScrollRegister {
        X_scroll: u8,
        Y_scroll: u8,
        scroll_ptr: bool
    }

    impl ScrollRegister {
        pub fn new() -> Self{
            ScrollRegister {
                X_scroll: 0,
                Y_scroll: 0,
                scroll_ptr: false
            }
        }

        pub fn write(&mut self, data: u8){
            if(!self.scroll_ptr){
                self.X_scroll = data;
            } else{
                self.Y_scroll = data;
            }
            self.scroll_ptr = !self.scroll_ptr;
        }

        pub fn reset_ptr(&mut self){
            self.scroll_ptr = false;
        }
    }

    #[cfg(test)]
    pub mod testing {
        use crate::cartridge::*;
        use super::*;
    
        #[test]
        fn test_ppu_vram_writes() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_ppu_address(0x23);
            ppu.write_ppu_address(0x05);
            ppu.write_data(0x66);
    
            assert_eq!(ppu.vram[0x0305], 0x66);
        }
    
        #[test]
        fn test_ppu_vram_reads() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_control_register(0);
            ppu.vram[0x0305] = 0x66;
    
            ppu.write_ppu_address(0x23);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load_into_buffer
            assert_eq!(ppu.address.get(), 0x2306);
            assert_eq!(ppu.read_data(), 0x66);
        }
    
        #[test]
        fn test_ppu_vram_reads_cross_page() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_control_register(0);
            ppu.vram[0x01ff] = 0x66;
            ppu.vram[0x0200] = 0x77;
    
            ppu.write_ppu_address(0x21);
            ppu.write_ppu_address(0xff);
    
            ppu.read_data(); //load_into_buffer
            assert_eq!(ppu.read_data(), 0x66);
            assert_eq!(ppu.read_data(), 0x77);
        }
    
        #[test]
        fn test_ppu_vram_reads_step_32() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_control_register(0b100);
            ppu.vram[0x01ff] = 0x66;
            ppu.vram[0x01ff + 32] = 0x77;
            ppu.vram[0x01ff + 64] = 0x88;
    
            ppu.write_ppu_address(0x21);
            ppu.write_ppu_address(0xff);
    
            ppu.read_data(); //load_into_buffer
            assert_eq!(ppu.read_data(), 0x66);
            assert_eq!(ppu.read_data(), 0x77);
            assert_eq!(ppu.read_data(), 0x88);
        }
    
        // Horizontal: https://wiki.nesdev.com/w/index.php/Mirroring
        //   [0x2000 A ] [0x2400 a ]
        //   [0x2800 B ] [0x2C00 b ]
        #[test]
        fn test_vram_horizontal_mirror() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_ppu_address(0x24);
            ppu.write_ppu_address(0x05);
    
            ppu.write_data(0x66); //write to a
    
            ppu.write_ppu_address(0x28);
            ppu.write_ppu_address(0x05);
    
            ppu.write_data(0x77); //write to B
    
            ppu.write_ppu_address(0x20);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load into buffer
            assert_eq!(ppu.read_data(), 0x66); //read from A
    
            ppu.write_ppu_address(0x2C);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load into buffer
            assert_eq!(ppu.read_data(), 0x77); //read from b
        }
    
        // Vertical: https://wiki.nesdev.com/w/index.php/Mirroring
        //   [0x2000 A ] [0x2400 B ]
        //   [0x2800 a ] [0x2C00 b ]
        #[test]
        fn test_vram_vertical_mirror() {
            let mut ppu = ppu::new(vec![0; 2048], Mirroring::VERTICAL);

            ppu.write_ppu_address(0x20);
            ppu.write_ppu_address(0x05);
    
            ppu.write_data(0x66); //write to A
    
            ppu.write_ppu_address(0x2C);
            ppu.write_ppu_address(0x05);
    
            ppu.write_data(0x77); //write to b
    
            ppu.write_ppu_address(0x28);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load into buffer
            assert_eq!(ppu.read_data(), 0x66); //read from a
    
            ppu.write_ppu_address(0x24);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load into buffer
            assert_eq!(ppu.read_data(), 0x77); //read from B
        }
    
        #[test]
        fn test_read_status_resets_latch() {
            let mut ppu = ppu::new_empty_rom();
            ppu.vram[0x0305] = 0x66;
    
            ppu.write_ppu_address(0x21);
            ppu.write_ppu_address(0x23);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load_into_buffer
            assert_ne!(ppu.read_data(), 0x66);
    
            ppu.read_status_register();
    
            ppu.write_ppu_address(0x23);
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load_into_buffer
            assert_eq!(ppu.read_data(), 0x66);
        }
    
        #[test]
        fn test_ppu_vram_mirroring() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_control_register(0);
            ppu.vram[0x0305] = 0x66;
    
            ppu.write_ppu_address(0x63); //0x6305 -> 0x2305
            ppu.write_ppu_address(0x05);
    
            ppu.read_data(); //load into_buffer
            assert_eq!(ppu.read_data(), 0x66);
            // assert_eq!(ppu.addr.read(), 0x0306)
        }
    
        #[test]
        fn test_read_status_resets_vblank() {
            let mut ppu = ppu::new_empty_rom();
            ppu.status_register.set_vblank(true);
    
            let status = ppu.read_status_register();
    
            assert_eq!(status >> 7, 1);
            assert_eq!(ppu.status_register.get() >> 7, 0);
        }
    
        #[test]
        fn test_oam_read_write() {
            let mut ppu = ppu::new_empty_rom();
            ppu.write_oam_address(0x10);
            ppu.write_oam_data(0x66);
            ppu.write_oam_data(0x77);
    
            ppu.write_oam_address(0x10);
            assert_eq!(ppu.read_oam_data(), 0x66);
    
            ppu.write_oam_address(0x11);
            assert_eq!(ppu.read_oam_data(), 0x77);
        }
    
        #[test]
        fn test_oam_dma() {
            let mut ppu = ppu::new_empty_rom();
    
            let mut data = [0x66; 256];
            data[0] = 0x77;
            data[255] = 0x88;
    
            ppu.write_oam_address(0x10);
            ppu.write_oam_dma(&data);
    
            ppu.write_oam_address(0xf); //wrap around
            assert_eq!(ppu.read_oam_data(), 0x88);
    
            ppu.write_oam_address(0x10);
            ppu.write_oam_address(0x77);
            ppu.write_oam_address(0x11);
            ppu.write_oam_address(0x66);
        }
    }