use crate::cpu::CPU;
use crate::cpu::registers::{Reg16, Reg8};

impl CPU {
    pub fn read(&mut self, operand: Operand) -> u8 {
        match operand {
            Operand::RegA => self.registers.read_8(Reg8::A),
            Operand::RegB => self.registers.read_8(Reg8::B),
            Operand::RegC => self.registers.read_8(Reg8::C),
            Operand::RegD => self.registers.read_8(Reg8::D),
            Operand::RegE => self.registers.read_8(Reg8::E),
            Operand::RegH => self.registers.read_8(Reg8::H),
            Operand::RegL => self.registers.read_8(Reg8::L),
            Operand::AddressHL => {
                let address = self.registers.read_16(Reg16::HL);
                self.bus.read_byte(address)
            }
            Operand::Immediate8 => { self.fetch_byte() }
        }
    }
    pub fn write(&mut self, operand: Operand, value: u8) {
        match operand {
            Operand::RegA => self.registers.write_8(Reg8::A, value),
            Operand::RegB => self.registers.write_8(Reg8::B, value),
            Operand::RegC => self.registers.write_8(Reg8::C, value),
            Operand::RegD => self.registers.write_8(Reg8::D, value),
            Operand::RegE => self.registers.write_8(Reg8::E, value),
            Operand::RegH => self.registers.write_8(Reg8::H, value),
            Operand::RegL => self.registers.write_8(Reg8::L, value),
            Operand::AddressHL => {
                let address = self.registers.read_16(Reg16::HL);
                self.bus.write_byte(address, value)
            }
            Operand::Immediate8 => panic!("Kan ikke skrive til umiddelbar operand"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    RegA,
    RegB,
    RegC,
    RegD,
    RegE,
    RegH,
    RegL,
    AddressHL,
    Immediate8,
}