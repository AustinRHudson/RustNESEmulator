extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status: u8,
    memory: [u8; 0xFFFF],
}

#[derive(PartialEq, Eq)]
pub enum addressing_mode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
    Accumulator,
    Relative,
    Implied,
}

pub struct opCode {
    code: u8,
    bytes: u8,
    cycles: u8,
    address_mode: addressing_mode,
}

impl opCode {
    pub const fn new(opCodeNum: u8, bytesNum: u8, cyclesNum: u8, mode: addressing_mode) -> Self {
        opCode {
            code: opCodeNum,
            bytes: bytesNum,
            cycles: cyclesNum,
            address_mode: mode,
        }
    }
}

lazy_static! {
    pub static ref opcode_list: Vec<opCode> = vec![
        
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
        //BRK
        opCode::new(0x00, 1, 7, addressing_mode::Implied),
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
        //CPX
        opCode::new(0xE0, 2, 2, addressing_mode::Immediate),
        opCode::new(0xE4, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xEC, 3, 4, addressing_mode::Absolute),
        //CPY
        opCode::new(0xC0, 2, 2, addressing_mode::Immediate),
        opCode::new(0xC4, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xCC, 3, 4, addressing_mode::Absolute),
        //DEC
        opCode::new(0xC6, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0xD6, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0xCE, 3, 6, addressing_mode::Absolute),
        opCode::new(0xDE, 3, 7, addressing_mode::Absolute_X),
        //DEX
        opCode::new(0xCA, 1, 2, addressing_mode::Implied),
        //DEY
        opCode::new(0x88, 1, 2, addressing_mode::Implied),
        //EOR
        opCode::new(0x49, 2, 2, addressing_mode::Immediate),
        opCode::new(0x45, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x55, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x4D, 3, 4, addressing_mode::Absolute),
        opCode::new(0x5D, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0x59, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0x41, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0x51, 2, 5, addressing_mode::Indirect_Y),
        //INC
        opCode::new(0xE6, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0xF6, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0xEE, 3, 6, addressing_mode::Absolute),
        opCode::new(0xFE, 3, 7, addressing_mode::Absolute_X),
        //INX
        opCode::new(0xE8, 1, 2, addressing_mode::Implied),
        //INY
        opCode::new(0xC8, 1, 2, addressing_mode::Implied),
        //JMP
        opCode::new(0x4C, 3, 3, addressing_mode::Absolute),
        opCode::new(0x6C, 3, 5, addressing_mode::Indirect),
        //JSR
        opCode::new(0x20, 3, 6, addressing_mode::Absolute),
        //LDA
        opCode::new(0xA9, 2, 2, addressing_mode::Immediate),
        opCode::new(0xA5, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xB5, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xAD, 3, 4, addressing_mode::Absolute),
        opCode::new(0xBD, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0xB9, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0xA1, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0xB1, 2, 5, addressing_mode::Indirect_Y),
        //LDX
        opCode::new(0xA2, 2, 2, addressing_mode::Immediate),
        opCode::new(0xA6, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xB6, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xAE, 3, 4, addressing_mode::Absolute),
        opCode::new(0xBE, 3, 4, addressing_mode::Absolute_Y),
        //LDY
        opCode::new(0xA0, 2, 2, addressing_mode::Immediate),
        opCode::new(0xA4, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xB4, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xAC, 3, 4, addressing_mode::Absolute),
        opCode::new(0xBC, 3, 4, addressing_mode::Absolute_X),
        //LSR
        opCode::new(0x4A, 1, 2, addressing_mode::Accumulator),
        opCode::new(0x46, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0x56, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0x4E, 3, 6, addressing_mode::Absolute),
        opCode::new(0x5E, 3, 7, addressing_mode::Absolute_X),
        //NOP
        opCode::new(0xEA, 1, 2, addressing_mode::Implied),
        //ORA
        opCode::new(0x09, 2, 2, addressing_mode::Immediate),
        opCode::new(0x05, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x15, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x0D, 3, 4, addressing_mode::Absolute),
        opCode::new(0x1D, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0x19, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0x01, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0x11, 2, 5, addressing_mode::Indirect_Y),
        //PHA
        opCode::new(0x48, 1, 3, addressing_mode::Implied),
        //PHP
        opCode::new(0x08, 1, 3, addressing_mode::Implied),
        //PLA
        opCode::new(0x68, 1, 4, addressing_mode::Implied),
        //PLP
        opCode::new(0x28, 1, 4, addressing_mode::Implied),
        //ROL
        opCode::new(0x2A, 1, 2, addressing_mode::Accumulator),
        opCode::new(0x26, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0x36, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0x2E, 3, 6, addressing_mode::Absolute),
        opCode::new(0x3E, 3, 7, addressing_mode::Absolute_X),
        //ROR
        opCode::new(0x6A, 1, 2, addressing_mode::Accumulator),
        opCode::new(0x66, 2, 5, addressing_mode::ZeroPage),
        opCode::new(0x76, 2, 6, addressing_mode::ZeroPage_X),
        opCode::new(0x6E, 3, 6, addressing_mode::Absolute),
        opCode::new(0x7E, 3, 7, addressing_mode::Absolute_X),
        //RTI
        opCode::new(0x40, 1, 6, addressing_mode::Implied),
        //RTS
        opCode::new(0x60, 1, 6, addressing_mode::Implied),
        //SBC
        opCode::new(0xE9, 2, 2, addressing_mode::Immediate),
        opCode::new(0xE5, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0xF5, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0xED, 3, 4, addressing_mode::Absolute),
        opCode::new(0xFD, 3, 4, addressing_mode::Absolute_X),
        opCode::new(0xF9, 3, 4, addressing_mode::Absolute_Y),
        opCode::new(0xE1, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0xF1, 2, 5, addressing_mode::Indirect_Y),
        //SEC
        opCode::new(0x38, 1, 2, addressing_mode::Implied),
        //SED
        opCode::new(0xF8, 1, 2, addressing_mode::Implied),
        //SEI
        opCode::new(0x78, 1, 2, addressing_mode::Implied),
        //STA
        opCode::new(0x85, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x95, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x8D, 3, 4, addressing_mode::Absolute),
        opCode::new(0x9D, 3, 5, addressing_mode::Absolute_X),
        opCode::new(0x99, 3, 5, addressing_mode::Absolute_Y),
        opCode::new(0x81, 2, 6, addressing_mode::Indirect_X),
        opCode::new(0x91, 2, 6, addressing_mode::Indirect_Y),
        //STX
        opCode::new(0x86, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x96, 2, 4, addressing_mode::ZeroPage_Y),
        opCode::new(0x8E, 3, 4, addressing_mode::Absolute),
        //STY
        opCode::new(0x84, 2, 3, addressing_mode::ZeroPage),
        opCode::new(0x94, 2, 4, addressing_mode::ZeroPage_X),
        opCode::new(0x8C, 3, 4, addressing_mode::Absolute),
        //TAX
        opCode::new(0xAA, 1, 2, addressing_mode::Implied),
        //TAY
        opCode::new(0xA8, 1, 2, addressing_mode::Implied),
        //TSX
        opCode::new(0xBA, 1, 2, addressing_mode::Implied),
        //TXA
        opCode::new(0x8A, 1, 2, addressing_mode::Implied),
        //TXS
        opCode::new(0x9A, 1, 2, addressing_mode::Implied),
        //TYA
        opCode::new(0x98, 1, 2, addressing_mode::Implied),
    ];

    pub static ref opcode_map: HashMap<u8, &'static opCode> = {
        let mut map = HashMap::new();
        for cpuop in 0..opcode_list.len() {
            map.insert(opcode_list[cpuop].code, &opcode_list[cpuop]);
        }
        map
    };
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0;
        self.status = 0;
        self.program_counter = self.memory_read_u16(0xFFFC);
    }

    fn memory_read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    fn memory_read_u16(&self, address: u16) -> u16 {
        let lo = self.memory_read(address) as u16;
        let hi = self.memory_read(address + 1) as u16;
        return (hi << 8) | lo;
    }

    fn memory_write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn memory_write_u16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;
        self.memory_write(address, lo);
        self.memory_write(address + 1, hi);
    }

    pub fn load_and_execute(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.execute()
    }

    fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.memory_write_u16(0xFFFC, 0x8000);
    }

    fn update_negative_zero_flags(&mut self, value: u8) {
        if (value == 0) {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if (value & 0b1000_0000 != 0) {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn get_operand_address(&self, mode: &addressing_mode) -> u16 {
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

    fn LDA(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_a = value;
        self.update_negative_zero_flags(self.register_a);
    }

    fn STA(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_a);
    }

    fn AND(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        self.register_a = self.register_a & value;
        self.update_negative_zero_flags(self.register_a);
    }

    fn ASL(&mut self, mode: &addressing_mode) {
        let mut value: u8;
        let mut address: u16 = 0;
        if (*mode == addressing_mode::Accumulator) {
            value = self.register_a;
        } else {
            address = self.get_operand_address(mode);
            value = self.memory_read(address);
        }
        self.status = (value >> 7) | (0b1111_1110 & self.status);
        value = value << 1;
        if (*mode == addressing_mode::Accumulator) {
            self.register_a = value;
        } else {
            self.memory_write(address, value);
        }
        self.update_negative_zero_flags(value);
    }

    fn BCC(&mut self) {
        if ((0b0000_0001 & self.status) != 0b0000_0001) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            //println!("{}", (value as u16) >> 8);
            //println!("bcc pc preadd {:#x}", self.program_counter);
            //self.program_counter += (value as u16);

            self.program_counter = self.program_counter.wrapping_add(value as u16);
            //println!("bcc pc postadd {:#x}", self.program_counter);
        }
        self.program_counter += 1;
    }

    fn BCS(&mut self) {
        if ((0b0000_0001 & self.status) == 0b0000_0001) {
            //println!("bcc pc preadd {:#x}", self.program_counter);
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
            //println!("bcc pc postadd {:#x}", self.program_counter);
        }
        self.program_counter += 1;
    }

    fn BEQ(&mut self) {
        if ((0b0000_0010 & self.status) == 0b0000_0010) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BIT(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        let result = self.register_a & value;
        if (result == 0) {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
        if (value & 0b1000_0000 == 0b1000_0000) {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111
        }
        if (value & 0b0100_0000 == 0b0100_0000) {
            self.status = self.status | 0b0100_0000
        } else {
            self.status = self.status & 0b1011_1111
        }
    }

    fn BMI(&mut self) {
        if ((0b1000_0000 & self.status) == 0b1000_0000) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BNE(&mut self) {
        if ((0b0000_0010 & self.status) != 0b0000_0010) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BPL(&mut self) {
        if ((0b1000_0000 & self.status) != 0b1000_0000) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BVC(&mut self) {
        if ((0b0100_0000 & self.status) != 0b0100_0000) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn BVS(&mut self) {
        if ((0b0100_0000 & self.status) == 0b0100_0000) {
            let value: i8 = (self.memory_read(self.program_counter) as i8);
            self.program_counter = self.program_counter.wrapping_add(value as u16);
        }
        self.program_counter += 1;
    }

    fn CLC(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    fn CLD(&mut self) {
        self.status = self.status & 0b1111_0111;
    }

    fn CLI(&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    fn CLV(&mut self) {
        self.status = self.status & 0b1011_1111;
    }

    fn CMP(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        let result = self.register_a.wrapping_sub(value);
        self.update_negative_zero_flags(result);
        if (result >= 0) {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
    }

    fn STX(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_x);
    }

    fn LDX(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_x = value;
        self.update_negative_zero_flags(self.register_x);
    }

    fn LDY(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_y = value;
        self.update_negative_zero_flags(self.register_y);
    }

    fn CPX(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        let result = self.register_x.wrapping_sub(value);
        self.update_negative_zero_flags(result);
        if (result >= 0) {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;
            println!("op code {:#x}", opcode);

            match opcode {
                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.LDA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // TAX
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_negative_zero_flags(self.register_x);
                }
            
                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    let opcode_object = opcode_map[&opcode];
                    self.STA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    let opcode_object = opcode_map[&opcode];
                    self.AND(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // ASL
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    let opcode_object = opcode_map[&opcode];
                    self.ASL(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // // ADC
                // 0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.ADC(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // // SBC
                // 0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.SBC(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // BCC
                0x90 => self.BCC(),
            
                // BCS
                0xB0 => self.BCS(),
            
                // BEQ
                0xF0 => self.BEQ(),
            
                // BIT
                0x24 | 0x2C => {
                    let opcode_object = opcode_map[&opcode];
                    self.BIT(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // BMI
                0x30 => self.BMI(),
            
                // BNE
                0xD0 => self.BNE(),
            
                // BPL
                0x10 => self.BPL(),

                //BRK
                0x00 => {
                    return;
                }
            
                // BVC
                0x50 => self.BVC(),
            
                // BVS
                0x70 => self.BVS(),
            
                // CLC
                0x18 => self.CLC(),
            
                // CLD
                0xD8 => self.CLD(),
            
                // CLI
                0x58 => self.CLI(),
            
                // CLV
                0xB8 => self.CLV(),
            
                // CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.CMP(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // CPX
                0xE0 | 0xE4 | 0xEC => {
                    let opcode_object = opcode_map[&opcode];
                    self.CPX(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // // CPY
                // 0xC0 | 0xC4 | 0xCC => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.CPY(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // // DEC
                // 0xC6 | 0xD6 | 0xCE | 0xDE => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.DEC(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // DEX
                0xCA => {
                    self.register_x = self.register_x.wrapping_sub(1);
                    self.update_negative_zero_flags(self.register_x);
                }
            
                // DEY
                0x88 => {
                    self.register_y = self.register_y.wrapping_sub(1);
                    self.update_negative_zero_flags(self.register_y);
                }
            
                // // EOR
                // 0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.EOR(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // // INC
                // 0xE6 | 0xF6 | 0xEE | 0xFE => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.INC(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // INX
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_negative_zero_flags(self.register_x);
                }
            
                // INY
                0xC8 => {
                    self.register_y = self.register_y.wrapping_add(1);
                    self.update_negative_zero_flags(self.register_y);
                }
            
                // // JMP
                // 0x4C | 0x6C => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.JMP(&opcode_object.address_mode);
                // }
            
                // // JSR
                // 0x20 => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.JSR(&opcode_object.address_mode);
                // }
            
                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    let opcode_object = opcode_map[&opcode];
                    self.LDX(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    let opcode_object = opcode_map[&opcode];
                    self.LDY(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // // LSR
                // 0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.LSR(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }

                //NOP
                0xEA => {

                }
            
                // // ORA
                // 0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.ORA(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }
            
                // // PHA
                // 0x48 => {
                //     self.stack_push(self.register_a);
                // }

                // // PHP
                // 0x08 => {
                //     self.stack_push(self.status | 0b00110000); // Push status with B and unused bits set
                // }

                // // PLA
                // 0x68 => {
                //     self.register_a = self.stack_pull();
                //     self.update_negative_zero_flags(self.register_a);
                // }

                // // PLP
                // 0x28 => {
                //     self.status = self.stack_pull() & 0b11001111; // Mask off B and unused bits
                // }

                // // ROL
                // 0x2A => {
                //     self.ROL_accumulator();
                // }
                // 0x26 | 0x36 | 0x2E | 0x3E => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.ROL(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }

                // // ROR
                // 0x6A => {
                //     self.ROR_accumulator();
                // }
                // 0x66 | 0x76 | 0x6E | 0x7E => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.ROR(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }

                // // RTI
                // 0x40 => {
                //     self.status = self.stack_pull() & 0b11001111; // Mask off B and unused bits
                //     let lo = self.stack_pull() as u16;
                //     let hi = self.stack_pull() as u16;
                //     self.program_counter = (hi << 8) | lo;
                // }

                // // RTS
                // 0x60 => {
                //     let lo = self.stack_pull() as u16;
                //     let hi = self.stack_pull() as u16;
                //     self.program_counter = ((hi << 8) | lo) + 1;
                // }

                // // SEC
                // 0x38 => {
                //     self.set_carry_flag();
                // }

                // // SED
                // 0xF8 => {
                //     self.set_decimal_mode();
                // }

                // // SEI
                // 0x78 => {
                //     self.set_interrupt_disable();
                // }

                // STX
                0x86 | 0x96 | 0x8E => {
                    let opcode_object = opcode_map[&opcode];
                    self.STX(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // // STY
                // 0x84 | 0x94 | 0x8C => {
                //     let opcode_object = opcode_map[&opcode];
                //     self.STY(&opcode_object.address_mode);
                //     self.program_counter += ((opcode_object.bytes - 1) as u16);
                // }

                // TAX
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_negative_zero_flags(self.register_x);
                }

                // TAY
                0xA8 => {
                    self.register_y = self.register_a;
                    self.update_negative_zero_flags(self.register_y);
                }

                // // TSX
                // 0xBA => {
                //     self.register_x = self.stack_pointer;
                //     self.update_negative_zero_flags(self.register_x);
                // }

                // TXA
                0x8A => {
                    self.register_a = self.register_x;
                    self.update_negative_zero_flags(self.register_a);
                }

                // TXS
                0x9A => {
                    self.stack_pointer = self.register_x;
                }

                // TYA
                0x98 => {
                    self.register_a = self.register_y;
                    self.update_negative_zero_flags(self.register_a);
                }

                _ => todo!("Unimplemented opcode: {:02X}", opcode),
            }
        }
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_LDA() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
    }

    #[test]
    fn test_addressing_modes() {
        //Zeropage
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x05, 0x85, 0xAA, 0x00]);
        assert_eq!(cpu.memory_read(0xAA), 0x05);
        //Zeropage_x
        cpu.load_and_execute(vec![0xa9, 0x05, 0xA2, 0x05, 0x95, 0xA0, 0x00]);
        assert_eq!(cpu.memory_read(0xA5), 0x05);
        //Absolute
        cpu.load_and_execute(vec![0xa9, 0x05, 0x8D, 0x05, 0x06, 0x00]);
        assert_eq!(cpu.memory_read(0x0605), 0x05);
        //Absolute_X
        cpu.load_and_execute(vec![0xa9, 0x08, 0xA2, 0x05, 0x9D, 0x05, 0x06, 0x00]);
        assert_eq!(cpu.memory_read(0x060A), 0x08);
        //Absolute_Y
        cpu.load_and_execute(vec![0xa9, 0x08, 0xA0, 0x05, 0x99, 0x05, 0x06, 0x00]);
        assert_eq!(cpu.memory_read(0x060A), 0x08);
    }

    #[test]
    fn test_TAX() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0xAA, 0x00]);
        assert_eq!(cpu.register_a, 4);
        assert_eq!(cpu.register_x, 4);
    }
    #[test]
    fn test_STA() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x23, 0x8D, 0x05, 0x06, 0x00]);
        assert_eq!(cpu.memory_read(0x0605), 0x23);
    }
    #[test]
    fn test_AND() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![
            0xa9, 0x07, 0x8D, 0x05, 0x06, 0xa9, 0x04, 0x2D, 0x05, 0x06, 0x00]);
        assert_eq!(0b0000_0100, cpu.register_a);
    }

    #[test]
    fn test_ASL() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x04, 0x0A, 0x00]);
        assert_eq!(0b0000_1000, cpu.register_a);
        println!("{}", cpu.register_a);
    }

    #[test]
    fn test_pos_BCC() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![
            0xEA, 0x90, 0x0D, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 3);
    }

    #[test]
    fn test_neg_BCC() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x90, 0x04, 0xE8, 0xE8, 0x00, 0xEA, 0x90, 0xFA, 0x00]);
        assert_eq!(cpu.register_x, 2);
        cpu.load_and_execute(vec![
            0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0x90, 0xED, 0x00]);
        assert_eq!(cpu.register_x, 0);
    }
    #[test]
    fn test_BCS() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![
            0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0xB0, 0xF7, 0x00]);
        assert_eq!(cpu.register_x, 2);
    }

    #[test]
    fn test_BEQ() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x00, 0xF0, 0x01, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 2);
        cpu.load_and_execute(vec![0xA9, 0x00, 0xF0, 0x03, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 0);
    }

    #[test]
    fn test_BIT() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![
            0xa9, 0x04, 0x8D, 0x05, 0x06, 0xa9, 0x07, 0x2C, 0x05, 0x06, 0x00]);
        assert_eq!(0x07, cpu.register_a);
    }

    #[test]
    fn test_BMI() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0xCD, 0x30, 0x02, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BNE() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x01, 0xD0, 0x02, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BPL() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x06, 0x10, 0x02, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(0x01, cpu.register_x);
    }

    #[test]
    fn test_BVC() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xa9, 0x06, 0x50, 0x02, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(0x01, cpu.register_x);
    }

    //#[test]
    //fn test_BVS(){
    //    //write function test, currently copied from previous test. No way to set overflow flag.
    //}

    #[test]
    fn test_CLC() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![
            0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0x18, 0xB0, 0xF7, 0x00]);
        assert_eq!(0, cpu.register_x);
    }

    //#[test]
    //fn test_CLD(){
    //    //No way to test decimal mode yet so can't clear the flag.
    //}

    //#[test]
    //fn test_CLI(){
    //    //No way to test interrupt flag yet so can't clear flag.
    //}

    //#[test]
    //fn test_CLV(){
    //    //No way to test overflow flag yet so can't clear flag.
    //}

    #[test]
    fn test_CMP(){
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x08, 0x85, 0x05, 0xC5, 0x05, 0x00]);
        assert_eq!(cpu.status, 0b0000_0011);
        cpu.load_and_execute(vec![0xA9, 0x08, 0x85, 0x05, 0xA9, 0x09, 0xC5, 0x05, 0x00]);
        assert_eq!(cpu.status, 0b0000_0001);
    }

    #[test]
    fn test_CPX(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA2, 0x07, 0xE0, 0x08, 0x00]);
        assert_eq!(cpu.status, 0b1000_0001);    
    }
}
