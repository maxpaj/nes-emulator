use crate::debug;

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
const ADC_X_IMPL: u8 = 0x61;
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

struct CPU {
    pc: u8,
    reg_a: u8,
    reg_b: u8,
    reg_x: u8,
    reg_y: u8,
    flags: u8,
}

pub fn run(program: Vec<u8>) -> Vec<u8> {
    let mut memory: Vec<u8> = vec![0; 0xffff as usize];

    let mut cpu = CPU {
        pc: 0x0,
        reg_a: 0x0,
        reg_b: 0x0,
        reg_x: 0x0,
        reg_y: 0x0,
        flags: 0b00000000,
    };

    let mut running = true;

    while ((cpu.pc as usize) < program.len()) && running {
        let instr: u8 = program[cpu.pc as usize];

        match instr {
            // 0x0X
            BRK_IMPL => {
                println!("BRK_IMPL");
            }
            ORA_X_IND => { println!("ORA_X_IND"); }
            ORA_ZPG => { println!("ORA_ZPG"); }
            ASL_ZPG => { println!("ASL_ZPG"); }
            PHP_IMPL => { println!("PHP_IMPL"); }
            ORA_IMM => { println!("ORA_IMM"); }
            ASL_ACC => { println!("ASL_ACC"); }
            ORA_ABS => { println!("ORA_ABS"); }
            ASL_ABS => { println!("ASL_ABS"); }

            // 0x1X
            BPL_REL => { println!("BPL_REL"); }
            ORA_IND => { println!("ORA_IND"); }
            ORA_ZPG_X => { println!("ORA_ZPG_X"); }
            ASL_ZPG_X => { println!("ASL_ZPG_X"); }
            CLC_IMP => { println!("CLC_IMP"); }
            ORA_ABS_Y => { println!("ORA_ABS_Y"); }
            ORA_ABS_X => { println!("ORA_ABS_X"); }
            ASL_ABS_X => { println!("ASL_ABS_X"); }

            // 0x2X
            JSR_ABS => { println!("JSR_ABS"); }
            AND_X_IND => { println!("AND_X_IND"); }
            BIT_ZPG => { println!("BIT_ZPG"); }
            AND_ZPG => { println!("AND_ZPG"); }
            ROL_ZPG => { println!("ROL_ZPG"); }
            PLP_IMPL => { println!("PLP_IMPL"); }
            AND_IMM => { println!("AND_IMM"); }
            ROL_A => { println!("ROL_A"); }
            BIT_ABS => { println!("BIT_ABS"); }
            AND_ABS => { println!("AND_ABS"); }
            ROL_ABS => { println!("ROL_ABS"); }

            // 0xAX
            LDY_IMM => { println!("LDY_IMM"); }
            LDY_ZPG => { println!("LDY_ZPG"); }
            LDY_ABS => { println!("LDY_ABS"); }

            // 0xBX
            LDY_X => { println!("LDY_X"); }
            LDY_X_ZPG => { println!("LDY_X_ZPG"); }
            LDY_IMM => { println!("LDY_IMM"); }

            //0x8X
            STA_ABS => { println!("STA_ABS"); }
            _ => println!("({:x}) Missing instruction", instr),
        }

        cpu.pc += 1;
    }

    return memory;
}