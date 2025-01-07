
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

#[derive(Debug)]
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

    pub fn execute(&mut self){
        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                //LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.LDA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                    println!("{}", opcode_object.bytes - 1);
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
        println!("{:?}", opcode_map[&0xa9].address_mode);
    }
}

