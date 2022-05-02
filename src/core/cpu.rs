// 0x0X
const BRK_IMPL: u8 = 0x00;
const ORA_X_IND: u8 = 0x01;
const ORA_ZPG: u8 = 0x05;
const ASL_ZPG: u8 = 0x06;
const PHP_IMPL: u8 = 0x08;
const ORA_IMM: u8 = 0x09;
const ASL_ACC: u8 = 0x0A;
const ORA_ABS: u8 = 0x0D;
const ASL_ABS: u8 = 0x0E;

// 0x1X
const BPL_REL: u8 = 0x10;
const ORA_IND: u8 = 0x11;
const ORA_ZPG_X: u8 = 0x15;
const ASL_ZPG_X: u8 = 0x16;
const CLC_IMP: u8 = 0x18;
const ORA_ABS_Y: u8 = 0x19;
const ORA_ABS_X: u8 = 0x1D;
const ASL_ABS_X: u8 = 0x1E;

// 0x2X
const JSR_ABS: u8 = 0x10;
const AND_X_IND: u8 = 0x11;
const BIT_ZPG: u8 = 0x14;
const AND_ZPG: u8 = 0x15;
const ROL_ZPG: u8 = 0x16;
const PLP_IMPL: u8 = 0x18;
const AND_IMM: u8 = 0x19;
const ROL_A: u8 = 0x1A;
const BIT_ABS: u8 = 0x1C;
const AND_ABS: u8 = 0x1D;
const ROL_ABS: u8 = 0x1E;

// 0x3X
const BMI_REL: u8 = 0x30;
const AND_IND_Y: u8 = 0x31;
const AND_ZPG_X: u8 = 0x35;
const ROL_ZPG_X: u8 = 0x36;
const SEC_IMPL: u8 = 0x38;
const AND_ABS_Y: u8 = 0x39;
const AND_ABS_X: u8 = 0x3D;
const ROL_ABS_X: u8 = 0x3E;

// 0x4X
const RTI_IMPL: u8 = 0x40;
const EOR_X_IND: u8 = 0x41;
const EOR_ZPG: u8 = 0x45;
const LSR_ZPG: u8 = 0x46;
const PHA_IMPL: u8 = 0x48;
const EOR_IMM: u8 = 0x49;
const LSR_A: u8 = 0x4A;
const JMP_ABS: u8 = 0x4C;
const EOR_ABS: u8 = 0x4D;
const LSR_ABS: u8 = 0x4E;

// 0x5X
const BVC_REL: u8 = 0x50;
const EOR_IND_Y: u8 = 0x51;
const EOR_ZPG_X: u8 = 0x55;
const LSR_ZPG_X: u8 = 0x56;
const CLI_IMPL: u8 = 0x58;
const EOR_ABS_Y: u8 = 0x59;
const EOR_ABS_X: u8 = 0x5D;
const LSR_ABS_X: u8 = 0x5E;

// 0x6X
const RTS_IMPL: u8 = 0x60;
const ADC_X_IND: u8 = 0x61;
const ADC_ZPG: u8 = 0x65;
const ROR_ZPG: u8 = 0x66;
const PLA_IMPL: u8 = 0x68;
const ADC_IMM: u8 = 0x69;
const ROR_A: u8 = 0x6A;
const JMP_IND: u8 = 0x6C;
const ADC_ABS: u8 = 0x6D;
const ROR_ABS: u8 = 0x6E;

// 0x7X
const BVS_REL: u8 = 0x70;
const ADC_IND_Y: u8 = 0x71;
const ADC_ZPG_X: u8 = 0x75;
const ROR_ZPG_X: u8 = 0x76;
const SEI_IMPL: u8 = 0x78;
const ADC_ABS_Y: u8 = 0x79;
const ADC_ABS_X: u8 = 0x7D;
const ROR_ABS_X: u8 = 0x7E;

// 0x8X
const STA_X_IND: u8 = 0x81;
const STY_ZPG: u8 = 0x84;
const STA_ZPG: u8 = 0x85;
const STX_ZPG: u8 = 0x86;
const DEY_IMPL: u8 = 0x88;
const TXA_IMPL: u8 = 0x8A;
const STY_ABS: u8 = 0x8C;
const STA_ABS: u8 = 0x8D;
const STX_ABS: u8 = 0x8E;

// 0x9X
const BCC_REL: u8 = 0x90;
const STA_IND_Y: u8 = 0x91;
const STY_ZPG_X: u8 = 0x94;
const STA_ZPG_X: u8 = 0x95;
const STX_ZPG_Y: u8 = 0x96;
const TYA_IMPL: u8 = 0x98;
const STA_ABS_Y: u8 = 0x99;
const TXS_IMPL: u8 = 0x9A;
const STA_ABS_X: u8 = 0x9D;

// 0xAX
const LDY_IMM: u8 = 0xA0;
const LDA_X_IND: u8 = 0xA1;
const LDX_IMM: u8 = 0xA2;
const LDY_ZPG: u8 = 0xA4;
const LDA_ZPG: u8 = 0xA5;
const LDX_ZPG: u8 = 0xA6;
const TAY_IMPL: u8 = 0xA8;
const LDA_IMM: u8 = 0xA9;
const TAX_IMPL: u8 = 0xAA;
const LDY_ABS: u8 = 0xAC;
const LDA_ABS: u8 = 0xAD;
const LDX_ABS: u8 = 0xAE;

// 0xBX
const BCS_REL: u8 = 0xB0;
const LDA_IND_Y: u8 = 0xB1;
const LDY_ZPG_X: u8 = 0xB4;
const LDA_ZPG_X: u8 = 0xB4;
const LDX_ZPG_Y: u8 = 0xB5;
const CLV_IMPL: u8 = 0xB8;
const LDA_ABS_Y: u8 = 0xB9;
const TSX_IMPL: u8 = 0xBA;
const LDY_ABS_X: u8 = 0xBC;
const LDA_ABS_X: u8 = 0xBD;
const LDX_ABS_Y: u8 = 0xBE;

// 0xCX
const CPY_IMM: u8 = 0xC0;
const CMP_X_IND: u8 = 0xC1;
const CPY_ZPG: u8 = 0xC4;
const CMP_ZPG: u8 = 0xC5;
const DEC_ZPG: u8 = 0xC6;
const INY_IMPL: u8 = 0xC8;
const CMP_IMM: u8 = 0xC9;
const DEX_IMPL: u8 = 0xCA;
const CPY_ABS: u8 = 0xCC;
const CMP_ABS: u8 = 0xCD;
const DEC_ABS: u8 = 0xCE;

// 0xDX
const BNE_REL: u8 = 0xD0;
const CMP_IND_Y: u8 = 0xD1;
const CMP_ZPG_X: u8 = 0xD5;
const DEC_ZPG_X: u8 = 0xD6;
const CLD_IMPL: u8 = 0xD8;
const CMP_ABS_Y: u8 = 0xD9;
const CMP_ABS_X: u8 = 0xDD;
const DEC_ABS_X: u8 = 0xDE;

// 0xEX
const CPX_IMM: u8 = 0xE0;
const SBC_X_IND: u8 = 0xE1;
const CPX_ZPG: u8 = 0xE4;
const SBC_ZPG: u8 = 0xE5;
const INC_ZPG: u8 = 0xE6;
const INX_IMPL: u8 = 0xE8;
const SBC_IMM: u8 = 0xE9;
const NOP_IMPL: u8 = 0xEA;
const CPX_ABS: u8 = 0xEC;
const SBC_ABS: u8 = 0xED;
const INC_ABS: u8 = 0xEE;

// 0xFX
const BEQ_REL: u8 = 0xF0;
const SBC_IND_Y: u8 = 0xF1;
const SBC_ZPG_X: u8 = 0xF5;
const INC_ZPG_X: u8 = 0xF6;
const SED_IMPL: u8 = 0xF8;
const SBC_ABS_Y: u8 = 0xF9;
const SBC_ABS_X: u8 = 0xFD;
const INC_ABS_X: u8 = 0xFE;

const Z_PAGE_BEGIN: u8 = 0x0;
const Z_PAGE_END: u8 = 0xFF;
const Z_PAGE_SIZE: u8 = Z_PAGE_END - Z_PAGE_BEGIN;

struct Instruction {
    name: String,
    opcode: u8,
    addr_mod: u8,
    cycles: u8,
}

#[derive(Default)]
pub struct CPU {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    cycles: u32,

    /// Status register, most commonly used flags are C, Z, V, S.
    /// - N = Negative/sign
    /// - V = Overflow
    /// - _ = (unused)
    /// - B = Break
    /// - D = Decimal (use BCD for arithmetics)
    /// - I = Interrupt (IRQ disable)
    /// - Z = Zero
    /// - C = Carry
    status_register: u8,
    stack_pointer: u8
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            pc: 0x0000, 
            ac: 0x00, 
            x: 0x00, 
            y: 0x00, 
            cycles: 0, 
            status_register: 0b00000000, 
            stack_pointer: 0x00 
        }
    }

    fn set_negative_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, 7, value);
    }

    fn set_overflow_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, 6, value);
    }

    fn set_break_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, 4, value);
    }

    fn set_decimal_flag(&mut self, value: bool) -> () {
        toggle_bit(&mut self.status_register, 3, value);
    }

    fn set_interrupt_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, 2, value);
    }

    fn set_zero_flag(&mut self, value: bool) -> () {
        toggle_bit(&mut self.status_register, 1, value);
    }

    fn set_carry_flag(&mut self, set: bool) -> () {
        toggle_bit(&mut self.status_register, 0, set);
    }

    pub fn execute_one(&mut self, prg: Vec<u8>, ram: &mut Vec<u8>) {
        // Why is it illegal to index a vec of u8 with a u16? 
        // Why wouldn't it be possible to have a vec of a length that exceeds u16?
        //
        // A usize is 4 bytes on a 32-bit machine, and 8 bytes on a 64-bit machine
        // 4 bytes = 32 bits, 2^32 =              4,294,967,296 =~ 4 GB of RAM
        // 8 bytes = 64 bits, 2^64 = 18,446,744,073,709,551,616 =~ lost of RAM
        //
        // So, casting a u8 (8 bits) or a u16 (16 bits) to usize (32 or 64 bits) should be safe
        let instr: u8 = prg[self.pc as usize];
        let pc: usize = self.pc as usize;


        // Addressing modes
        // ------------------------
        // Non-Indexed, non memory
        //   - Accumulator 
        //        Instructions have the accumulator register as it's target
        //   - Immediate
        //   - Implied
        // Non-Indexed memory ops
        //   - Relative
        //   - Absolute
        //   - Zero page 
        //   - Indirect
        // Indexed memory ops
        //   - Absolute Indexed
        //   - Zero-Page Indexed
        //   - Indexed Indirect
        //   - Indirect indexed
        match instr {
            // ADD WITH CARRY
            ADC_X_IND => {
                let inst_val = prg[pc + 1];
                let calc_addr = self.x + inst_val;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();
    
                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 6;
                self.pc += 2;
            }
            ADC_ZPG => {
                // Use the value from a relative address inside Z-page
                let inst_val = prg[pc + 1];
                let calc_addr = Z_PAGE_BEGIN as u8 + inst_val % Z_PAGE_SIZE as u8;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 3;
                self.pc += 2;
            },
            ADC_IMM => {
                let inst_val = prg[pc + 1];
                let value = inst_val;
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 2;
                self.pc += 2;
            },
            ADC_ABS => {
                let inst_val = prg[pc + 1] + prg[pc + 2];
                let calc_addr = inst_val;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 3;
            },
            ADC_IND_Y => {
                let inst_val = prg[pc + 1];
                let calc_addr = self.y + inst_val;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();
    
                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                let page_crossed = 0;

                self.bytes_cycles += 5 + page_crossed;
                self.pc += 2;
            },
            ADC_ZPG_X => {
                let inst_val = prg[pc + 1];
                let calc_addr = Z_PAGE_BEGIN as u8 + inst_val % Z_PAGE_SIZE as u8 + self.x;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 2;
            },
            ADC_ABS_Y => {
                let inst_val = prg[pc + 1] + prg[pc + 2];
                let calc_addr = inst_val + self.y;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 3;
            },
            ADC_ABS_X => {
                let inst_val = prg[pc + 1] + prg[pc + 2];
                let calc_addr = inst_val + self.x;
                let value = ram[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 3;
            },
    
            // AND
            AND_ABS => {},
            AND_ABS_X => {},
            AND_ABS_Y => {},
            AND_IMM => {},
            AND_IND_Y => {},
            AND_X_IND => {}
            AND_ZPG => {},
            AND_ZPG_X => {},
    
            // ARITHMETIC SHIFT LEFT
            ASL_ABS => {},
            ASL_ABS_X => {},
            ASL_ACC => {},
            ASL_ZPG => {},
            ASL_ZPG_X => {},
    
            // BIT
            BIT_ZPG => {},
            BIT_ABS => {},
    
            // BRANCH
            // add 1 to cycles if branch occurs on same page   
            // add 2 to cycles if branch occurs to different page
            BCC_REL => {},
            BCS_REL => {},
            BEQ_REL => {},
            BMI_REL => {},
            BNE_REL => {},
            BPL_REL => {},
            BVS_REL => {},
            BVC_REL => {},
    
            // BREAK / INTERRUPT
            BRK_IMPL => {
                ram[self.stack_pointer as usize] = (self.pc + 2) as u8;
                self.stack_pointer + 1;
            }
    
            // CLEAR
            CLC_IMP => {},
            CLD_IMPL => {},
            CLI_IMPL => {},
            CLV_IMPL => {},
    
            // COMPARE
            CMP_ABS => {},
            CMP_IMM => {},
            CMP_ZPG => {},
            CMP_ABS_X => {},
            CMP_ABS_Y => {},
            CMP_IND_Y => {},
            CMP_X_IND => {},
            CMP_ZPG_X => {},
    
            CPX_IMM => {},
            CPX_ZPG => {},
            CPX_ABS => {},
    
            CPY_IMM => {},
            CPY_ZPG => {},
            CPY_ABS => {},
    
            // DECREMENT
            DEC_ZPG => {},
            DEC_ABS => {},
            DEC_ZPG_X => {},
            DEC_ABS_X => {},
            DEY_IMPL => {},
            DEX_IMPL => {},
    
            // EXCLUSIVE OR
            EOR_X_IND => {}
            EOR_ZPG => {},
            EOR_IMM => {},
            EOR_ABS => {},
            EOR_IND_Y => {},
            EOR_ZPG_X => {},
            EOR_ABS_Y => {},
            EOR_ABS_X => {},
    
            // INCREMENT
            INC_ZPG => {},
            INC_ABS => {},
            INC_ZPG_X => {},
            INC_ABS_X => {},
            INX_IMPL => {},
            INY_IMPL => {},
    
            // JUMP
            JMP_ABS => {},
            JMP_IND => {},
    
            // JUMP SUBROUTINE
            JSR_ABS => {},
    
            // LOAD ACCUMULATOR
            LDA_ABS => {},
            LDA_ABS_Y => {},
            LDA_ABS_X => {},
            LDA_IMM => {},
            LDA_IND_Y => {},
            LDA_X_IND => {},
            LDA_ZPG => {},
            LDA_ZPG_X => {},
    
            // LOAD X
            LDX_ABS => {},
            LDX_ABS_Y => {},
            LDX_IMM => {},
            LDX_ZPG => {},
            LDX_ZPG_Y => {},
    
            // LOAD Y
            LDY_IMM => {},
            LDY_ZPG => {},
            LDY_ABS => {},
            LDY_ZPG_X => {},
            LDY_ABS_X => {},
    
            // LOGICAL SHIFT RIGHT
            LSR_A => {},
            LSR_ABS => {},
            LSR_ZPG => {},
            LSR_ZPG_X => {},
            LSR_ABS_X => {},
    
            // OR WITH ACCUMULATOR
            ORA_X_IND => {},
            ORA_ZPG => {},
            ORA_IMM => {},
            ORA_ABS => {},
            ORA_IND => {},
            ORA_ZPG_X => {},
            ORA_ABS_Y => {},
            ORA_ABS_X => {},
    
            // ROTATE LEFT
            ROL_A => {},
            ROL_ABS => {},
            ROL_ABS_X => {},
            ROL_ZPG_X => {},
            ROL_ZPG => {},
    
            // ROTATE RIGHT
            ROR_A => {},
            ROR_ABS => {},
            ROR_ZPG_X => {},
            ROR_ABS_X => {},
            ROR_ZPG => {},
    
            // PUSH
            PHA_IMPL => {},
            PHP_IMPL => {},
    
            // PULL
            PLA_IMPL => {},
            PLP_IMPL => {},
    
            // RETURN
            RTI_IMPL => {},
            RTS_IMPL => {},
    
            // SUBTRACT WITH CARRY
            SBC_X_IND => {},
            SBC_ZPG => {},
            SBC_IMM => {},
            SBC_ABS => {},
            SBC_IND_Y => {},
            SBC_ZPG_X => {},
            SBC_ABS_Y => {},
            SBC_ABS_X => {},
    
            // SET
            SEC_IMPL => {},
            SED_IMPL => {},
            SEI_IMPL => {},
    
            // STA
            STA_ABS => {},
            STA_ABS_X => {},
            STA_ABS_Y => {},
            STA_X_IND => {},
            STA_IND_Y => {},
            STA_ZPG_X => {},
            STA_ZPG => {},
    
            // STX
            STX_ABS => {},
            STX_ZPG => {},
            STX_ZPG_Y => {},
    
            // STY
            STY_ABS => {},
            STY_ZPG => {},
            STY_ZPG_X => {},
    
            // TRANSFER
            TXA_IMPL => {},
            TYA_IMPL => {},
            TXS_IMPL => {},
            TAY_IMPL => {},
            TAX_IMPL => {},
            TSX_IMPL => {},
    
            // NOP
            NOP_IMPL => {},
    
            _ => println!("({:x}) Missing instruction", instr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::CPU;

    #[test]
    fn it_works() {
        let mut c = CPU::new();
        let mut program = vec![0, 0xFF];

        program[0x0000] = 0x00;
        program[0x0001] = 0xAA;

        let mut ram = vec![0, 0xFF];
        c.execute_one(program, &mut ram);

        assert_eq!(ram[0], 0x11);
    }
}