use crate::libs::cartridge;
use std::process::exit;

use super::SetBytes;

pub struct Memory {
    pub cart: cartridge::Cartridge,
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x80],
    pub ioram: [u8; 0x80],
    pub ie_register: u8,
}

impl Memory {
    pub fn new(cart: cartridge::Cartridge) -> Self {
        Memory {
            cart: cart,
            wram: [0; 0x2000],
            hram: [0; 0x80],
            ioram: [0; 0x80],
            ie_register: 0,
        }
    }
    // 0x0000 - 0x3FFF : ROM Bank 0
    // 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
    // 0x8000 - 0x97FF : CHR RAM
    // 0x9800 - 0x9BFF : BG Map 1
    // 0x9C00 - 0x9FFF : BG Map 2
    // 0xA000 - 0xBFFF : Cartridge RAM
    // 0xC000 - 0xCFFF : RAM Bank 0
    // 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
    // 0xE000 - 0xFDFF : Reserved - Echo RAM
    // 0xFE00 - 0xFE9F : Object Attribute Memory
    // 0xFEA0 - 0xFEFF : Reserved - Unusable
    // 0xFF00 - 0xFF7F : I/O Registers
    // 0xFF80 - 0xFFFE : Zero Page
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0..0x8000 => {
                //cartriidge ROM
                return self.cart.read(address);
            }
            0x8000..0xA000 => {
                //Char/BG
                todo!("OAM not implemented yet")
            }
            0xA000..0xC000 => {
                //Ext ram
                return self.cart.read(address);
            }
            0xC000..0xE000 => {
                //Working RAM
                return self.wram_read(address - 0xC000);
            }
            0xE000..0xFE00 => {
                //reserved echo RAM. useless for us
                return 0;
            }
            0xFE00..0xFEA0 => {
                //OAM
                todo!("OAM not implemented yet")
            }
            0xFEA0..0xFF00 => {
                //useless part
                return 0;
            }
            0xFF00..0xFF80 => {
                //IO data
                todo!("IO not implemented yet");

                // self.ioram_read(address - 0xFF00)
            }
            0xFF80..0xFFFF => {
                //high ram/zero page
                todo!("hiram not done")
            }
            //CPU Interrupt enable register
            0xFFFF => self.ie_register,
            _ => {
                return self.hram_read(address - 0xFF80);
            }
        }
        // return 0;
    }
    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0..0x8000 => {
                //cartriidge ROM
                self.cart.write(address, value);
            }
            0x8000..0xA000 => {
                //Char/BG
                todo!("OAM not implemented yet")
            }
            0xA000..0xC000 => {
                //External RAM
                self.cart.write(address, value);
            }
            0xC000..0xE000 => {
                //Working RAM
                self.wram_write(address - 0xC000, value);
            }
            0xE000..0xFE00 => {
                //unused part
            }
            0xFE00..0xFEA0 => {
                //OAM
                todo!("OAM not implemented yet")
            }
            0xFEA0..0xFF00 => {
                //unused part
            }
            0xFF00..0xFF80 => {
                //IO data
                // todo!("{address:#4X} - IO not implemented yet");

                // self.ioram_write(address - 0xFF00, value);
            }
            0xFF80..0xFFFF => {
                //high ram/zero page
                self.hram_write(address - 0xFF80, value);
            }
            //CPU Interrupt enable register
            0xFFFF => self.ie_register = value,

            _ => {}
        }
    }

    pub fn read16(&self, address: usize) -> u16 {
        let mut val: u16 = 0;
        val.set_low(self.read(address));
        val.set_high(self.read(address + 1));
        return val;
    }
    pub fn write16(&mut self, address: usize, mut value: u16) {
        let separated = value.separate_bytes();
        self.write(address, separated.0);
        self.write(address + 1, separated.1);
    }
    pub fn wram_write(&mut self, address: usize, value: u8) {
        self.wram[address as usize] = value;
    }
    pub fn wram_read(&self, address: usize) -> u8 {
        return self.wram[address as usize];
    }

    pub fn hram_write(&mut self, address: usize, value: u8) {
        self.hram[address as usize] = value;
    }
    pub fn hram_read(&self, address: usize) -> u8 {
        return self.hram[address as usize];
    }
    pub fn ioram_write(&mut self, address: usize, value: u8) {
        self.ioram[address as usize] = value;
    }
    pub fn ioram_read(&self, address: usize) -> u8 {
        return self.ioram[address as usize];
    }

    pub fn get_ie_register(&self) -> u8 {
        self.ie_register
    }
    pub fn set_ie_register(&mut self, val: u8) {
        self.ie_register = val;
    }

    pub fn stack_push(&mut self, sp: &mut u16, value: u8) {
        *sp = sp.wrapping_sub(1);
        self.write(*sp as usize, value);
    }
    pub fn stack_pop(&mut self, sp: &mut u16) -> u8 {
        let val = self.read(*sp as usize);
        *sp = sp.wrapping_add(1);
        val
    }
    pub fn stack_push16(&mut self, sp: &mut u16, value: u16) {
        // println!("pushed {:#X}", value);
        self.stack_push(sp, (value >> 8) as u8);
        self.stack_push(sp, value as u8);
    }
    pub fn stack_pop16(&mut self, sp: &mut u16) -> u16 {
        let val = u16::from_pair(self.stack_pop(sp), self.stack_pop(sp));
        // println!("popped {:#X}", val);
        val
    }
}
