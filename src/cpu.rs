mod decode;
mod execute;
mod flags_register;
mod registers;
mod interrupt_master_enable;
mod read_write;

use std::fmt::Debug;
use crate::cartridge::Cartridge;
use crate::address_bus::AddressBus;
use registers::Registers;
use crate::cpu::interrupt_master_enable::InterruptMasterEnable;

pub struct CPU {
    registers: Registers,
    pub bus: AddressBus,
    is_halted: bool,
    interrupt_master_enable: InterruptMasterEnable,
}

impl CPU {
    pub fn new(cartridge_path: &str) -> Result<Self, &'static str> {
        let cartridge = Cartridge::from_path(cartridge_path.into())?;
        
        Ok(Self {
            registers: Registers::new(),
            bus: AddressBus::new(cartridge),
            is_halted: false,
            interrupt_master_enable: InterruptMasterEnable::new(),
        })
    }
    pub fn cycle(&mut self) -> u32 {
        let interrupts_handle_time = self.handle_interrupts();
        if interrupts_handle_time > 0 {
            return interrupts_handle_time
        }
        if self.is_halted {
            return 1
        }

        self.call()
    }
    fn handle_interrupts(&mut self) -> u32 {
        let interrupt_master_enable = self.interrupt_master_enable.read();
        if !interrupt_master_enable && !self.is_halted {
            return 0
        }
        let interrupts = self.bus.interrupt_enable_register & self.bus.interrupt_flag;
        if interrupts == 0 {
            return 0
        }

        self.is_halted = false;
        if !interrupt_master_enable { return 0 }

        let highest_priority_bit = interrupts.trailing_zeros();
        if highest_priority_bit > 4 {
            panic!("Ugyldig interrupt-verdi")
        }
        self.bus.interrupt_flag &= !(1 << highest_priority_bit);
        self.push_stack(self.registers.pc);

        self.registers.pc = match highest_priority_bit {
            0 => 0x0040, // VBLANK
            1 => 0x0048, // LCD STAT
            2 => 0x0050, // Timer
            3 => 0x0058, // Serial
            4 => 0x0060, // Joypad
            _ => unreachable!()
        };

        5
    }
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }
    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.read_word(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        word
    }
    fn pop_stack(&mut self) -> u16 {
        let value = self.bus.read_word(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        value
    }
    fn push_stack(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.write_word(self.registers.sp, value);
    }
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc_mem: Vec<u8> = Vec::from_iter((0..4).map(|pc_offset| self.bus.read_byte(self.registers.pc + pc_offset)));
        write!(f, "{:?} PCMEM:{:02x},{:02x},{:02x},{:02x}",
            self.registers, pc_mem[0], pc_mem[1], pc_mem[2], pc_mem[3])
    }
}
