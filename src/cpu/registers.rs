use std::fmt::Debug;
use crate::cpu::flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Registers {
    pub fn new() -> Self {
        let mut registers = Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            f: FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: true,
                carry: true,
            },
            h: 0x01,
            l: 0x4d,
            pc: 0,
            sp: 0xfffe,
        };
        if cfg!(feature = "test") {
            registers.set_state_after_boot_rom();
        }
        registers
    }
    pub fn set_state_after_boot_rom(&mut self) {
        self.a = 0x01;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xd8;
        self.f = FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: true,
                carry: true,
            };
        self.h = 0x01;
        self.l = 0x4d;
        self.pc = 0x0100;
        self.sp = 0xfffe;
    }
    pub fn read_8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::F => u8::from(self.f),
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }
    pub fn write_8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.a = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::F => self.f = FlagsRegister::from(value),
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
        }
    }
    pub fn read_16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.a as u16) << 8) | (u8::from(self.f) as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            Reg16::SP => self.sp,
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
            Reg16::SP => { self.sp = value }
        }
    }
    pub fn hli(&mut self) -> u16 {
        let address = self.read_16(Reg16::HL);
        self.write_16(Reg16::HL, address.wrapping_add(1));
        address
    }
    pub fn hld(&mut self) -> u16 {
        let address = self.read_16(Reg16::HL);
        self.write_16(Reg16::HL, address.wrapping_sub(1));
        address
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A:{:02x} F:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} H:{:02x} L:{:02x} SP:{:04x} PC:{:04x}",
               self.a, u8::from(self.f), self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc)
    }
}
