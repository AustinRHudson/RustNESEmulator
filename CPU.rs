pub struct CPU{
    register_a: u8,
    register_x: u8,
    register_y: u8,
    program_counter: u16,
    status: u8,
    memory: [u8; 0xFFFF]
}

pub enum addressing_mode{
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

impl CPU{
    pub fn new() -> Self {
        CPU{
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            status: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn reset(&mut self){
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.memory_read_u16(0xFFFC);
    }

    fn memory_read(&self, address: u16) -> u8{
        return self.memory[address as usize];
    }

    fn memory_read_u16(&self, address: u16) -> u16{
        let lo = self.memory_read(address) as u16;
        let hi = self.memory_read(address + 1) as u16;
        return (hi << 8) | lo;
    }

    fn memory_write(&mut self, address: u16, value: u8){
        self.memory[address as usize] = value;
    }

    fn memory_write_u16(&mut self, address: u16, value: u16){
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;
        self.memory_write(address, lo);
        self.memory_write(address + 1, hi);
    }

    pub fn load_and_execute(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.execute()
    }

    fn load(&mut self, program: Vec<u8>){
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.memory_write_u16(0xFFFC, 0x8000);
    }

    fn update_negative_zero_flags(&mut self, value: u8){
        if(value == 0){
            self.status = self.status | 0b0000_0010;
        }else{
            self.status = self.status & 0b1111_1101;
        }

        if(value & 0b1000_0000 != 0){
            self.status = self.status | 0b1000_0000;
        }else{
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn inc_program_counter(&self, mode: &addressing_mode) -> u16{
        match mode {
            addressing_mode::Immediate => {
                return 1;
            }

            addressing_mode::ZeroPage => {
                return 1;
            }

            addressing_mode::ZeroPage_X => {
                return 1;
            }

            addressing_mode::ZeroPage_Y => {
                return 1;
            }

            addressing_mode::Absolute => {
                return 2;
            }
            
            addressing_mode::Absolute_X => {
                return 2;
            }

            addressing_mode::Absolute_Y => {
                return 2;
            }

            addressing_mode::Indirect_X => {
                return 1;
            }

            addressing_mode::Indirect_Y => {
                return 1;
            }

            _ => {
                todo!();
            }
        }
    }

    pub fn get_operand_address(&self, mode: &addressing_mode) -> u16{
        match mode {
            addressing_mode::Immediate => {
                return self.program_counter;
            }

            addressing_mode::ZeroPage => {
                return self.memory_read(self.program_counter) as u16;
            }

            addressing_mode::ZeroPage_X => {
                let address = self.memory_read(self.program_counter);
                return address.wrapping_add(self.register_x) as u16;
            }

            addressing_mode::ZeroPage_Y => {
                let address = self.memory_read(self.program_counter);
                return address.wrapping_add(self.register_y) as u16;
            }

            addressing_mode::Absolute => {
                return self.memory_read_u16(self.program_counter);
            }

            addressing_mode::Absolute_X => {
                let address = self.memory_read_u16(self.program_counter);
                return address.wrapping_add(self.register_x as u16);
            }

            addressing_mode::Absolute_Y => {
                let address = self.memory_read_u16(self.program_counter);
                return address.wrapping_add(self.register_y as u16);
            }

            addressing_mode::Indirect_X => {
                let address = self.memory_read(self.program_counter);
                let addressX: u8 = (address as u8).wrapping_add(self.register_x);
                let lo = self.memory_read(addressX as u16);
                let hi = self.memory_read(addressX.wrapping_add(1) as u16);
                return ((hi as u16) << 8) | (lo as u16);
            }

            addressing_mode::Indirect_Y => {
                let address = self.memory_read(self.program_counter);
                let lo = self.memory_read(address as u16);
                let hi = self.memory_read((address as u16).wrapping_add(1) as u16);
                let combinedAddress = ((hi as u16) << 8) | (lo as u16);
                return combinedAddress.wrapping_add(self.register_y as u16);
            }

            _ => {
                todo!();
            }
        }
    }

    fn LDA(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_a = value;
        self.update_negative_zero_flags(self.register_a);
    }

    fn STA(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_a);
    }

    pub fn execute(&mut self){
        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                //LDA
                0xA9 => {
                    self.LDA(&addressing_mode::Immediate);
                    self.program_counter += 1;
                }

                0xA5 => {
                    self.LDA(&addressing_mode::ZeroPage);
                    self.program_counter += 1;
                }

                0xB5 => {
                    self.LDA(&addressing_mode::ZeroPage_X);
                    self.program_counter += 1;
                }

                0xAD => {
                    self.LDA(&addressing_mode::Absolute);
                    self.program_counter += 2;
                }

                0xBD => {
                    self.LDA(&addressing_mode::Absolute_X);
                    self.program_counter += 2;
                }

                0xB9 => {
                    self.LDA(&addressing_mode::Absolute_Y);
                    self.program_counter += 2;
                }

                0xA1 => {
                    self.LDA(&addressing_mode::Indirect_X);
                    self.program_counter += 1;
                }

                0xB1 => {
                    self.LDA(&addressing_mode::Indirect_Y);
                    self.program_counter += 1;
                }
    
                //TAX
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_negative_zero_flags(self.register_x);
                }

                //STA
                0x85 => {
                    self.STA(&addressing_mode::ZeroPage);
                    self.program_counter += 1;
                }
    
                //ADC
                0x69 => {
    
                }
    
                //BRK
                0x00 => {
                    return;
                }
    
                _ => {
                    todo!();
                }
            }
        }
    }
}

//tests
fn main(){
    pub fn test_LDA(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x05, 0x00]);
        println!("{}", cpu.status);
        println!("{}", cpu.register_a);
    }
    pub fn test_TAX(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0xAA, 0x00]);
        println!("{}", cpu.register_a);
        println!("{}", cpu.register_x);
    }
    test_LDA();
}

