
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
        //BCS
        opCode::new(0xB0, 2, 2, addressing_mode::Relative),
        //BEQ
        opCode::new(0xF0, 2, 2, addressing_mode::Relative),
        //BIT
        opCode::new(0x24, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x2C, 3, 4, addressing_mode::Absolute),
        //BMI
        opCode::new(0x30, 2, 2, addressing_mode::Relative),
        //BNE
        opCode::new(0xD0, 2, 2, addressing_mode::Relative),
        //BPL
        opCode::new(0x10, 2, 2, addressing_mode::Relative),
        //BVC
        opCode::new(0x50, 2, 2, addressing_mode::Relative),
        //BVS
        opCode::new(0x70, 2, 2, addressing_mode::Relative),
        //CLC
        opCode::new(0x18, 1, 2, addressing_mode::Implied),
        //CLD
        opCode::new(0xD8, 1, 2, addressing_mode::Implied),
        //CLI
        opCode::new(0x58, 1, 2, addressing_mode::Implied),
        //CLV
        opCode::new(0xB8, 1, 2, addressing_mode::Implied),
        //CMP
        opCode::new(0xC9, 2, 2, addressing_mode::Immediate),
        opCode::new(0xC5, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xD5, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xCD, 3, 4, addressing_mode::Absolute),
        opCode::new(0xDD, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0xD9, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0xC1, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0xD1, 2, 5, addressing_mode::Indirect_Y),
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
        self.update_negative_zero_flags(value);
    }

    fn BCC(&mut self){
        if((0b0000_0001 & self.status) != 0b0000_0001){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            // println!("{}", (value as u16) >> 8);
            //println!("bcc pc preadd {:#x}", self.program_counter);
            // self.program_counter += (value as u16);

            self.program_counter = self.program_counter.wrapping_add(value as u16);
            //println!("bcc pc postadd {:#x}", self.program_counter);
        }
        self.program_counter += 1;
    }

    fn BCS(&mut self){
        if((0b0000_0001 & self.status) == 0b0000_0001){
            // println!("bcc pc preadd {:#x}", self.program_counter);
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
            // println!("bcc pc postadd {:#x}", self.program_counter);
        }
        self.program_counter += 1;
    }

    fn BEQ(&mut self){
        if((0b0000_0010 & self.status) == 0b0000_0010){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BIT(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let mut value = self.memory_read(address);
        value = self.register_a & value;
        self.update_negative_zero_flags(value);
        if(value & 0b0100_0000 == 0b0100_0000){
            self.status = self.status | 0b0100_0000
        }else{
            self.status = self.status & 0b1011_1111
        }
    }

    fn BMI(&mut self){
        if((0b1000_0000 & self.status) == 0b1000_0000){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BNE(&mut self){
        if((0b0000_0010 & self.status) != 0b0000_0010){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BPL(&mut self){
        if((0b1000_0000 & self.status) != 0b1000_0000){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BVC(&mut self){
        if((0b0100_0000 & self.status) != 0b0100_0000){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BVS(&mut self){
        if((0b0100_0000 & self.status) == 0b0100_0000){
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn CLC(&mut self){
        self.status = self.status & 0b1111_1110;
    }

    fn CLD(&mut self){
        self.status = self.status & 0b1111_0111;
    }

    fn CLI(&mut self){
        self.status = self.status & 0b1111_1011;
    }

    fn CLV(&mut self){
        self.status = self.status & 0b1011_1111;
    }

    fn CMP(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        
        let result = self.register_a - value;
        self.update_negative_zero_flags(result);
        if(result >= 0){
            self.status = self.status | 0b0000_0001;    
        }else {
            self.status = self.status & 0b1111_1110;
        }
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

                //BCS
                0xB0 => {
                    self.BCS();
                }

                //BEQ
                0xF0 => {
                    self.BEQ();
                }

                //BIT
                0x24 | 0x2C => {
                    let opcode_object = opcode_map[&opcode];
                    self.BIT(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //BMI
                0x30 => {
                    self.BMI();
                }

                //BNE
                0xD0 => {
                    self.BNE();
                }

                //BPL
                0x10 => {
                    self.BPL();
                }

                //BVC
                0x50 => {
                    self.BVC();
                }

                //BVS
                0x70 => {
                    self.BVS();
                }

                //CLC
                0x18 => {
                    self.CLC();
                }

                //CLD
                0xD8 => {
                    self.CLD();
                }

                //CLI
                0x58 => {
                    self.CLI();
                }

                //CLV
                0xB8 => {
                    self.CLV();
                }

                //CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.CMP(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
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
        cpu.load_and_execute(vec![0xa9, 0x07, 0x8D, 0x05, 0x06, 0xa9, 0x04, 0x2D, 0x05, 0x06]);
        assert_eq!(0b0000_0100, cpu.register_a);
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
    fn test_BCS(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0xB0, 0xF7]);
        assert_eq!(cpu.register_x, 2);
    }

    #[test]
    fn test_BEQ(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x00, 0xF0, 0x01, 0x00, 0xE8, 0xE8]);
        assert_eq!(cpu.register_x, 2);
        cpu.load_and_execute(vec![0xA9, 0x00, 0xF0, 0x03, 0x00, 0xE8, 0xE8]);
        assert_eq!(cpu.register_x, 0);
    }

    #[test]
    fn test_BIT(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0x8D, 0x05, 0x06, 0xa9, 0x07, 0x2C, 0x05, 0x06]);
        assert_eq!(0x07, cpu.register_a);
    }

    #[test]
    fn test_BMI(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0xCD, 0x30, 0x02, 0x00, 0xE8, 0xE8]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BNE(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x01, 0xD0, 0x02, 0x00, 0xE8, 0xE8]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BPL(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x06, 0x10, 0x02, 0x00, 0xE8, 0xE8]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BVC(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x06, 0x50, 0x02, 0x00, 0xE8, 0xE8]);
        assert_eq!(0x01, cpu.register_x);
    }

    // #[test]
    // fn test_BVS(){
    //     //write function test, currently copied from previous test. No way to set overflow flag.
    // }

    #[test]
    fn test_CLC(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0x18, 0xB0, 0xF7]);
        assert_eq!(0, cpu.register_x);
    }

    // #[test]
    // fn test_CLD(){
    //     //No way to test decimal mode yet so can't clear the flag.
    // }

    // #[test]
    // fn test_CLI(){
    //     //No way to test interrupt flag yet so can't clear flag.
    // }

    // #[test]
    // fn test_CLV(){
    //     //No way to test overflow flag yet so can't clear flag.
    // }
}

