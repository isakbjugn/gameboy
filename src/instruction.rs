
pub enum Instruction {
    // https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7
    
    // Load instructions
    LD(LoadType),
    
    // 8-bit arithmetic instructions
    
    ADD(ArithmeticTarget),
    JP(JumpTest),
    CALL(JumpTest),
    REST(JumpTest),
    NOP,
    HALT
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_not_prefixed(byte)
        }
    }
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        Some(Instruction::ADD(ArithmeticTarget::C))
    }
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            
        }
        Some(Instruction::ADD(ArithmeticTarget::C))
    }
}

pub enum ArithmeticTarget {
    A, B, C, D, E, F, H, L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource)
}