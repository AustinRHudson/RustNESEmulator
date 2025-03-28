extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::opcodes::*;
use crate::bus::*;
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///



pub struct CPU<'a> {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status: u8,
    pub bus: Bus<'a>,
    additional_cycles: u8,
}

#[derive(PartialEq, Eq, Debug)]
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

pub trait Mem {
    fn memory_read(&mut self, address: u16) -> u8;

    fn memory_write(&mut self, address: u16, value: u8);

    fn memory_read_u16(&mut self, address: u16) -> u16 {
        let lo = self.memory_read(address) as u16;
        let hi = self.memory_read(address + 1) as u16;
        return (hi << 8) | lo;
    }

    fn memory_write_u16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;
        self.memory_write(address, lo);
        self.memory_write(address + 1, hi);
    }
}

pub struct opCode {
    pub code: u8,
    pub bytes: u8,
    pub cycles: u8,
    pub address_mode: addressing_mode,
    pub mnemonic: &'static str,
}

impl opCode {
    pub const fn new(opCodeNum: u8, bytesNum: u8, cyclesNum: u8, mode: addressing_mode, mnemonicString: &'static str) -> Self {
        opCode {
            code: opCodeNum,
            bytes: bytesNum,
            cycles: cyclesNum,
            address_mode: mode,
            mnemonic: mnemonicString,
        }
    }
}

#[derive(PartialEq, Eq)]
    pub enum InterruptType {
        NMI,
        BRK,
    }

    #[derive(PartialEq, Eq)]
    pub(super) struct Interrupt {
        pub(super) itype: InterruptType,
        pub(super) vector_addr: u16,
        pub(super) b_flag_mask: u8,
        pub(super) cpu_cycles: u8,
    }

    pub(super) const NMI: Interrupt = Interrupt {
        itype: InterruptType::NMI,
        vector_addr: 0xfffA,
        b_flag_mask: 0b00100000,
        cpu_cycles: 2,
    };

    pub(super) const BRK: Interrupt = Interrupt {
        itype: InterruptType::BRK,
        vector_addr: 0xfffe,
        b_flag_mask: 0b00110000,
        cpu_cycles: 1,
    };

lazy_static! {
    pub static ref opcode_list: Vec<opCode> = vec![
        //ADC
        opCode::new(0x69, 2, 2, addressing_mode::Immediate, "ADC"),
        opCode::new(0x65, 2, 3, addressing_mode::ZeroPage, "ADC"),
        opCode::new(0x75, 2, 4, addressing_mode::ZeroPage_X, "ADC"),
        opCode::new(0x6D, 3, 4, addressing_mode::Absolute, "ADC"),
        opCode::new(0x7D, 3, 4, addressing_mode::Absolute_X, "ADC"),
        opCode::new(0x79, 3, 4, addressing_mode::Absolute_Y, "ADC"),
        opCode::new(0x61, 2, 6, addressing_mode::Indirect_X, "ADC"),
        opCode::new(0x71, 2, 5, addressing_mode::Indirect_Y, "ADC"),
        //AND
        opCode::new(0x29, 2, 2, addressing_mode::Immediate, "AND"),
        opCode::new(0x25, 2, 3, addressing_mode::ZeroPage, "AND"),
        opCode::new(0x35, 2, 4, addressing_mode::ZeroPage_X, "AND"),
        opCode::new(0x2D, 3, 4, addressing_mode::Absolute, "AND"),
        opCode::new(0x3D, 3, 4, addressing_mode::Absolute_X, "AND"),
        opCode::new(0x39, 3, 4, addressing_mode::Absolute_Y, "AND"),
        opCode::new(0x21, 2, 6, addressing_mode::Indirect_X, "AND"),
        opCode::new(0x31, 2, 5, addressing_mode::Indirect_Y, "AND"),
        //ASL
        opCode::new(0x0A, 1, 2, addressing_mode::Accumulator, "ASL"),
        opCode::new(0x06, 2, 5, addressing_mode::ZeroPage, "ASL"),
        opCode::new(0x16, 2, 6, addressing_mode::ZeroPage_X, "ASL"),
        opCode::new(0x0E, 3, 6, addressing_mode::Absolute, "ASL"),
        opCode::new(0x1E, 3, 7, addressing_mode::Absolute_X, "ASL"),
        //BCC
        opCode::new(0x90, 2, 2, addressing_mode::Relative, "BCC"),
        //BRK
        opCode::new(0x00, 1, 7, addressing_mode::Implied, "BRK"),
        //BCS
        opCode::new(0xB0, 2, 2, addressing_mode::Relative, "BCS"),
        //BEQ
        opCode::new(0xF0, 2, 2, addressing_mode::Relative, "BEQ"),
        //BIT
        opCode::new(0x24, 2, 3, addressing_mode::ZeroPage, "BIT"),
        opCode::new(0x2C, 3, 4, addressing_mode::Absolute, "BIT"),
        //BMI
        opCode::new(0x30, 2, 2, addressing_mode::Relative, "BMI"),
        //BNE
        opCode::new(0xD0, 2, 2, addressing_mode::Relative, "BNE"),
        //BPL
        opCode::new(0x10, 2, 2, addressing_mode::Relative, "BPL"),
        //BVC
        opCode::new(0x50, 2, 2, addressing_mode::Relative, "BVC"),
        //BVS
        opCode::new(0x70, 2, 2, addressing_mode::Relative, "BVS"),
        //CLC
        opCode::new(0x18, 1, 2, addressing_mode::Implied, "CLC"),
        //CLD
        opCode::new(0xD8, 1, 2, addressing_mode::Implied, "CLD"),
        //CLI
        opCode::new(0x58, 1, 2, addressing_mode::Implied, "CLI"),
        //CLV
        opCode::new(0xB8, 1, 2, addressing_mode::Implied, "CLV"),
        //CMP
        opCode::new(0xC9, 2, 2, addressing_mode::Immediate, "CMP"),
        opCode::new(0xC5, 2, 3, addressing_mode::ZeroPage, "CMP"),
        opCode::new(0xD5, 2, 4, addressing_mode::ZeroPage_X, "CMP"),
        opCode::new(0xCD, 3, 4, addressing_mode::Absolute, "CMP"),
        opCode::new(0xDD, 3, 4, addressing_mode::Absolute_X, "CMP"),
        opCode::new(0xD9, 3, 4, addressing_mode::Absolute_Y, "CMP"),
        opCode::new(0xC1, 2, 6, addressing_mode::Indirect_X, "CMP"),
        opCode::new(0xD1, 2, 5, addressing_mode::Indirect_Y, "CMP"),
        //CPX
        opCode::new(0xE0, 2, 2, addressing_mode::Immediate, "CPX"),
        opCode::new(0xE4, 2, 3, addressing_mode::ZeroPage, "CPX"),
        opCode::new(0xEC, 3, 4, addressing_mode::Absolute, "CPX"),
        //CPY
        opCode::new(0xC0, 2, 2, addressing_mode::Immediate, "CPY"),
        opCode::new(0xC4, 2, 3, addressing_mode::ZeroPage, "CPY"),
        opCode::new(0xCC, 3, 4, addressing_mode::Absolute, "CPY"),
        //DEC
        opCode::new(0xC6, 2, 5, addressing_mode::ZeroPage, "DEC"),
        opCode::new(0xD6, 2, 6, addressing_mode::ZeroPage_X, "DEC"),
        opCode::new(0xCE, 3, 6, addressing_mode::Absolute, "DEC"),
        opCode::new(0xDE, 3, 7, addressing_mode::Absolute_X, "DEC"),
        //DEX
        opCode::new(0xCA, 1, 2, addressing_mode::Implied, "DEX"),
        //DEY
        opCode::new(0x88, 1, 2, addressing_mode::Implied, "DEY"),
        //EOR
        opCode::new(0x49, 2, 2, addressing_mode::Immediate, "EOR"),
        opCode::new(0x45, 2, 3, addressing_mode::ZeroPage, "EOR"),
        opCode::new(0x55, 2, 4, addressing_mode::ZeroPage_X, "EOR"),
        opCode::new(0x4D, 3, 4, addressing_mode::Absolute, "EOR"),
        opCode::new(0x5D, 3, 4, addressing_mode::Absolute_X, "EOR"),
        opCode::new(0x59, 3, 4, addressing_mode::Absolute_Y, "EOR"),
        opCode::new(0x41, 2, 6, addressing_mode::Indirect_X, "EOR"),
        opCode::new(0x51, 2, 5, addressing_mode::Indirect_Y, "EOR"),
        //INC
        opCode::new(0xE6, 2, 5, addressing_mode::ZeroPage, "INC"),
        opCode::new(0xF6, 2, 6, addressing_mode::ZeroPage_X, "INC"),
        opCode::new(0xEE, 3, 6, addressing_mode::Absolute, "INC"),
        opCode::new(0xFE, 3, 7, addressing_mode::Absolute_X, "INC"),
        //INX
        opCode::new(0xE8, 1, 2, addressing_mode::Implied, "INX"),
        //INY
        opCode::new(0xC8, 1, 2, addressing_mode::Implied, "INY"),
        //JMP
        opCode::new(0x4C, 3, 3, addressing_mode::Absolute, "JMP"),
        opCode::new(0x6C, 3, 5, addressing_mode::Indirect, "JMP"),
        //JSR
        opCode::new(0x20, 3, 6, addressing_mode::Absolute, "JSR"),
        //LDA
        opCode::new(0xA9, 2, 2, addressing_mode::Immediate, "LDA"),
        opCode::new(0xA5, 2, 3, addressing_mode::ZeroPage, "LDA"),
        opCode::new(0xB5, 2, 4, addressing_mode::ZeroPage_X, "LDA"),
        opCode::new(0xAD, 3, 4, addressing_mode::Absolute, "LDA"),
        opCode::new(0xBD, 3, 4, addressing_mode::Absolute_X, "LDA"),
        opCode::new(0xB9, 3, 4, addressing_mode::Absolute_Y, "LDA"),
        opCode::new(0xA1, 2, 6, addressing_mode::Indirect_X, "LDA"),
        opCode::new(0xB1, 2, 5, addressing_mode::Indirect_Y, "LDA"),
        //LDX
        opCode::new(0xA2, 2, 2, addressing_mode::Immediate, "LDX"),
        opCode::new(0xA6, 2, 3, addressing_mode::ZeroPage, "LDX"),
        opCode::new(0xB6, 2, 4, addressing_mode::ZeroPage_X, "LDX"),
        opCode::new(0xAE, 3, 4, addressing_mode::Absolute, "LDX"),
        opCode::new(0xBE, 3, 4, addressing_mode::Absolute_Y, "LDX"),
        //LDY
        opCode::new(0xA0, 2, 2, addressing_mode::Immediate, "LDY"),
        opCode::new(0xA4, 2, 3, addressing_mode::ZeroPage, "LDY"),
        opCode::new(0xB4, 2, 4, addressing_mode::ZeroPage_X, "LDY"),
        opCode::new(0xAC, 3, 4, addressing_mode::Absolute, "LDY"),
        opCode::new(0xBC, 3, 4, addressing_mode::Absolute_X, "LDY"),
        //LSR
        opCode::new(0x4A, 1, 2, addressing_mode::Accumulator, "LSR"),
        opCode::new(0x46, 2, 5, addressing_mode::ZeroPage, "LSR"),
        opCode::new(0x56, 2, 6, addressing_mode::ZeroPage_X, "LSR"),
        opCode::new(0x4E, 3, 6, addressing_mode::Absolute, "LSR"),
        opCode::new(0x5E, 3, 7, addressing_mode::Absolute_X, "LSR"),
        //NOP
        opCode::new(0xEA, 1, 2, addressing_mode::Implied, "NOP"),
        //ORA
        opCode::new(0x09, 2, 2, addressing_mode::Immediate, "ORA"),
        opCode::new(0x05, 2, 3, addressing_mode::ZeroPage, "ORA"),
        opCode::new(0x15, 2, 4, addressing_mode::ZeroPage_X, "ORA"),
        opCode::new(0x0D, 3, 4, addressing_mode::Absolute, "ORA"),
        opCode::new(0x1D, 3, 4, addressing_mode::Absolute_X, "ORA"),
        opCode::new(0x19, 3, 4, addressing_mode::Absolute_Y, "ORA"),
        opCode::new(0x01, 2, 6, addressing_mode::Indirect_X, "ORA"),
        opCode::new(0x11, 2, 5, addressing_mode::Indirect_Y, "ORA"),
        //PHA
        opCode::new(0x48, 1, 3, addressing_mode::Implied, "PHA"),
        //PHP
        opCode::new(0x08, 1, 3, addressing_mode::Implied, "PHP"),
        //PLA
        opCode::new(0x68, 1, 4, addressing_mode::Implied, "PLA"),
        //PLP
        opCode::new(0x28, 1, 4, addressing_mode::Implied, "PLP"),
        //ROL
        opCode::new(0x2A, 1, 2, addressing_mode::Accumulator, "ROL"),
        opCode::new(0x26, 2, 5, addressing_mode::ZeroPage, "ROL"),
        opCode::new(0x36, 2, 6, addressing_mode::ZeroPage_X, "ROL"),
        opCode::new(0x2E, 3, 6, addressing_mode::Absolute, "ROL"),
        opCode::new(0x3E, 3, 7, addressing_mode::Absolute_X, "ROL"),
        //ROR
        opCode::new(0x6A, 1, 2, addressing_mode::Accumulator, "ROR"),
        opCode::new(0x66, 2, 5, addressing_mode::ZeroPage, "ROR"),
        opCode::new(0x76, 2, 6, addressing_mode::ZeroPage_X, "ROR"),
        opCode::new(0x6E, 3, 6, addressing_mode::Absolute, "ROR"),
        opCode::new(0x7E, 3, 7, addressing_mode::Absolute_X, "ROR"),
        //RTI
        opCode::new(0x40, 1, 6, addressing_mode::Implied, "RTI"),
        //RTS
        opCode::new(0x60, 1, 6, addressing_mode::Implied, "RTS"),
        //SBC
        opCode::new(0xE9, 2, 2, addressing_mode::Immediate, "SBC"),
        opCode::new(0xE5, 2, 3, addressing_mode::ZeroPage, "SBC"),
        opCode::new(0xF5, 2, 4, addressing_mode::ZeroPage_X, "SBC"),
        opCode::new(0xED, 3, 4, addressing_mode::Absolute, "SBC"),
        opCode::new(0xFD, 3, 4, addressing_mode::Absolute_X, "SBC"),
        opCode::new(0xF9, 3, 4, addressing_mode::Absolute_Y, "SBC"),
        opCode::new(0xE1, 2, 6, addressing_mode::Indirect_X, "SBC"),
        opCode::new(0xF1, 2, 5, addressing_mode::Indirect_Y, "SBC"),
        //SEC
        opCode::new(0x38, 1, 2, addressing_mode::Implied, "SEC"),
        //SED
        opCode::new(0xF8, 1, 2, addressing_mode::Implied, "SED"),
        //SEI
        opCode::new(0x78, 1, 2, addressing_mode::Implied, "SEI"),
        //STA
        opCode::new(0x85, 2, 3, addressing_mode::ZeroPage, "STA"),
        opCode::new(0x95, 2, 4, addressing_mode::ZeroPage_X, "STA"),
        opCode::new(0x8D, 3, 4, addressing_mode::Absolute, "STA"),
        opCode::new(0x9D, 3, 5, addressing_mode::Absolute_X, "STA"),
        opCode::new(0x99, 3, 5, addressing_mode::Absolute_Y, "STA"),
        opCode::new(0x81, 2, 6, addressing_mode::Indirect_X, "STA"),
        opCode::new(0x91, 2, 6, addressing_mode::Indirect_Y, "STA"),
        //STX
        opCode::new(0x86, 2, 3, addressing_mode::ZeroPage, "STX"),
        opCode::new(0x96, 2, 4, addressing_mode::ZeroPage_Y, "STX"),
        opCode::new(0x8E, 3, 4, addressing_mode::Absolute, "STX"),
        //STY
        opCode::new(0x84, 2, 3, addressing_mode::ZeroPage, "STY"),
        opCode::new(0x94, 2, 4, addressing_mode::ZeroPage_X, "STY"),
        opCode::new(0x8C, 3, 4, addressing_mode::Absolute, "STY"),
        //TAX
        opCode::new(0xAA, 1, 2, addressing_mode::Implied, "TAX"),
        //TAY
        opCode::new(0xA8, 1, 2, addressing_mode::Implied, "TAY"),
        //TSX
        opCode::new(0xBA, 1, 2, addressing_mode::Implied, "TSX"),
        //TXA
        opCode::new(0x8A, 1, 2, addressing_mode::Implied, "TXA"),
        //TXS
        opCode::new(0x9A, 1, 2, addressing_mode::Implied, "TXS"),
        //TYA
        opCode::new(0x98, 1, 2, addressing_mode::Implied, "TYA"),

        //Unoffical
        opCode::new(0xc7, 2, 5, addressing_mode::ZeroPage, "*DCP"),
        opCode::new(0xd7, 2, 6, addressing_mode::ZeroPage_X, "*DCP"),
        opCode::new(0xCF, 3, 6, addressing_mode::Absolute, "*DCP"),
        opCode::new(0xdF, 3, 7, addressing_mode::Absolute_X, "*DCP"),
        opCode::new(0xdb, 3, 7, addressing_mode::Absolute_Y, "*DCP"),
        opCode::new(0xd3, 2, 8, addressing_mode::Indirect_Y, "*DCP"),
        opCode::new(0xc3, 2, 8, addressing_mode::Indirect_X, "*DCP"),

        opCode::new(0x27, 2, 5, addressing_mode::ZeroPage, "*RLA"),
        opCode::new(0x37, 2, 6, addressing_mode::ZeroPage_X, "*RLA"),
        opCode::new(0x2F, 3, 6, addressing_mode::Absolute, "*RLA"),
        opCode::new(0x3F, 3, 7, addressing_mode::Absolute_X, "*RLA"),
        opCode::new(0x3b, 3, 7, addressing_mode::Absolute_Y, "*RLA"),
        opCode::new(0x33, 2, 8, addressing_mode::Indirect_Y, "*RLA"),
        opCode::new(0x23, 2, 8, addressing_mode::Indirect_X, "*RLA"),

        opCode::new(0x07, 2, 5, addressing_mode::ZeroPage, "*SLO"),
        opCode::new(0x17, 2, 6, addressing_mode::ZeroPage_X, "*SLO"),
        opCode::new(0x0F, 3, 6, addressing_mode::Absolute, "*SLO"),
        opCode::new(0x1f, 3, 7, addressing_mode::Absolute_X, "*SLO"),
        opCode::new(0x1b, 3, 7, addressing_mode::Absolute_Y, "*SLO"),
        opCode::new(0x03, 2, 8, addressing_mode::Indirect_X, "*SLO"),
        opCode::new(0x13, 2, 8, addressing_mode::Indirect_Y, "*SLO"),

        opCode::new(0x47, 2, 5, addressing_mode::ZeroPage, "*SRE"),
        opCode::new(0x57, 2, 6, addressing_mode::ZeroPage_X, "*SRE"),
        opCode::new(0x4F, 3, 6, addressing_mode::Absolute, "*SRE"),
        opCode::new(0x5f, 3, 7, addressing_mode::Absolute_X, "*SRE"),
        opCode::new(0x5b, 3, 7, addressing_mode::Absolute_Y, "*SRE"),
        opCode::new(0x43, 2, 8, addressing_mode::Indirect_X, "*SRE"),
        opCode::new(0x53, 2, 8, addressing_mode::Indirect_Y, "*SRE"),

        opCode::new(0x80, 2, 2, addressing_mode::Immediate, "*NOP"),
        opCode::new(0x82, 2, 2, addressing_mode::Immediate, "*NOP"),
        opCode::new(0x89, 2, 2, addressing_mode::Immediate, "*NOP"),
        opCode::new(0xc2, 2, 2, addressing_mode::Immediate, "*NOP"),
        opCode::new(0xe2, 2, 2, addressing_mode::Immediate, "*NOP"),

        opCode::new(0xCB, 2, 2, addressing_mode::Immediate, "*AXS"),
        opCode::new(0x6B, 2, 2, addressing_mode::Immediate, "*ARR"),
        opCode::new(0xeb, 2, 2, addressing_mode::Immediate, "*SBC"),
        opCode::new(0x0b, 2, 2, addressing_mode::Immediate, "*ANC"),
        opCode::new(0x2b, 2, 2, addressing_mode::Immediate, "*ANC"),
        opCode::new(0x4b, 2, 2, addressing_mode::Immediate, "*ALR"),

        opCode::new(0x04, 2, 3, addressing_mode::ZeroPage, "*NOP"),
        opCode::new(0x44, 2, 3, addressing_mode::ZeroPage, "*NOP"),
        opCode::new(0x64, 2, 3, addressing_mode::ZeroPage, "*NOP"),
        opCode::new(0x14, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0x34, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0x54, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0x74, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0xd4, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0xf4, 2, 4, addressing_mode::ZeroPage_X, "*NOP"),
        opCode::new(0x0c, 3, 4, addressing_mode::Absolute, "*NOP"),
        opCode::new(0x1c, 3, 4, addressing_mode::Absolute_X, "*NOP"),
        opCode::new(0x3c, 3, 4, addressing_mode::Absolute_X, "*NOP"),
        opCode::new(0x5c, 3, 4, addressing_mode::Absolute_X, "*NOP"),
        opCode::new(0x7c, 3, 4, addressing_mode::Absolute_X, "*NOP"),
        opCode::new(0xdc, 3, 4, addressing_mode::Absolute_X, "*NOP"),
        opCode::new(0xfc, 3, 4, addressing_mode::Absolute_X, "*NOP"),

        opCode::new(0x67, 2, 5, addressing_mode::ZeroPage, "*RRA"),
        opCode::new(0x77, 2, 6, addressing_mode::ZeroPage_X, "*RRA"),
        opCode::new(0x6f, 3, 6, addressing_mode::Absolute, "*RRA"),
        opCode::new(0x7f, 3, 7, addressing_mode::Absolute_X, "*RRA"),
        opCode::new(0x7b, 3, 7, addressing_mode::Absolute_Y, "*RRA"),
        opCode::new(0x63, 2, 8, addressing_mode::Indirect_X, "*RRA"),
        opCode::new(0x73, 2, 8, addressing_mode::Indirect_Y, "*RRA"),

        opCode::new(0xe7, 2, 5, addressing_mode::ZeroPage, "*ISB"),
        opCode::new(0xf7, 2, 6, addressing_mode::ZeroPage_X, "*ISB"),
        opCode::new(0xef, 3, 6, addressing_mode::Absolute, "*ISB"),
        opCode::new(0xff, 3, 7, addressing_mode::Absolute_X, "*ISB"),
        opCode::new(0xfb, 3, 7, addressing_mode::Absolute_Y, "*ISB"),
        opCode::new(0xe3, 2, 8, addressing_mode::Indirect_X, "*ISB"),
        opCode::new(0xf3, 2, 8, addressing_mode::Indirect_Y, "*ISB"),

        opCode::new(0x02, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x12, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x22, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x32, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x42, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x52, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x62, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x72, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x92, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0xb2, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0xd2, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0xf2, 1, 2, addressing_mode::NoneAddressing, "*NOP"),

        opCode::new(0x1a, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x3a, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x5a, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0x7a, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0xda, 1, 2, addressing_mode::NoneAddressing, "*NOP"),
        opCode::new(0xfa, 1, 2, addressing_mode::NoneAddressing, "*NOP"),

        opCode::new(0xab, 2, 3, addressing_mode::Immediate, "*LXA"),
        opCode::new(0x8b, 2, 3, addressing_mode::Immediate, "*XAA"),
        opCode::new(0xbb, 3, 2, addressing_mode::Absolute_Y, "*LAS"),
        opCode::new(0x9b, 3, 2, addressing_mode::Absolute_Y, "*TAS"),
        opCode::new(0x93, 2, 8, addressing_mode::Indirect_Y, "*AHX"),
        opCode::new(0x9f, 3, 4, addressing_mode::Absolute_Y, "*AHX"),
        opCode::new(0x9e, 3, 4, addressing_mode::Absolute_Y, "*SHX"),
        opCode::new(0x9c, 3, 4, addressing_mode::Absolute_X, "*SHY"),

        opCode::new(0xa7, 2, 3, addressing_mode::ZeroPage, "*LAX"),
        opCode::new(0xb7, 2, 4, addressing_mode::ZeroPage_Y, "*LAX"),
        opCode::new(0xaf, 3, 4, addressing_mode::Absolute, "*LAX"),
        opCode::new(0xbf, 3, 4, addressing_mode::Absolute_Y, "*LAX"),
        opCode::new(0xa3, 2, 6, addressing_mode::Indirect_X, "*LAX"),
        opCode::new(0xb3, 2, 5, addressing_mode::Indirect_Y, "*LAX"),

        opCode::new(0x87, 2, 3, addressing_mode::ZeroPage, "*SAX"),
        opCode::new(0x97, 2, 4, addressing_mode::ZeroPage_Y, "*SAX"),
        opCode::new(0x8f, 3, 4, addressing_mode::Absolute, "*SAX"),
        opCode::new(0x83, 2, 6, addressing_mode::Indirect_X, "*SAX"),

    ];

    pub static ref opcode_map: HashMap<u8, &'static opCode> = {
        let mut map = HashMap::new();
        for cpuop in 0..opcode_list.len() {
            map.insert(opcode_list[cpuop].code, &opcode_list[cpuop]);
        }
        map
    };
}

impl Mem for CPU<'_> {
    fn memory_read(&mut self, address: u16) -> u8 {
        // return self.memory[address as usize];
        return self.bus.memory_read(address)
    }

    fn memory_write(&mut self, address: u16, value: u8) {
        // self.memory[address as usize] = value;
        return self.bus.memory_write(address, value)
    }

    fn memory_read_u16(&mut self, address: u16) -> u16 {
        self.bus.memory_read_u16(address)
    }
  
    fn memory_write_u16(&mut self, address: u16, value: u16) {
        self.bus.memory_write_u16(address, value)
    }
}

const stack_reset: u8 = 0xFD;

impl <'a>CPU<'a> {
    pub fn new<'b>(bus: Bus<'b>) -> CPU<'b> {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status: 0,
            bus: bus,
            additional_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = stack_reset;
        self.status = 0b0010_0100;
        let pc = self.memory_read_u16(0xFFFC);
        // println!("{:02x}", self.memory_read(0xFFFC));
        // println!("{:02x}", self.memory_read(0xFFFD));
        // println!("{:04x}", pc);
        self.program_counter = if pc == 0 { 0x8000 } else {pc};
    }

    pub fn stack_push(&mut self, value: u8) {
        self.memory_write(0x100 + (self.stack_pointer as u16), value);
        //println!("{:x}", self.memory_read(0x100 + (self.stack_pointer as u16)));
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    } 

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        //println!("{:x}", self.memory_read(0x100 + (self.stack_pointer as u16)));
        let val = self.memory_read(0x100 + (self.stack_pointer as u16));
        // self.memory_write
        return val;
    }

    fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }

    pub fn load_and_execute(&mut self, program: Vec<u8>, starting_addr: u16) {
        //self.load(program, starting_addr);
        self.reset();
        self.execute(|_| {});
    }

    // pub fn load(&mut self, program: Vec<u8>) {
    //     self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    //     self.memory_write_u16(0xFFFC, 0x8000);
    // }
    pub fn load(&mut self, program: Vec<u8>, starting_addr: u16) {
        for i in 0..(program.len() as u16) {
            self.memory_write(starting_addr + i, program[i as usize]);
        }
        // old load
        // self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]); 
        //self.memory_write_u16(0xFFFC, 0x0600);
    }

    pub fn update_negative_zero_flags(&mut self, value: u8) {
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

    pub fn get_operand_address(&mut self, mode: &addressing_mode) -> u16 {
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
                let offset_address = address.wrapping_add(self.register_x as u16);
                if((offset_address & 0xFF00) != (address & 0xFF00)){
                    self.additional_cycles += 1;
                }
                return offset_address;
            }

            addressing_mode::Absolute_Y => {
                let address = self.memory_read_u16(self.program_counter);
                let offset_address = address.wrapping_add(self.register_y as u16);
                if((offset_address & 0xFF00) != (address & 0xFF00)){
                    self.additional_cycles += 1;
                }
                return offset_address
            }

            addressing_mode::Indirect => {
                let mem_address = self.memory_read_u16(self.program_counter);
                    // let indirect_ref = self.mem_read_u16(mem_address);
                    //6502 bug mode with with page boundary:
                    //  if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
                    // the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
                    // i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000

                    let indirect_ref = if mem_address & 0x00FF == 0x00FF {
                        let lo = self.memory_read(mem_address);
                        let hi = self.memory_read(mem_address & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    } else {
                        self.memory_read_u16(mem_address)
                    };

                    return indirect_ref;
            }

            addressing_mode::Indirect_X => {
                let address = self.memory_read(self.program_counter);
                let addressX: u8 = (address as u8).wrapping_add(self.register_x);
                let lo = self.memory_read(addressX as u16);
                let hi = self.memory_read(addressX.wrapping_add(1) as u16);
                return ((hi as u16) << 8) | (lo as u16);
            }

            addressing_mode::Indirect_Y => {
                let base = self.memory_read(self.program_counter);
                let lo = self.memory_read(base as u16);
                let hi = self.memory_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                if(((deref & 0xFF00) != (deref_base & 0xFF00))){
                    self.additional_cycles += 1;
                }
                return deref;
            }

            _ => {
                todo!("addressing_mode: {:?}", mode);
            }
        }
    }

    pub fn branch(&mut self){
        let value: i8 = (self.memory_read(self.program_counter) as i8);
        self.additional_cycles += 1;
        if((self.program_counter & 0xFF00) != (self.program_counter.wrapping_add(value as u16) & 0xFF00)){
            self.additional_cycles += 1;
        }
        self.program_counter = self.program_counter.wrapping_add(value as u16);
    }

    pub fn LDA(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_a = value;
        //println!("{:x}", value);
        self.update_negative_zero_flags(self.register_a);
    }

    pub fn STA(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_a);
    }

    pub fn AND(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        //println!("{:x}", value);
        self.register_a = self.register_a & value;
        self.update_negative_zero_flags(self.register_a);
    }

    pub fn ASL(&mut self, mode: &addressing_mode) {
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

    pub fn BCC(&mut self) {
        if ((0b0000_0001 & self.status) != 0b0000_0001) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BCS(&mut self) {
        if ((0b0000_0001 & self.status) == 0b0000_0001) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BEQ(&mut self) {
        if ((0b0000_0010 & self.status) == 0b0000_0010) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BIT(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        let result = self.register_a & value;
        if (result == 0) {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
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

    pub fn BMI(&mut self) {
        if ((0b1000_0000 & self.status) == 0b1000_0000) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BNE(&mut self) {
        if ((0b0000_0010 & self.status) != 0b0000_0010) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BPL(&mut self) {
        if ((0b1000_0000 & self.status) != 0b1000_0000) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BVC(&mut self) {
        if ((0b0100_0000 & self.status) != 0b0100_0000) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn BVS(&mut self) {
        if ((0b0100_0000 & self.status) == 0b0100_0000) {
            self.branch();
        }
        self.program_counter += 1;
    }

    pub fn CLC(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    pub fn CLD(&mut self) {
        self.status = self.status & 0b1111_0111;
    }

    pub fn CLI(&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    pub fn CLV(&mut self) {
        self.status = self.status & 0b1011_1111;
    }

    pub fn compare(&mut self, mode: &addressing_mode, register: u8) {
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        let result = register.wrapping_sub(value);
        self.update_negative_zero_flags(result);
        if (register >= value) {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
    }

    pub fn STX(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_x);
    }

    pub fn STY(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_y);
    }

    pub fn LDX(&mut self, mode: &addressing_mode) {
        let address = self.get_operand_address(mode);
        //println!("{:x}", address);
        let value = self.memory_read(address);

        self.register_x = value;
        self.update_negative_zero_flags(self.register_x);
    }

    pub fn LDY(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);

        self.register_y = value;
        self.update_negative_zero_flags(self.register_y);
    }

    pub fn DEC(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let mut value = self.memory_read(address);
    
        value = value.wrapping_sub(1);
        self.memory_write(address, value);
        self.update_negative_zero_flags(value);
    }

    pub fn INC(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let mut value = self.memory_read(address);
    
        value = value.wrapping_add(1);
        self.memory_write(address, value);
        self.update_negative_zero_flags(value);
    }

    pub fn EOR(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        let result = value ^ self.register_a;
        self.register_a = result;
        self.update_negative_zero_flags(result);
    }

    pub fn JMP(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        self.program_counter = address;
    }

    pub fn JSR(&mut self, mode: &addressing_mode){
        
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_address = self.memory_read_u16(self.program_counter);
        self.program_counter = target_address
    }

    pub fn RTS(&mut self){
        self.program_counter = self.stack_pop_u16() + 1;
    }

    pub fn LSR(&mut self, mode: &addressing_mode) {
        let mut value: u8;
        let mut address: u16 = 0;
        if (*mode == addressing_mode::Accumulator) {
            value = self.register_a;
        } else {
            address = self.get_operand_address(mode);
            value = self.memory_read(address);
        }
        self.status = (value & 0b0000_0001) | (0b1111_1110 & self.status);
        value = value >> 1;
        if (*mode == addressing_mode::Accumulator) {
            self.register_a = value;
        } else {
            self.memory_write(address, value);
        }
        self.update_negative_zero_flags(value);
    }

    pub fn ORA(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        self.register_a = value | self.register_a;
        self.update_negative_zero_flags(self.register_a);
    }

    pub fn ROL(&mut self, mode: &addressing_mode) {
        let mut value: u8;
        let mut address: u16 = 0;
        if (*mode == addressing_mode::Accumulator) {
            value = self.register_a;
        } else {
            address = self.get_operand_address(mode);
            value = self.memory_read(address);
        }
        let status_copy = (value >> 7) | (0b1111_1110 & self.status);
        value = value << 1;
        value = value | (self.status & 0b0000_0001);
        self.status = status_copy;
        if (*mode == addressing_mode::Accumulator) {
            self.register_a = value;
        } else {
            self.memory_write(address, value);
        }
        self.update_negative_zero_flags(value);
    }

    pub fn ROR(&mut self, mode: &addressing_mode) {
        let mut value: u8;
        let mut address: u16 = 0;
        if (*mode == addressing_mode::Accumulator) {
            value = self.register_a;
        } else {
            address = self.get_operand_address(mode);
            value = self.memory_read(address);
        }
        let status_copy = (value & 0b0000_0001) | (0b1111_1110 & self.status);
        value = value >> 1;
        value = value | ((self.status << 7) & 0b1000_0000);
        self.status = status_copy;
        if (*mode == addressing_mode::Accumulator) {
            self.register_a = value;
        } else {
            self.memory_write(address, value);
        }
        self.update_negative_zero_flags(value);
    }

    pub fn ADC(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let value = self.memory_read(address);
        let sum: u16 = (value as u16) + (self.register_a as u16) + ((self.status & 0b0000_0001) as u16);
        self.status = (self.status & 0b1111_1110) | (0b0000_0001 & (sum > 0xff) as u8);
        let overflow = !(self.register_a ^ value) & (self.register_a ^ (sum as u8)) & 0b1000_0000;
        self.status = (self.status & 0b1011_1111) | (0b0100_0000 & overflow >> 1);
        self.register_a = sum as u8;
        self.update_negative_zero_flags(self.register_a);
    }

    pub fn SBC(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let mut value = self.memory_read(address);
        value = !value;
        let sum: u16 = (value as u16) + (self.register_a as u16) + ((self.status & 0b0000_0001) as u16);
        self.status = (self.status & 0b1111_1110) | (0b0000_0001 & (sum > 0xff) as u8);
        let overflow = !(self.register_a ^ value) & (self.register_a ^ (sum as u8)) & 0b1000_0000;
        self.status = (self.status & 0b1011_1111) | (0b0100_0000 & overflow >> 1);
        self.register_a = sum as u8;
        self.update_negative_zero_flags(self.register_a);
    }

    pub fn DCP(&mut self, mode: &addressing_mode){
        let address = self.get_operand_address(mode);
        let mut value = self.memory_read(address);
    
        value = value.wrapping_sub(1);
        self.memory_write(address, value);
        let result = self.register_a.wrapping_sub(value);
        self.update_negative_zero_flags(result);
        if (result >= 0) {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
    }

    pub fn RLA(&mut self, mode: &addressing_mode){
        self.ROL(mode);
        self.AND(mode);
    }

    pub fn SLO(&mut self, mode: &addressing_mode){
        self.ASL(mode);
        self.ORA(mode);
    }

    pub fn SRE(&mut self, mode: &addressing_mode){
        self.LSR(mode);
        self.EOR(mode);
    }

    fn interrupt(&mut self,  interrupt: Interrupt){
        self.stack_push_u16(self.program_counter);
        if(interrupt.itype == InterruptType::NMI){
            self.stack_push((self.status & 0b1110_1111) | 0b0010_0000);
        } else if(interrupt.itype == InterruptType::BRK){
            self.stack_push(self.status | 0b0011_0000);
        }
        self.status = self.status | 0b0000_0100;
        self.bus.tick(interrupt.cpu_cycles);
        self.program_counter = self.memory_read_u16(interrupt.vector_addr);
    }


    pub fn execute<F>(&mut self, mut callback: F)
    where F: FnMut(&mut CPU) {
        loop {
            if let Some(_nmi) = self.bus.poll_nmi_status() {
                self.interrupt(NMI);
            }
            callback(self);
            self.additional_cycles = 0;
            //println!("{}", self.status);
            // let opcode = self.memory[self.program_counter as usize];
            let opcode = self.memory_read(self.program_counter);
            self.program_counter += 1;
            //println!("{:04x}", ((self.memory_read((self.stack_pointer + 1) as u16 + 0x100) as u16) << 8) | (self.memory_read((self.stack_pointer + 2) as u16 + 0x100))as u16);
            //println!("op code {:#x}", opcode);

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
            
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    let opcode_object = opcode_map[&opcode];
                    self.ADC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    let opcode_object = opcode_map[&opcode];
                    self.SBC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
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
                    self.interrupt(BRK);
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
                    self.compare(&opcode_object.address_mode, self.register_a);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // CPX
                0xE0 | 0xE4 | 0xEC => {
                    let opcode_object = opcode_map[&opcode];
                    self.compare(&opcode_object.address_mode, self.register_x);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // CPY
                0xC0 | 0xC4 | 0xCC => {
                    let opcode_object = opcode_map[&opcode];
                    self.compare(&opcode_object.address_mode, self.register_y);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    let opcode_object = opcode_map[&opcode];
                    self.DEC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
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
            
                // EOR
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    let opcode_object = opcode_map[&opcode];
                    self.EOR(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // INC
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    let opcode_object = opcode_map[&opcode];
                    self.INC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
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
            
                // JMP
                0x4C | 0x6C => {
                    let opcode_object = opcode_map[&opcode];
                    self.JMP(&opcode_object.address_mode);
                }
            
                // JSR
                0x20 => {
                    let opcode_object = opcode_map[&opcode];
                    self.JSR(&opcode_object.address_mode);
                }
            
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
            
                // LSR
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    let opcode_object = opcode_map[&opcode];
                    self.LSR(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //NOP
                0xEA => {

                }
            
                // ORA
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    let opcode_object = opcode_map[&opcode];
                    self.ORA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }
            
                // PHA
                0x48 => {
                    self.stack_push(self.register_a);
                }

                // PHP
                0x08 => {
                    self.stack_push(self.status | 0b0011_0000); // Push status with B and unused bit set
                }

                // PLA
                0x68 => {
                    self.register_a = self.stack_pop();
                    self.update_negative_zero_flags(self.register_a);
                }

                // PLP
                0x28 => {
                    self.status = (self.stack_pop() & 0b1110_1111) | 0b0010_0000;
                }

                // ROL
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    let opcode_object = opcode_map[&opcode];
                    self.ROL(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // ROR
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    let opcode_object = opcode_map[&opcode];
                    self.ROR(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // RTI
                0x40 => {
                    self.status = (self.stack_pop() & 0b1110_1111) | 0b0010_0000;
                    let lo = self.stack_pop() as u16;
                    let hi = self.stack_pop() as u16;
                    self.program_counter = (hi << 8) | lo;
                }

                // RTS
                0x60 => {
                    self.RTS();
                }

                // SEC
                0x38 => {
                    self.status |= 0b0000_0001; 
                }

                // SED
                0xF8 => {
                    self.status |= 0b0000_1000; 
                }

                // SEI
                0x78 => {
                    self.status |= 0b0000_0100; 
                }

                // STX
                0x86 | 0x96 | 0x8E => {
                    let opcode_object = opcode_map[&opcode];
                    self.STX(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // STY
                0x84 | 0x94 | 0x8C => {
                    let opcode_object = opcode_map[&opcode];
                    self.STY(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

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

                // TSX
                0xBA => {
                    self.register_x = self.stack_pointer;
                    self.update_negative_zero_flags(self.register_x);
                }

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

                //Unofficial Opcodes
                
                // //KIL (JAM) [HLT]
                0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xb2 | 0xd2
                | 0xf2 => { /* do nothing */ }

                0x1a | 0x3a | 0x5a | 0x7a | 0xda | 0xfa => { /* do nothing */ }

                //DCP
                0xc7 | 0xd7 | 0xCF | 0xdF | 0xdb | 0xd3 | 0xc3 => {
                    let opcode_object = opcode_map[&opcode];
                    self.DCP(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //RLA
                0x27 | 0x37 | 0x2F | 0x3F | 0x3b | 0x33 | 0x23 => {
                    let opcode_object = opcode_map[&opcode];
                    self.RLA(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //SLO
                0x07 | 0x17 | 0x0F | 0x1f | 0x1b | 0x03 | 0x13 => {
                    let opcode_object = opcode_map[&opcode];
                    self.SLO(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //SRE
                0x47 | 0x57 | 0x4F | 0x5f | 0x5b | 0x43 | 0x53 => {
                    let opcode_object = opcode_map[&opcode];
                    self.SRE(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //SKB
                0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 => {
                    let opcode_object = opcode_map[&opcode];
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                //AXS
                0xCB => {
                    let opcode_object = opcode_map[&opcode];
                    let addr = self.get_operand_address(&opcode_object.address_mode);
                    let data = self.memory_read(addr);
                    let x_and_a = self.register_x & self.register_a;
                    let result = x_and_a.wrapping_sub(data);

                    if data <= x_and_a {
                        self.status = self.status | 0b0000_0001;
                    }
                    self.update_negative_zero_flags(result);

                    self.register_x = result;
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* ARR */
                0x6B => {
                    let opcode_object = opcode_map[&opcode];
                    self.AND(&opcode_object.address_mode);
                    self.ROR(&addressing_mode::Accumulator);
                    //todo: registers
                    let result = self.register_a;
                    let bit_5 = (result >> 5) & 1;
                    let bit_6 = (result >> 6) & 1;

                    if bit_6 == 1 {
                        self.status = self.status | 0b0000_0001;
                    } else {
                        self.status = self.status & 0b1111_1110;
                    }

                    if bit_5 ^ bit_6 == 1 {
                        self.status = self.status | 0b0100_0000;
                    } else {
                        self.status = self.status & 0b1011_1111;
                    }

                    self.update_negative_zero_flags(result);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // /* unofficial SBC */
                0xeb => {
                    let opcode_object = opcode_map[&opcode];
                    self.SBC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* ANC */
                0x0b | 0x2b => {
                    let opcode_object = opcode_map[&opcode];
                    self.AND(&opcode_object.address_mode);
                    if self.status & 0b1000_0000 == 0b1000_0000 {
                        self.status = self.status | 0b0000_0001;
                    } else {
                        self.status = self.status & 0b1111_1110;
                    }
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* ALR */
                0x4b => {
                    let opcode_object = opcode_map[&opcode];
                    self.AND(&opcode_object.address_mode);
                    self.LSR(&addressing_mode::Accumulator);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                // //todo: test for everything below

                /* RRA */
                0x67 | 0x77 | 0x6f | 0x7f | 0x7b | 0x63 | 0x73 => {
                    let opcode_object = opcode_map[&opcode];
                    let data = self.ROR(&opcode_object.address_mode);
                    self.ADC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* ISB */
                0xe7 | 0xf7 | 0xef | 0xff | 0xfb | 0xe3 | 0xf3 => {
                    let opcode_object = opcode_map[&opcode];
                    self.INC(&opcode_object.address_mode);
                    self.SBC(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* LAX */
                0xa7 | 0xb7 | 0xaf | 0xbf | 0xa3 | 0xb3 => {
                    let opcode_object = opcode_map[&opcode];
                    let addr = self.get_operand_address(&opcode_object.address_mode);
                    let data = self.memory_read(addr);
                    self.register_a = data;
                    self.update_negative_zero_flags(self.register_a);
                    self.register_x = self.register_a;
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* SAX */
                0x87 | 0x97 | 0x8f | 0x83 => {
                    let opcode_object = opcode_map[&opcode];
                    let data = self.register_a & self.register_x;
                    let addr = self.get_operand_address(&opcode_object.address_mode);
                    self.memory_write(addr, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* LXA */
                0xab => {
                    let opcode_object = opcode_map[&opcode];
                    let opcode_object = opcode_map[&opcode];
                    self.LDA(&opcode_object.address_mode);
                    self.register_x = self.register_a;
                    self.update_negative_zero_flags(self.register_x);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* XAA */
                0x8b => {
                    let opcode_object = opcode_map[&opcode];
                    self.register_a = self.register_x;
                    self.update_negative_zero_flags(self.register_a);
                    self.AND(&opcode_object.address_mode);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* LAS */
                0xbb => {
                    let opcode_object = opcode_map[&opcode];
                    let addr = self.get_operand_address(&opcode_object.address_mode);
                    let mut data = self.memory_read(addr);
                    data = data & self.stack_pointer;
                    self.register_a = data;
                    self.register_x = data;
                    self.stack_pointer = data;
                    self.update_negative_zero_flags(data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* TAS */
                0x9b => {
                    let opcode_object = opcode_map[&opcode];
                    let data = self.register_a & self.register_x;
                    self.stack_pointer = data;
                    let mem_address =
                        self.memory_read_u16(self.program_counter) + self.register_y as u16;

                    let data = ((mem_address >> 8) as u8 + 1) & self.stack_pointer;
                    self.memory_write(mem_address, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* AHX  Indirect Y */
                0x93 => {
                    let opcode_object = opcode_map[&opcode];
                    let pos: u8 = self.memory_read(self.program_counter);
                    let mem_address = self.memory_read_u16(pos as u16) + self.register_y as u16;
                    let data = self.register_a & self.register_x & (mem_address >> 8) as u8;
                    self.memory_write(mem_address, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* AHX Absolute Y*/
                0x9f => {
                    let opcode_object = opcode_map[&opcode];
                    let mem_address = self.memory_read_u16(self.program_counter) + self.register_y as u16;

                    let data = self.register_a & self.register_x & (mem_address >> 8) as u8;
                    self.memory_write(mem_address, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* SHX */
                0x9e => {
                    let opcode_object = opcode_map[&opcode];
                    let mem_address = self.memory_read_u16(self.program_counter) + self.register_y as u16;

                    // todo if cross page boundry {
                    //     mem_address &= (self.x as u16) << 8;
                    // }
                    let data = self.register_x & ((mem_address >> 8) as u8 + 1);
                    self.memory_write(mem_address, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                /* SHY */
                0x9c => {
                    let opcode_object = opcode_map[&opcode];
                    let mem_address = self.memory_read_u16(self.program_counter) + self.register_x as u16;
                    let data = self.register_y & ((mem_address >> 8) as u8 + 1);
                    self.memory_write(mem_address, data);
                    self.program_counter += ((opcode_object.bytes - 1) as u16);
                }

                _ => {}//todo!("Unimplemented opcode: {:02X}", opcode),
            }
            let opcode_cycles = opcode_map[&opcode].cycles;
            self.bus.tick(opcode_cycles + self.additional_cycles);
        }
    }
}


