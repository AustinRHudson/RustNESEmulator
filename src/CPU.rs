
extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct CPU{
    register_a: u8,
    register_x: u8,
    register_y: u8,
    program_counter: u16,
    status: u8,
    memory: [u8; 0xFFFF]
}

#[derive(PartialEq, Eq)]
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
   Accumulator,
   Relative,
   Implied,
}

pub struct opCode{
    code: u8,
    bytes: u8,
    cycles: u8,
    address_mode: addressing_mode
}

impl opCode{
    pub const fn new(opCodeNum: u8, bytesNum: u8, cyclesNum: u8, mode: addressing_mode) -> Self {
        opCode{
            code: opCodeNum,
            bytes: bytesNum,
            cycles: cyclesNum,
            address_mode: mode
        }
    }
}

lazy_static!{
    pub static ref opcode_list: Vec<opCode> = vec![
        //LDA
        opCode::new(0xA9, 2, 2, addressing_mode::Immediate),
        opCode::new(0xA5, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xB5, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xAD, 3, 4, addressing_mode::Absolute),
        opCode::new(0xBD, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0xB9, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0xA1, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0xB1, 2, 5, addressing_mode::Indirect_Y),
        //STA
        opCode::new(0x85, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x95, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x8D, 3, 4, addressing_mode::Absolute),
        opCode::new(0x9D, 3, 5, addressing_mode::Absolute_X),
        opCode::new(0x99, 3, 5, addressing_mode::Absolute_Y),
        opCode::new(0x81, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0x91, 2, 6, addressing_mode::Indirect_Y),
        //AND
        opCode::new(0x29, 2, 2, addressing_mode::Immediate),
        opCode::new(0x25, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x35, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x2D, 3, 4, addressing_mode::Absolute),
        opCode::new(0x3D, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0x39, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0x21, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0x31, 2, 5, addressing_mode::Indirect_Y),
        //ASL
        opCode::new(0x0A, 1, 2, addressing_mode::Accumulator),
        opCode::new(0x06, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0x16, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0x0E, 3, 6, addressing_mode::Absolute),
        opCode::new(0x1E, 3, 7, addressing_mode::Absolute_X),
        //BCC
        opCode::new(0x90, 2, 2, addressing_mode::Relative),
        //NOP
        opCode::new(0xEA, 1, 2, addressing_mode::Implied),
        //BRK
        opCode::new(0x00, 1, 7, addressing_mode::Implied),
        //INX
        opCode::new(0xE8, 1, 2, addressing_mode::Implied),
    ];
    
    pub static ref opcode_map: HashMap<u8, &'static opCode> = {
        let mut map = HashMap::new();
        for cpuop in 0..opcode_list.len() {
            map.insert(opcode_list[cpuop].code, &opcode_list[cpuop]);
        }
        map
    };
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

    fn AND(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        self.register_a = self.register_a & value;
        self.update_negative_zero_flags(self.register_a);
    }

    fn ASL(&mut self, mode: &addressing_mode){
        let mut value: u8;
        let mut address: u16 = 0;
        if(*mode == addressing_mode::Accumulator){
            value = self.register_a;
        }else{
            address = self.get_operand_address(mode);
            value = self.memory_read(address);
        }
        self.status = (value >> 7) | (0b1111_1110 & self.status);
        value = value << 1;
        if(*mode == addressing_mode::Accumulator){
            self.register_a = value;
        }else{
            self.memory_write(address, value);
        }
        println!("{}", "OVERHERE!!!");
        self.update_negative_zero_flags(value);
    }

    fn BCC(&mut self){
        if((0b0000_0001 & self.status) != 0b0000_0001){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            // println!("{}", (value as u16) >> 8);
            println!("bcc pc preadd {:#x}", self.program_counter);
            // self.program_counter += (value as u16);

            self.program_counter = self.program_counter.wrapping_add(value as u16);
            println!("bcc pc postadd {:#x}", self.program_counter);
        }
        self.program_counter += 1;
    }

    pub fn execute(&mut self){
        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;
            println!("op code {:#x}", opcode);

            match opcode {
                //LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.LDA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //TAX
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_negative_zero_flags(self.register_x);
                }

                //STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    let opcode_object = opcode_map[&opcode];
                    self.STA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
                
                //AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    let opcode_object = opcode_map[&opcode];
                    self.AND(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //ASL
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    let opcode_object = opcode_map[&opcode];
                    self.ASL(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }   

                //BCC
                0x90 => {
                    let opcode_object = opcode_map[&opcode];
                    self.BCC();
                }

                //NOP
                0xEA => {
                   
                }

                //INX
                 0xE8 => {
                    self.register_x += 1;
                    self.update_negative_zero_flags(self.register_x);
                }

                //ADC
                0x69 => {
    
                }
    
                //BRK
                0x00 => {
                    return;
                }
    
                _ => {
                    todo!("ficinglol");
                }
            }
        }
    }
}

//tests
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_LDA(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
    }
    #[test]
    fn test_TAX(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0xAA, 0x00]);
        assert_eq!(cpu.register_a, 4);
        assert_eq!(cpu.register_x, 4);
    }
    #[test]
    fn test_STA(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x23, 0x8D, 0x05, 0x06]);
        assert_eq!(cpu.memory_read(0x0605), 0x23);
    }
    #[test]
    fn test_AND(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x07, 0x8D, 0x05, 0x06, 0xa9, 0x07, 0x2D, 0x05, 0x06]);
        assert_eq!(0b0000_0111, cpu.register_a);
    }

    #[test]
    fn test_ASL(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0x0A]);
        assert_eq!(0b0000_1000, cpu.register_a);
        println!("{}", cpu.register_a);
    }

    #[test]
    fn test_pos_BCC(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xEA, 0x90, 0x0D, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8]);
        assert_eq!(cpu.register_x, 3);
    }

    #[test]
    fn test_neg_BCC(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x90, 0x04, 0xE8, 0xE8, 0x00, 0xEA, 0x90, 0xFA]);
        assert_eq!(cpu.register_x, 2);
        cpu.load_and_execute(vec![0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0x90, 0xED]);
        assert_eq!(cpu.register_x, 0);
    }
    #[test]
    fn test_loop(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x90, 0x04, 0xE8, 0xE8, 0x00, 0xEA, 0x90, 0xFA]);
    }

}

