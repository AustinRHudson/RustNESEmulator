//tests
#[cfg(test)]


mod tests {
    use crate::cpu::CPU;
    use crate::opcodes::*;
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
        assert_eq!(0b0011_0000, cpu.status);
        cpu.load_and_execute(vec![0x29, 0x00, 0xD0, 0x02, 0x00, 0xE8, 0xE8, 0x00]);
        assert_eq!(0x00, cpu.register_x);
        assert_eq!(0b0011_0010, cpu.status);
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
        cpu.load_and_execute(vec![0x90, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0xCF, 0x0A, 0xEA, 0x18, 0xB0, 0xF7, 0x00]);
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
        assert_eq!(cpu.status, 0b0011_0011);
        cpu.load_and_execute(vec![0xA9, 0x08, 0x85, 0x05, 0xA9, 0x09, 0xC5, 0x05, 0x00]);
        assert_eq!(cpu.status, 0b0011_0001);
    }

    #[test]
    fn test_CPX(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA2, 0x07, 0xE0, 0x08, 0x00]);
        assert_eq!(cpu.status, 0b1011_0001);    
    }

    #[test]
    fn test_CPY(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA0, 0x10, 0xC0, 0x10, 0xC0, 0x20, 0xC0, 0x08, 0x84, 0x30, 0xC4, 0x30, 0x8C, 0x00, 0x80, 0xCC, 0x00, 0x80, 0x00]);
        assert_eq!(cpu.status, 0b0011_0011);
    }

    #[test]
    fn test_DEC(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x08, 0x85, 0x0A, 0xC6, 0x0A, 0x00]);
        assert_eq!(cpu.memory_read(0x0A), 0x07);
    }

    #[test]
    fn test_EOR(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0b1010_1010, 0x49, 0b0101_0101, 0x00]);
        assert_eq!(0b1111_1111, cpu.register_a);
    }

    #[test]
    fn test_INC(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x08, 0x85, 0x0A, 0xE6, 0x0A, 0x00]);
        assert_eq!(cpu.memory_read(0x0A), 0x09);
    }

    #[test]
    fn test_JMP(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![BCC_REL, 0x03, 0xE8, 0xE8, 0x00, 0xA9, 0x02, 0x85, 0x01, 0xA9, 0x80, 0x85, 0x02, 0x6C, 0x01, 0x00, 0x00]);
        assert_eq!(cpu.register_x, 2);
    }

    #[test]
    fn test_JSR_RTS(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0x20, 0x06, 0x80, 0xE8, 0xE8, 0x00, 0xA9, 0x1A, 0x60, 0x00]);
        assert_eq!(cpu.register_x, 2);
    }

    #[test]
    fn test_LSR(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0x01, LSR_ACC, 0x00]);
        assert_eq!(0b0000_0000, cpu.register_a);
        assert_eq!(0b0011_0011, cpu.status);
        println!("{}", cpu.register_a);
    }

    #[test]
    fn test_ORA(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0b0000_1111, STA_0PGE, 0x01, LDA_IMM, 0b1111_0000, ORA_0PGE, 0x01, 0x00]);
        assert_eq!(0b1111_1111, cpu.register_a);
        assert_eq!(0b1011_0000, cpu.status);
    }

    #[test]
    fn test_PHA(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0x05, PHA_IMP, 0x00]);
        assert_eq!(cpu.memory_read(0x01FF), 0x05);
        assert_eq!(cpu.stack_pointer, 0xFE);
    }

    #[test]
    fn test_PHP(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![EOR_0PGE, 0x01, PHP_IMP, 0x00]);
        assert_eq!(cpu.memory_read(0x01FF), 0b0011_0010);
        assert_eq!(cpu.stack_pointer, 0xFE);
    }

    #[test]
    fn test_PLA(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0x05, PHA_IMP, LDA_IMM, 0x07, PLA_IMP, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_PLP(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![PHP_IMP, PHA_IMP, PLA_IMP, PLP_IMP, 0x00]);
        assert_eq!(cpu.status, 0b0011_0000);
    }

    #[test]
    fn test_ROL(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![CMP_IMM, 0x01, LDA_IMM, 0x04, ROL_ACC, 0x00]);
        assert_eq!(cpu.register_a, 0b0000_1001);
        cpu.load_and_execute(vec![LDA_IMM, 0x04, ROL_ACC, 0x00]);
        assert_eq!(cpu.register_a, 0b0000_1000);
    }

    #[test]
    fn test_ROR(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![CMP_IMM, 0x01, LDA_IMM, 0x04, ROR_ACC, 0x00]);
        assert_eq!(cpu.register_a, 0b1000_0010);
        cpu.load_and_execute(vec![LDA_IMM, 0x04, ROR_ACC, 0x00]);
        assert_eq!(cpu.register_a, 0b0000_0010);
    }

    #[test]
    fn test_ADC(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0x05, ADC_IMM, 0x04, 0x00]);
        assert_eq!(cpu.register_a, 9);
        assert_eq!(cpu.status, 0b0011_0000);
        cpu.load_and_execute(vec![SEC_IMP, LDA_IMM, 0x05, ADC_IMM, 0x04, 0x00]);
        assert_eq!(cpu.register_a, 10);
        assert_eq!(cpu.status, 0b0011_0000);
        cpu.load_and_execute(vec![LDA_IMM, 0b1000_0000, ADC_IMM, 0b1000_0000, 0x00]);
        println!("{:08b}", cpu.status);
        assert_eq!(cpu.status, 0b0111_0011);
        cpu.load_and_execute(vec![LDA_IMM, 0b1111_1110, ADC_IMM, 0b1111_1100, 0x00]);
        println!("{:08b}", cpu.status);
    }

    #[test]
    fn test_SBC(){  
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![LDA_IMM, 0x08, SBC_IMM, 0x04, 0x00]);
        assert_eq!(cpu.register_a, 3);
        cpu.load_and_execute(vec![LDA_IMM, 0x05, SEC_IMP, SBC_IMM, 0x03, STA_0PGE, 0x00, LDA_IMM, 0x05, CLC_IMP,
					SBC_IMM, 0x03, STA_0PGE, 0x01, LDA_IMM, 0x00, SEC_IMP, SBC_IMM, 0x01, STA_0PGE,
					0x02, LDA_IMM, 0x80, SEC_IMP, SBC_IMM, 0xFF, STA_0PGE, 0x03, 0x00]);
        assert_eq!(cpu.memory_read(0x03), 0x81);
        assert_eq!(cpu.memory_read(0x02), 0xFF);
        assert_eq!(cpu.memory_read(0x01), 0x01);
        assert_eq!(cpu.memory_read(0x00), 0x02);
    }
}