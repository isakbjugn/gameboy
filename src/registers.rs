use crate::flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    pub fn read_16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.a as u16) << 8) | (u8::from(self.f) as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }
    pub fn write_16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = FlagsRegister::from((value & 0xF0) as u8);
            }
            Reg16::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF ) as u8;
            }
            Reg16::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF ) as u8;
            }
            Reg16::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF ) as u8;
            }
        }
    }
}