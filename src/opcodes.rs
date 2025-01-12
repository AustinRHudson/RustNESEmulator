// opcodes.rs

pub const LDA_IMM: u8 = 0xA9;
pub const LDA_0PGE: u8 = 0xA5;
pub const LDA_0PGE_X: u8 = 0xB5;
pub const LDA_ABS: u8 = 0xAD;
pub const LDA_ABS_X: u8 = 0xBD;
pub const LDA_ABS_Y: u8 = 0xB9;
pub const LDA_IND_X: u8 = 0xA1;
pub const LDA_IND_Y: u8 = 0xB1;

pub const STA_0PGE: u8 = 0x85;
pub const STA_0PGE_X: u8 = 0x95;
pub const STA_ABS: u8 = 0x8D;
pub const STA_ABS_X: u8 = 0x9D;
pub const STA_ABS_Y: u8 = 0x99;
pub const STA_IND_X: u8 = 0x81;
pub const STA_IND_Y: u8 = 0x91;

pub const ADC_IMM: u8 = 0x69;
pub const ADC_0PGE: u8 = 0x65;
pub const ADC_0PGE_X: u8 = 0x75;
pub const ADC_ABS: u8 = 0x6D;
pub const ADC_ABS_X: u8 = 0x7D;
pub const ADC_ABS_Y: u8 = 0x79;
pub const ADC_IND_X: u8 = 0x61;
pub const ADC_IND_Y: u8 = 0x71;

pub const AND_IMM: u8 = 0x29;
pub const AND_0PGE: u8 = 0x25;
pub const AND_0PGE_X: u8 = 0x35;
pub const AND_ABS: u8 = 0x2D;
pub const AND_ABS_X: u8 = 0x3D;
pub const AND_ABS_Y: u8 = 0x39;
pub const AND_IND_X: u8 = 0x21;
pub const AND_IND_Y: u8 = 0x31;

pub const ASL_ACC: u8 = 0x0A;
pub const ASL_0PGE: u8 = 0x06;
pub const ASL_0PGE_X: u8 = 0x16;
pub const ASL_ABS: u8 = 0x0E;
pub const ASL_ABS_X: u8 = 0x1E;

pub const BCC_REL: u8 = 0x90;
pub const BCS_REL: u8 = 0xB0;
pub const BEQ_REL: u8 = 0xF0;

pub const BIT_0PGE: u8 = 0x24;
pub const BIT_ABS: u8 = 0x2C;

pub const BMI_REL: u8 = 0x30;
pub const BNE_REL: u8 = 0xD0;
pub const BPL_REL: u8 = 0x10;
pub const BVC_REL: u8 = 0x50;
pub const BVS_REL: u8 = 0x70;

pub const BRK: u8 = 0x00;

pub const CLC_IMP: u8 = 0x18;
pub const CLD_IMP: u8 = 0xD8;
pub const CLI_IMP: u8 = 0x58;
pub const CLV_IMP: u8 = 0xB8;

pub const CMP_IMM: u8 = 0xC9;
pub const CMP_0PGE: u8 = 0xC5;
pub const CMP_0PGE_X: u8 = 0xD5;
pub const CMP_ABS: u8 = 0xCD;
pub const CMP_ABS_X: u8 = 0xDD;
pub const CMP_ABS_Y: u8 = 0xD9;
pub const CMP_IND_X: u8 = 0xC1;
pub const CMP_IND_Y: u8 = 0xD1;

pub const CPX_IMM: u8 = 0xE0;
pub const CPX_0PGE: u8 = 0xE4;
pub const CPX_ABS: u8 = 0xEC;

pub const CPY_IMM: u8 = 0xC0;
pub const CPY_0PGE: u8 = 0xC4;
pub const CPY_ABS: u8 = 0xCC;

pub const DEC_0PGE: u8 = 0xC6;
pub const DEC_0PGE_X: u8 = 0xD6;
pub const DEC_ABS: u8 = 0xCE;
pub const DEC_ABS_X: u8 = 0xDE;

pub const DEX_IMP: u8 = 0xCA;
pub const DEY_IMP: u8 = 0x88;

pub const EOR_IMM: u8 = 0x49;
pub const EOR_0PGE: u8 = 0x45;
pub const EOR_0PGE_X: u8 = 0x55;
pub const EOR_ABS: u8 = 0x4D;
pub const EOR_ABS_X: u8 = 0x5D;
pub const EOR_ABS_Y: u8 = 0x59;
pub const EOR_IND_X: u8 = 0x41;
pub const EOR_IND_Y: u8 = 0x51;

// INC
pub const INC_0PGE: u8 = 0xE6;
pub const INC_0PGE_X: u8 = 0xF6;
pub const INC_ABS: u8 = 0xEE;
pub const INC_ABS_X: u8 = 0xFE;

// INX
pub const INX_IMP: u8 = 0xE8;

// INY
pub const INY_IMP: u8 = 0xC8;

// JMP
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;

// JSR
pub const JSR_ABS: u8 = 0x20;

// LDX
pub const LDX_IMM: u8 = 0xA2;
pub const LDX_0PGE: u8 = 0xA6;
pub const LDX_0PGE_Y: u8 = 0xB6;
pub const LDX_ABS: u8 = 0xAE;
pub const LDX_ABS_Y: u8 = 0xBE;

// LDY
pub const LDY_IMM: u8 = 0xA0;
pub const LDY_0PGE: u8 = 0xA4;
pub const LDY_0PGE_X: u8 = 0xB4;
pub const LDY_ABS: u8 = 0xAC;
pub const LDY_ABS_X: u8 = 0xBC;

// LSR
pub const LSR_ACC: u8 = 0x4A;
pub const LSR_0PGE: u8 = 0x46;
pub const LSR_0PGE_X: u8 = 0x56;
pub const LSR_ABS: u8 = 0x4E;
pub const LSR_ABS_X: u8 = 0x5E;

// NOP
pub const NOP: u8 = 0xEA;

// ORA
pub const ORA_IMM: u8 = 0x09;
pub const ORA_0PGE: u8 = 0x05;
pub const ORA_0PGE_X: u8 = 0x15;
pub const ORA_ABS: u8 = 0x0D;
pub const ORA_ABS_X: u8 = 0x1D;
pub const ORA_ABS_Y: u8 = 0x19;
pub const ORA_IND_X: u8 = 0x01;
pub const ORA_IND_Y: u8 = 0x11;

// PHA
pub const PHA_IMP: u8 = 0x48;

// PHP
pub const PHP_IMP: u8 = 0x08;

// PLA
pub const PLA_IMP: u8 = 0x68;

// PLP
pub const PLP_IMP: u8 = 0x28;

// ROL
pub const ROL_ACC: u8 = 0x2A;
pub const ROL_0PGE: u8 = 0x26;
pub const ROL_0PGE_X: u8 = 0x36;
pub const ROL_ABS: u8 = 0x2E;
pub const ROL_ABS_X: u8 = 0x3E;

// ROR
pub const ROR_ACC: u8 = 0x6A;
pub const ROR_0PGE: u8 = 0x66;
pub const ROR_0PGE_X: u8 = 0x76;
pub const ROR_ABS: u8 = 0x6E;
pub const ROR_ABS_X: u8 = 0x7E;

// RTI
pub const RTI_IMP: u8 = 0x40;

// RTS
pub const RTS_IMP: u8 = 0x60;

// SBC
pub const SBC_IMM: u8 = 0xE9;
pub const SBC_0PGE: u8 = 0xE5;
pub const SBC_0PGE_X: u8 = 0xF5;
pub const SBC_ABS: u8 = 0xED;
pub const SBC_ABS_X: u8 = 0xFD;
pub const SBC_ABS_Y: u8 = 0xF9;
pub const SBC_IND_X: u8 = 0xE1;
pub const SBC_IND_Y: u8 = 0xF1;

// SEC
pub const SEC_IMP: u8 = 0x38;

// SED
pub const SED_IMP: u8 = 0xF8;

// SEI
pub const SEI_IMP: u8 = 0x78;

// STX
pub const STX_0PGE: u8 = 0x86;
pub const STX_0PGE_Y: u8 = 0x96;
pub const STX_ABS: u8 = 0x8E;

// STY
pub const STY_0PGE: u8 = 0x84;
pub const STY_0PGE_X: u8 = 0x94;
pub const STY_ABS: u8 = 0x8C;

// TAX
pub const TAX_IMP: u8 = 0xAA;

// TAY
pub const TAY_IMP: u8 = 0xA8;

// TSX
pub const TSX_IMP: u8 = 0xBA;

// TXA
pub const TXA_IMP: u8 = 0x8A;

// TXS
pub const TXS_IMP: u8 = 0x9A;

// TYA
pub const TYA_IMP: u8 = 0x98;


