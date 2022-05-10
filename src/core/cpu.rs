use crate::{libs::bit::{toggle_bit, check_bit}, core::debug::{to_hex_u16, to_hex_u8}};
use std::{thread, time};
use super::{rom::ROM, debug::print_bytes_table};

const DEBUG: bool = true;

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
const JSR_ABS: u8 = 0x20;
const AND_X_IND: u8 = 0x21;
const BIT_ZPG: u8 = 0x24;
const AND_ZPG: u8 = 0x25;
const ROL_ZPG: u8 = 0x26;
const PLP_IMPL: u8 = 0x28;
const AND_IMM: u8 = 0x29;
const ROL_A: u8 = 0x2A;
const BIT_ABS: u8 = 0x2C;
const AND_ABS: u8 = 0x2D;
const ROL_ABS: u8 = 0x2E;

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
const LDA_ZPG_X: u8 = 0xB5;
const LDX_ZPG_Y: u8 = 0xB6;
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

// MEMORY RANGES
const Z_PAGE_BEGIN: usize = 0x0000;
const Z_PAGE_END: usize = 0x00FF;
const Z_PAGE_SIZE: usize = Z_PAGE_END - Z_PAGE_BEGIN;

const STACK_BEGIN: usize = 0x0100;
const STACK_END: usize = 0x01FF;
const STACK_SIZE: usize = STACK_END - STACK_BEGIN;

const GENERAL_PURPOSE_BEGIN: usize = 0x0200;
const GENERAL_PURPOSE_END: usize = 0xFFFF;
const GENERAL_PURPOSE_SIZE: usize = GENERAL_PURPOSE_END - GENERAL_PURPOSE_BEGIN;

const NEGATIVE_FLAG_INDEX: u8 = 7;
const OVERFLOW_FLAG_INDEX: u8 = 6;
const BREAK_FLAG_INDEX: u8 = 4;
const DECIMAL_FLAG_INDEX: u8 = 3;
const INTERRUPT_FLAG_INDEX: u8 = 2;
const ZERO_FLAG_INDEX: u8 = 1;
const CARRY_FLAG_INDEX: u8 = 0;

fn get_instruction_name(instr_1: u8) -> &'static str {
    match instr_1 {
        // 0x0X
        BRK_IMPL => "BRK_IMPL",
        ORA_X_IND => "ORA_X_IND",
        ORA_ZPG => "ORA_ZPG",
        ASL_ZPG => "ASL_ZPG",
        PHP_IMPL => "PHP_IMPL",
        ORA_IMM => "ORA_IMM",
        ASL_ACC => "ASL_ACC",
        ORA_ABS => "ORA_ABS",
        ASL_ABS => "ASL_ABS",

        // 0x1X
        BPL_REL => "BPL_REL",
        ORA_IND => "ORA_IND",
        ORA_ZPG_X => "ORA_ZPG_X",
        ASL_ZPG_X => "ASL_ZPG_X",
        CLC_IMP => "CLC_IMP",
        ORA_ABS_Y => "ORA_ABS_Y",
        ORA_ABS_X => "ORA_ABS_X",
        ASL_ABS_X => "ASL_ABS_X",

        // 0x2X
        JSR_ABS => "JSR_ABS",
        AND_X_IND => "AND_X_IND",
        BIT_ZPG => "BIT_ZPG",
        AND_ZPG => "AND_ZPG",
        ROL_ZPG => "ROL_ZPG",
        PLP_IMPL => "PLP_IMPL",
        AND_IMM => "AND_IMM",
        ROL_A => "ROL_A",
        BIT_ABS => "BIT_ABS",
        AND_ABS => "AND_ABS",
        ROL_ABS => "ROL_ABS",

        // 0x3X
        BMI_REL => "BMI_REL",
        AND_IND_Y => "AND_IND_Y",
        AND_ZPG_X => "AND_ZPG_X",
        ROL_ZPG_X => "ROL_ZPG_X",
        SEC_IMPL => "SEC_IMPL",
        AND_ABS_Y => "AND_ABS_Y",
        AND_ABS_X => "AND_ABS_X",
        ROL_ABS_X => "ROL_ABS_X",

        // 0x4X
        RTI_IMPL => "RTI_IMPL",
        EOR_X_IND => "EOR_X_IND",
        EOR_ZPG => "EOR_ZPG",
        LSR_ZPG => "LSR_ZPG",
        PHA_IMPL => "PHA_IMPL",
        EOR_IMM => "EOR_IMM",
        LSR_A => "LSR_A",
        JMP_ABS => "JMP_ABS",
        EOR_ABS => "EOR_ABS",
        LSR_ABS => "LSR_ABS",

        // 0x5X
        BVC_REL => "BVC_REL",
        EOR_IND_Y => "EOR_IND_Y",
        EOR_ZPG_X => "EOR_ZPG_X",
        LSR_ZPG_X => "LSR_ZPG_X",
        CLI_IMPL => "CLI_IMPL",
        EOR_ABS_Y => "EOR_ABS_Y",
        EOR_ABS_X => "EOR_ABS_X",
        LSR_ABS_X => "LSR_ABS_X",

        // 0x6X
        RTS_IMPL => "RTS_IMPL",
        ADC_X_IND => "ADC_X_IND",
        ADC_ZPG => "ADC_ZPG",
        ROR_ZPG => "ROR_ZPG",
        PLA_IMPL => "PLA_IMPL",
        ADC_IMM => "ADC_IMM",
        ROR_A => "ROR_A",
        JMP_IND => "JMP_IND",
        ADC_ABS => "ADC_ABS",
        ROR_ABS => "ROR_ABS",

        // 0x7X
        BVS_REL => "BVS_REL",
        ADC_IND_Y => "ADC_IND_Y",
        ADC_ZPG_X => "ADC_ZPG_X",
        ROR_ZPG_X => "ROR_ZPG_X",
        SEI_IMPL => "SEI_IMPL",
        ADC_ABS_Y => "ADC_ABS_Y",
        ADC_ABS_X => "ADC_ABS_X",
        ROR_ABS_X => "ROR_ABS_X",

        // 0x8X
        STA_X_IND => "STA_X_IND",
        STY_ZPG => "STY_ZPG",
        STA_ZPG => "STA_ZPG",
        STX_ZPG => "STX_ZPG",
        DEY_IMPL => "DEY_IMPL",
        TXA_IMPL => "TXA_IMPL",
        STY_ABS => "STY_ABS",
        STA_ABS => "STA_ABS",
        STX_ABS => "STX_ABS",

        // 0x9X
        BCC_REL => "BCC_REL",
        STA_IND_Y => "STA_IND_Y",
        STY_ZPG_X => "STY_ZPG_X",
        STA_ZPG_X => "STA_ZPG_X",
        STX_ZPG_Y => "STX_ZPG_Y",
        TYA_IMPL => "TYA_IMPL",
        STA_ABS_Y => "STA_ABS_Y",
        TXS_IMPL => "TXS_IMPL",
        STA_ABS_X => "STA_ABS_X",

        // 0xAX
        LDY_IMM => "LDY_IMM",
        LDA_X_IND => "LDA_X_IND",
        LDX_IMM => "LDX_IMM",
        LDY_ZPG => "LDY_ZPG",
        LDA_ZPG => "LDA_ZPG",
        LDX_ZPG => "LDX_ZPG",
        TAY_IMPL => "TAY_IMPL",
        LDA_IMM => "LDA_IMM",
        TAX_IMPL => "TAX_IMPL",
        LDY_ABS => "LDY_ABS",
        LDA_ABS => "LDA_ABS",
        LDX_ABS => "LDX_ABS",

        // 0xBX
        BCS_REL => "BCS_REL",
        LDA_IND_Y => "LDA_IND_Y",
        LDY_ZPG_X => "LDY_ZPG_X",
        LDA_ZPG_X => "LDA_ZPG_X",
        LDX_ZPG_Y => "LDX_ZPG_Y",
        CLV_IMPL => "CLV_IMPL",
        LDA_ABS_Y => "LDA_ABS_Y",
        TSX_IMPL => "TSX_IMPL",
        LDY_ABS_X => "LDY_ABS_X",
        LDA_ABS_X => "LDA_ABS_X",
        LDX_ABS_Y => "LDX_ABS_Y",

        // 0xCX
        CPY_IMM => "CPY_IMM",
        CMP_X_IND => "CMP_X_IND",
        CPY_ZPG => "CPY_ZPG",
        CMP_ZPG => "CMP_ZPG",
        DEC_ZPG => "DEC_ZPG",
        INY_IMPL => "INY_IMPL",
        CMP_IMM => "CMP_IMM",
        DEX_IMPL => "DEX_IMPL",
        CPY_ABS => "CPY_ABS",
        CMP_ABS => "CMP_ABS",
        DEC_ABS => "DEC_ABS",

        // 0xDX
        BNE_REL => "BNE_REL",
        CMP_IND_Y => "CMP_IND_Y",
        CMP_ZPG_X => "CMP_ZPG_X",
        DEC_ZPG_X => "DEC_ZPG_X",
        CLD_IMPL => "CLD_IMPL",
        CMP_ABS_Y => "CMP_ABS_Y",
        CMP_ABS_X => "CMP_ABS_X",
        DEC_ABS_X => "DEC_ABS_X",

        // 0xEX
        CPX_IMM => "CPX_IMM",
        SBC_X_IND => "SBC_X_IND",
        CPX_ZPG => "CPX_ZPG",
        SBC_ZPG => "SBC_ZPG",
        INC_ZPG => "INC_ZPG",
        INX_IMPL => "INX_IMPL",
        SBC_IMM => "SBC_IMM",
        NOP_IMPL => "NOP_IMPL",
        CPX_ABS => "CPX_ABS",
        SBC_ABS => "SBC_ABS",
        INC_ABS => "INC_ABS",

        // 0xFX
        BEQ_REL => "BEQ_REL",
        SBC_IND_Y => "SBC_IND_Y",
        SBC_ZPG_X => "SBC_ZPG_X",
        INC_ZPG_X => "INC_ZPG_X",
        SED_IMPL => "SED_IMPL",
        SBC_ABS_Y => "SBC_ABS_Y",
        SBC_ABS_X => "SBC_ABS_X",
        INC_ABS_X => "INC_ABS_X",
        _ => "UNKNOWN",
    }
}

#[derive(Default)]
pub struct CPU {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    bytes_cycles: u32,

    /// Status register, most commonly used flags are C, Z, V, S.
    ///
    /// From highest to lowest bit:
    /// - N = Negative/sign
    /// - V = Overflow
    /// - _ = (unused) (no CPU effect, only for indicating how stack was pushed)
    /// - B = Break    (no CPU effect, only for indicating how stack was pushed)
    ///
    /// - D = Decimal  (on the NES this has no effect, use BCD for arithmetics)
    /// - I = Interrupt (IRQ disable)
    /// - Z = Zero
    /// - C = Carry
    status_register: u8,
    stack_pointer: u8,

    memory: Vec<u8>
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            pc: 0x0000, 
            ac: 0x00, 
            x: 0x00, 
            y: 0x00, 
            bytes_cycles: 0, 
            status_register: 0b00110000, 
            stack_pointer: 0xFD,
            memory: vec![0; 0xFFFF]
        }
    }

    fn set_negative_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, NEGATIVE_FLAG_INDEX, value);
    }

    fn set_overflow_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, OVERFLOW_FLAG_INDEX, value);
    }

    fn set_break_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, BREAK_FLAG_INDEX, value);
    }

    fn set_decimal_flag(&mut self, value: bool) -> () {
        toggle_bit(&mut self.status_register, DECIMAL_FLAG_INDEX, value);
    }

    fn set_interrupt_flag(&mut self, value: bool) -> () {
       toggle_bit(&mut self.status_register, INTERRUPT_FLAG_INDEX, value);
    }

    fn set_zero_flag(&mut self, value: bool) -> () {
        toggle_bit(&mut self.status_register, ZERO_FLAG_INDEX, value);
    }

    // When the addition result is 0 to 255, the carry is cleared.
    // When the addition result is greater than 255, the carry is set.
    // When the subtraction result is 0 to 255, the carry is set.
    // When the subtraction result is less than 0, the carry is cleared.
    fn set_carry_flag(&mut self, set: bool) -> () {
        toggle_bit(&mut self.status_register, CARRY_FLAG_INDEX, set);
    }

    fn push_stack(&mut self, value: u8) -> () {
        self.memory[0x100 + self.stack_pointer as usize] = value;
        self.stack_pointer -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;

        let stack_address = 0x100 + (self.stack_pointer) as usize;
        let value = self.memory[stack_address];

        return value;
    }
    fn get_memory_mapped(&mut self, rom: ROM) -> Vec<u8> {
        let mut memory = vec![0; 0xFFFF];

        println!("Mapping memory with mapper {}", rom.mapper);

        match rom.mapper {
            0 => { 
                // If the PRG ROM is larger than 16kb, copy data into 0x8000 - 0xFFFF, otherwise mirror the data
                if rom.program.len() > 0x4000 {
                    memory.splice(0x8000..0xBFFF, rom.program[0..0x4000].iter().cloned());
                    memory.splice(0xC000..0xFFFF, rom.program[0x4000..rom.program.len()].iter().cloned());
                } else {
                    memory.splice(0x8000..0xBFFF, rom.program[0..rom.program.len()].iter().cloned());
                    memory.splice(0xC000..0xFFFF, rom.program[0..rom.program.len()].iter().cloned());
                }
            }
            n => {
                panic!("Uknown mapper number {}", n);
            }
        }

        return memory;
    }

    pub fn run(&mut self, rom: ROM){
        // Boot sequence
        self.set_break_flag(false);

        // Map ROM program onto RAM
        self.memory = self.get_memory_mapped(rom);

        // Set PC to contents at 0xFFFC - 0xFFFD
        self.pc = 0xC000; // ((self.memory[0xFFFC] as u16) << 8) | self.memory[0x0FFFD] as u16;

        // Interrupts take 7 cycles to complete
        self.bytes_cycles = 7;

        while !check_bit(self.status_register, BREAK_FLAG_INDEX) {
            if DEBUG {
                self.print_debug();
            }

            self.execute_one();

            let one_sec = time::Duration::from_millis(500);
            thread::sleep(one_sec);
        }
    }

    pub fn execute_one(&mut self) {
        // Why is it illegal to index a vec of u8 with a u16? 
        // Why wouldn't it be possible to have a vec of a length that exceeds u16?
        //
        // A usize is 4 bytes on a 32-bit machine, and 8 bytes on a 64-bit machine
        // 4 bytes = 32 bits, 2^32 =              4,294,967,296 =~ 4 GB of RAM
        // 8 bytes = 64 bits, 2^64 = 18,446,744,073,709,551,616 =~ lost of RAM
        //
        // So, casting a u8 (8 bits) or a u16 (16 bits) to usize (32 or 64 bits) should be safe

        let instr: u8 = self.memory[self.pc as usize];
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
                let inst_val = self.memory[pc + 1];
                let calc_addr = self.x + inst_val;
                let value = self.memory[calc_addr as usize];
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
                let inst_val = self.memory[pc + 1];
                let calc_addr = Z_PAGE_BEGIN as u8 + inst_val % Z_PAGE_SIZE as u8;
                let value = self.memory[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 3;
                self.pc += 2;
            },
            ADC_IMM => {
                let inst_val = self.memory[pc + 1];
                let value = inst_val;
                let carry = self.status_register & 0b0000_0001;
                let result: u16 = (self.ac + value + carry).into();

                eprintln!(
                    "{} SR {} Carry {} value {} ac {} = Result {}",
                    to_hex_u16(self.pc),
                    to_binary_u8(self.status_register),
                    carry,
                    value,
                    self.ac,
                    result
                );

                // If both are positive and we end up with a negative number, we've overflowed
                let positive_overflow = value & 0b1000_0000 >> 7 == 0
                    && self.ac & 0b1000_0000 >> 7 == 0
                    && result & 0b1000_0000 >> 7 == 1;

                // If both are negative and we end up with a positive number, we've overflowed
                let negative_overflow = value & 0b1000_0000 >> 7 == 1
                    && self.ac & 0b1000_0000 >> 7 == 1
                    && result & 0b1000_0000 >> 7 == 0;

                let overflow = negative_overflow || positive_overflow;

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(overflow);
                self.set_negative_flag((self.ac & 0b1000_0000) >> 7 == 1);
                self.set_carry_flag((result & 0b1_0000_0000) >> 7 == 1);

                self.bytes_cycles += 2;
                self.pc += 2;
            }
            ADC_ABS => {
                let inst_val = self.memory[pc + 1] + self.memory[pc + 2];
                let calc_addr = inst_val;
                let value = self.memory[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 3;
            },
            ADC_IND_Y => {
                let inst_val = self.memory[pc + 1];
                let calc_addr = self.y + inst_val;
                let value = self.memory[calc_addr as usize];
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
                let inst_val = self.memory[pc + 1];
                let calc_addr = Z_PAGE_BEGIN as u8 + inst_val % Z_PAGE_SIZE as u8 + self.x;
                let value = self.memory[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 2;
            },
            ADC_ABS_Y => {
                let inst_val = self.memory[pc + 1] + self.memory[pc + 2];
                let calc_addr = inst_val + self.y;
                let value = self.memory[calc_addr as usize];
                let carry = self.status_register & 0b00000001;
                let result: u16 = (value + carry).into();

                self.ac = result as u8;
                self.set_zero_flag(self.ac == 0);
                self.set_overflow_flag(result > 0xFF);

                self.bytes_cycles += 4;
                self.pc += 3;
            },
            ADC_ABS_X => {
                let inst_val = self.memory[pc + 1] + self.memory[pc + 2];
                let calc_addr = inst_val + self.x;
                let value = self.memory[calc_addr as usize];
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
            BCC_REL => {
                let carry_set = self.status_register & 0b0000_0001 == 1;
                let same_page = true;

                if !carry_set {
                    let address = pc + 1;
                    // Offset address is a signed 8-bit
                    let value = self.memory[address as usize] as i8;
                    let pc_i16 = (pc as i16) + (value as i16);
                    self.pc = pc_i16 as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BCS_REL => {
                let carry_set = self.status_register & 0b0000_0001 == 1;
                let same_page = true;

                if carry_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BEQ_REL => {
                let zero_set = (self.status_register & 0b0000_0010) >> 1 == 1;
                let same_page = true;

                if zero_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BMI_REL => {
                let negative_set = (self.status_register & 0b1000_0000) >> 7 == 1;
                let same_page = true;

                if negative_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BNE_REL => {
                let zero_set = (self.status_register & 0b0000_0010) >> 1 == 1;
                let same_page = true;

                if !zero_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BPL_REL => {
                let negative_set = (self.status_register & 0b1000_0000) >> 7 == 1;
                let same_page = true;

                if !negative_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BVS_REL => {
                let overflow_set = (self.status_register & 0b0100_0000) >> 6 == 1;
                let same_page = true;

                if overflow_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            BVC_REL => {
                let overflow_set = (self.status_register & 0b0100_0000) >> 6 == 1;
                let same_page = true;

                if !overflow_set {
                    self.pc += self.memory[pc + 1] as u16;
                }

                self.pc += 2;
                self.bytes_cycles += if same_page { 1 } else { 2 };
            }
            // BREAK / INTERRUPT
            BRK_IMPL => {
                // Push return address
                let return_address = self.pc + 2;
                self.push_stack((return_address >> 8) as u8 & 0xFF);
                self.push_stack(return_address as u8 & 0xFF);

                // Push status register with interrupt flag set
                self.set_break_flag(true);

                self.push_stack(self.status_register | 0b0000_1000);

                self.pc += 1;
                self.bytes_cycles += 7;
            }
    
            // CLEAR
            CLC_IMP => {
                self.pc += 1;
                self.set_carry_flag(false);
            }
            CLD_IMPL => {
                self.pc += 1;
                self.set_decimal_flag(false);
            }
            CLI_IMPL => {
                self.pc += 1;
                self.set_interrupt_flag(false);
            }
            CLV_IMPL => {
                self.pc += 1;
                self.set_overflow_flag(false);
            }
            // COMPARE
            CMP_IMM => {
                let result: i16 = self.ac as i16 - self.memory[pc + 1] as i16;

                // TODO: Set the carry flag here ... how?
                self.set_carry_flag(result >= 0);
                self.set_zero_flag(result == 0);
                self.set_negative_flag(((0b1000_0000 & result) >> 7) == 1);
                self.pc += 2;
                self.bytes_cycles += 2;
            }
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
            JMP_ABS => {
                let first = self.memory[pc + 2] as u16;
                let second = self.memory[pc + 1] as u16;
                let address: u16 = (first << 8) | second;
                self.pc = address;
                self.bytes_cycles += 3;
            },
            JMP_IND => {},
    
            // JUMP SUBROUTINE
            JSR_ABS => {
                // Push current PC to stack
                self.push_stack(((pc >> 8) & 0x00FF) as u8);
                self.push_stack((pc & 0xFF) as u8);

                // Calculate next address
                let first = self.memory[(pc + 2) as usize] as u16;
                let second = self.memory[(pc + 1) as usize] as u16;
                let address: u16 = (first << 8) | second;

                self.pc = address;
                self.bytes_cycles += 6;
            }
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
            LDX_ABS => {
                let calc_addr = self.memory[pc + 1];
                self.x = self.memory[calc_addr as usize];
            },
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
            ORA_X_IND => {

                self.bytes_cycles += 6;
            ORA_IMM => {
                self.ac = self.ac | self.memory[pc + 1];
                self.set_negative_flag((self.ac & 0b1000_0000) >> 7 == 1);
                self.set_zero_flag(self.ac == 0);
                self.pc += 2;
                self.bytes_cycles += 2;
            }
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
            PHA_IMPL => {
                self.push_stack(self.ac);
                self.pc += 1;
                self.bytes_cycles += 3;
            }
            PHP_IMPL => {
                // Push the status register with the "B-flag" set, to distinguish how the status register was pushed
                self.push_stack(self.status_register | 0b0011_0000);
                self.pc += 1;
                self.bytes_cycles += 3;
            }

            // PULL
            PLA_IMPL => {
                self.ac = self.pop_stack();
                self.set_zero_flag(self.ac == 0);
                self.set_negative_flag(((0b1000_0000 & self.ac) >> 7) == 1);
                self.pc += 1;
                self.bytes_cycles += 4;
            }
            PLP_IMPL => {
                // Ignore bits 5 and 4
                self.status_register =
                    (self.pop_stack() & 0b1100_1111) | (self.status_register & 0b0011_0000);
                self.pc += 1;
                self.bytes_cycles += 4;
            }
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

    pub fn print_debug(&self) {
        let pc = self.pc as usize;
        let current_instr = self.memory[self.pc as usize];

        println!("{pc}  {instr_number:0>width$}  {instr:<instr_width$} A:{a:0>width$} X:{x:0>width$} Y:{y:0>width$} P:{sr:>width$} ({sr_binary}) SP:{sp:>width$} PPU:{ppuX:>width$},{ppuY:>width$} CYC:{cyc}", 
            pc=to_hex_u16(self.pc),
            instr_number=to_hex_u8(current_instr),
            instr=get_instruction_name(self.memory[pc]),
            a=to_hex_u8(self.ac),
            x=to_hex_u8(self.x),
            y=to_hex_u8(self.y),
            sr=to_hex_u8(self.status_register),
            sr_binary=to_binary_u8(self.status_register),
            sp=to_hex_u8(self.stack_pointer),
            cyc=self.bytes_cycles,
            ppuX=0,
            ppuY=0,
            width=2,
            instr_width=30
        );
    }

    pub fn print_debug_stack(&self) {
        let range = (STACK_BEGIN + self.stack_pointer as usize)..STACK_END;
        eprintln!("STACK (size {}) ", range.len());
        self.print_debug_memory(STACK_BEGIN + self.stack_pointer as usize, STACK_END - 1);
    }

    pub fn print_debug_memory(&self, start: usize, end: usize) {
        print_bytes_table(&self.memory, start, end, 8);
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::core::cpu::{CPU, self};

    use super::ORA_ABS;

    #[test]
    fn test_status_flags_carry() {
        let mut c = CPU::new();
        c.status_register = 0b10000000;
        c.set_carry_flag(true);
        assert_eq!(c.status_register, 0b10000001);

        c.set_carry_flag(false);

    #[test]
    fn test_stack() {
        let mut c = CPU::new();

        assert_eq!(c.stack_pointer, 0xFD);
        c.push_stack(0xAA);

        assert_eq!(c.stack_pointer, 0xFC);

        let v = c.pop_stack();
        assert_eq!(v, 0xAA);
        assert_eq!(c.stack_pointer, 0xFD);
    }

    #[test]
    fn test_status_flags_break() {
        let mut c = CPU::new();
        c.status_register = 0b10000000;
        c.set_break_flag(true);
        assert_eq!(c.status_register, 0b10010000);

        c.set_break_flag(false);
        assert_eq!(c.status_register, 0b10000000);
    }

    #[test]
    fn test_brk_impl() {
        let mut c = CPU::new();
        c.status_register = 0b00000000;

        c.memory[0x4020] = cpu::BRK_IMPL;
        c.execute_one();

        assert_eq!(c.status_register, 0b00010000);
        assert_eq!(c.stack_pointer, 0xFE);
    }

    #[test]
    fn test_ora_x_ind() {
        let mut c = CPU::new();
        let mut ram = vec![0; 0xFFFF];

        ram[0] = cpu::ORA_ABS;
    }
}