// //tests
// #[cfg(test)]


// mod tests {
//     use test::test_rom;

//     use crate::cpu::*;
//     use crate::opcodes::*;
//     use crate::bus::*;
//     use crate::cartridge::*;
// 	#[test]
//     fn test_LDA() {
//         let mut bus = Bus::new(test_rom(vec![LDA_IMM, 0x05, 0x00]));
//         let mut cpu = CPU::new(bus);
//         cpu.memory_write_u16(0xFFFC, 0x8000);
//         cpu.reset();
//         cpu.execute(move |cpu| {
            
//         });
//         assert_eq!(cpu.register_a, 5);
//     }

//     #[test]
//     fn test_addressing_modes() {
//         //Zeropage
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, STA_0PGE, 0xAA, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0xAA), 0x05);
//         //Zeropage_x
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, 0xA2, 0x05, 0x95, 0xA0, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0xA5), 0x05);
//         //Absolute
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, STA_ABS, 0x05, 0x06, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x0605), 0x05);
//         //Absolute_X
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, 0xA2, 0x05, 0x9D, 0x05, 0x06, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x060A), 0x08);
//         //Absolute_Y
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, 0xA0, 0x05, 0x99, 0x05, 0x06, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x060A), 0x08);
//     }

//     #[test]
//     fn test_TAX() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x04, 0xAA, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 4);
//         assert_eq!(cpu.register_x, 4);
//     }
//     #[test]
//     fn test_STA() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x23, STA_ABS, 0x05, 0x10, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x1005), 0x23);
//     }
//     #[test]
//     fn test_AND() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![
//             LDA_IMM, 0x07, STA_ABS, 0x05, 0x10, LDA_IMM, 0x04, AND_ABS, 0x05, 0x10, 0x00], 0x0600);
//         assert_eq!(0b0000_0100, cpu.register_a);
//     }

//     #[test]
//     fn test_ASL() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x04, 0x0A, 0x00], 0x0600);
//         assert_eq!(0b0000_1000, cpu.register_a);
//     }

//     #[test]
//     fn test_pos_BCC() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![
//             0xEA, 0x90, 0x0D, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 3);
//     }

//     #[test]
//     fn test_neg_BCC() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![0x90, 0x04, INX_IMP, INX_IMP, 0x00, 0xEA, 0x90, 0xFA, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 2);
//         cpu.load_and_execute(vec![
//             0x90, 0x03, INX_IMP, INX_IMP, 0x00, LDA_IMM, 0xCF, 0x0A, 0xEA, 0x90, 0xED, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 0);
//     }
//     #[test]
//     fn test_BCS() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![BCC_REL, 0x03, INX_IMP, INX_IMP, 0x00, LDA_IMM, 0xCF, ASL_ACC, NOP, BCS_REL, 0xF7, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 2);
//     }

//     #[test]
//     fn test_BEQ() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x00, 0xF0, 0x01, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 2);
//         cpu.load_and_execute(vec![LDA_IMM, 0x00, 0xF0, 0x03, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 0);
//     }

//     #[test]
//     fn test_BIT() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![
//             LDA_IMM, 0x04, STA_ABS, 0x05, 0x10, LDA_IMM, 0x07, BIT_ABS, 0x05, 0x10, 0x00], 0x0600);
//         assert_eq!(0x07, cpu.register_a);
//     }

//     #[test]
//     fn test_BMI() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0xCD, 0x30, 0x02, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(0x01, cpu.register_x);
//     }

//     #[test]
//     fn test_BNE() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x01, 0xD0, 0x02, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(0x01, cpu.register_x);
//         assert_eq!(0b0011_0000, cpu.status);
//         cpu.load_and_execute(vec![0x29, 0x00, 0xD0, 0x02, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(0x00, cpu.register_x);
//         assert_eq!(0b0011_0010, cpu.status);
//     }

//     #[test]
//     fn test_BPL() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x06, 0x10, 0x02, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(0x01, cpu.register_x);
//     }

//     #[test]
//     fn test_BVC() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x06, 0x50, 0x02, 0x00, INX_IMP, INX_IMP, 0x00], 0x0600);
//         assert_eq!(0x01, cpu.register_x);
//     }

//     //#[test]
//     //fn test_BVS(){
//     //    //write function test, currently copied from previous test. No way to set overflow flag.
//     //}

//     #[test]
//     fn test_CLC() {
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![0x90, 0x03, INX_IMP, INX_IMP, 0x00, LDA_IMM, 0xCF, 0x0A, 0xEA, 0x18, 0xB0, 0xF7, 0x00], 0x0600);
//         assert_eq!(0, cpu.register_x);
//     }

//     //#[test]
//     //fn test_CLD(){
//     //    //No way to test decimal mode yet so can't clear the flag.
//     //}

//     //#[test]
//     //fn test_CLI(){
//     //    //No way to test interrupt flag yet so can't clear flag.
//     //}

//     //#[test]
//     //fn test_CLV(){
//     //    //No way to test overflow flag yet so can't clear flag.
//     //}

//     #[test]
//     fn test_CMP(){
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, STA_0PGE, 0x05, CMP_0PGE, 0x05, 0x00], 0x0600);
//         println!("{:08b}", cpu.status);
//         assert_eq!(cpu.status, 0b0011_0011);
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, STA_0PGE, 0x05, LDA_IMM, 0x09, CMP_0PGE, 0x05, 0x00], 0x0600);
//         assert_eq!(cpu.status, 0b0011_0001);
//     }

//     #[test]
//     fn test_CPX(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![0xA2, 0x07, 0xE0, 0x08, 0x00], 0x0600);
//         assert_eq!(cpu.status, 0b1011_0001);    
//     }

//     #[test]
//     fn test_CPY(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDY_IMM, 0x10, CPY_IMM, 0x10, CPY_IMM, 0x20, CPY_IMM, 0x08, STY_0PGE, 0x30, CPY_0PGE, 0x30, STY_ABS, 0x00, 0x00, CPY_ABS, 0x00, 0x00, 0x00], 0x0600);
//         assert_eq!(cpu.status, 0b0011_0011);
//     }

//     #[test]
//     fn test_DEC(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, STA_0PGE, 0x0A, 0xC6, 0x0A, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x0A), 0x07);
//     }

//     #[test]
//     fn test_EOR(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0b1010_1010, 0x49, 0b0101_0101, 0x00], 0x0600);
//         assert_eq!(0b1111_1111, cpu.register_a);
//     }

//     #[test]
//     fn test_INC(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, STA_0PGE, 0x0A, 0xE6, 0x0A, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x0A), 0x09);
//     }

//     #[test]
//     fn test_JMP(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![BCC_REL, 0x03, INX_IMP, INX_IMP, 0x00, LDA_IMM, 0x02, STA_0PGE, 0x01, LDA_IMM, 0x06, STA_0PGE, 0x02, JMP_IND, 0x01, 0x00, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 2);
//     }

//     #[test]
//     fn test_JSR_RTS(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![JSR_ABS, 0x06, 0x06, INX_IMP, INX_IMP, 0x00, LDA_IMM, 0x1A, RTS_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.register_x, 2);
//     }

//     #[test]
//     fn test_LSR(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x01, LSR_ACC, 0x00], 0x0600);
//         assert_eq!(0b0000_0000, cpu.register_a);
//         assert_eq!(0b0011_0011, cpu.status);
//     }

//     #[test]
//     fn test_ORA(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0b0000_1111, STA_0PGE, 0x01, LDA_IMM, 0b1111_0000, ORA_0PGE, 0x01, 0x00], 0x0600);
//         assert_eq!(0b1111_1111, cpu.register_a);
//         assert_eq!(0b1011_0000, cpu.status);
//     }

//     #[test]
//     fn test_PHA(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, PHA_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x01FF), 0x05);
//         assert_eq!(cpu.stack_pointer, 0xFE);
//     }

//     #[test]
//     fn test_PHP(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![EOR_0PGE, 0x01, PHP_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x01FF), 0b0011_0010);
//         assert_eq!(cpu.stack_pointer, 0xFE);
//     }

//     #[test]
//     fn test_PLA(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, PHA_IMP, LDA_IMM, 0x07, PLA_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 0x05);
//         assert_eq!(cpu.stack_pointer, 0xFF);
//     }

//     #[test]
//     fn test_PLP(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![PHP_IMP, PHA_IMP, PLA_IMP, PLP_IMP, 0x00], 0x0600);
//         assert_eq!(cpu.status, 0b0011_0000);
//     }

//     #[test]
//     fn test_ROL(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![CMP_IMM, 0x01, LDA_IMM, 0x04, ROL_ACC, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 0b0000_1001);
//         cpu.load_and_execute(vec![LDA_IMM, 0x04, ROL_ACC, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 0b0000_1000);
//     }

//     #[test]
//     fn test_ROR(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![CMP_IMM, 0x01, LDA_IMM, 0x04, ROR_ACC, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 0b1000_0010);
//         cpu.load_and_execute(vec![LDA_IMM, 0x04, ROR_ACC, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 0b0000_0010);
//     }

//     #[test]
//     fn test_ADC(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, ADC_IMM, 0x04, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 9);
//         assert_eq!(cpu.status, 0b0011_0000);
//         cpu.load_and_execute(vec![SEC_IMP, LDA_IMM, 0x05, ADC_IMM, 0x04, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 10);
//         assert_eq!(cpu.status, 0b0011_0000);
//         cpu.load_and_execute(vec![LDA_IMM, 0b1000_0000, ADC_IMM, 0b1000_0000, 0x00], 0x0600);
//         assert_eq!(cpu.status, 0b0111_0011);
//         cpu.load_and_execute(vec![LDA_IMM, 0b1111_1110, ADC_IMM, 0b1111_1100, 0x00], 0x0600);
//     }

//     #[test]
//     fn test_SBC(){  
//         let bus = Bus::new(Rom::new(&vec![LDA_IMM, 0x05, 0x00]).unwrap());
//         let mut cpu = CPU::new(bus);
//         cpu.load_and_execute(vec![LDA_IMM, 0x08, SBC_IMM, 0x04, 0x00], 0x0600);
//         assert_eq!(cpu.register_a, 3);
//         cpu.load_and_execute(vec![LDA_IMM, 0x05, SEC_IMP, SBC_IMM, 0x03, STA_0PGE, 0x00, LDA_IMM, 0x05, CLC_IMP,
// 					SBC_IMM, 0x03, STA_0PGE, 0x01, LDA_IMM, 0x00, SEC_IMP, SBC_IMM, 0x01, STA_0PGE,
// 					0x02, LDA_IMM, 0x80, SEC_IMP, SBC_IMM, 0xFF, STA_0PGE, 0x03, 0x00], 0x0600);
//         assert_eq!(cpu.memory_read(0x03), 0x81);
//         assert_eq!(cpu.memory_read(0x02), 0xFF);
//         assert_eq!(cpu.memory_read(0x01), 0x01);
//         assert_eq!(cpu.memory_read(0x00), 0x02);
//     }
// }