use crate::libs::cartridge;

use super::{
    cpu::INT_FLAGS,
    timer::{read_timer_byte, write_timer_byte, Timer},
    SetBytes,
};

pub struct Memory {
    pub cart: cartridge::Cartridge,
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x80],
    // pub ioram: [u8; 0x80],
    pub ie_register: u8,
    pub serial_data: [u8; 2],

    pub ly: u8,
}

impl Memory {
    pub fn new(cart: cartridge::Cartridge) -> Self {
        Memory {
            cart: cart,
            wram: [0; 0x2000],
            hram: [0; 0x80],
            // ioram: [0; 0x80],
            ie_register: 0,
            serial_data: [0; 2],
            ly: 0,
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
    pub fn read(&mut self, address: usize) -> u8 {
        match address {
            //cartriidge ROM
            0..0x8000 => self.cart.read(address),
            //Char/BG
            0x8000..0xA000 => {
                // todo!("OAM not implemented yet")
                0
            }
            //Ext ram
            0xA000..0xC000 => self.cart.read(address),
            //Working RAM
            0xC000..0xE000 => self.wram_read(address),
            //reserved echo RAM. useless for us
            0xE000..0xFE00 => 0,
            //OAM
            0xFE00..0xFEA0 => {
                // todo!("OAM not implemented yet")
                0
            }
            //useless part
            0xFEA0..0xFF00 => 0,
            //IO
            0xFF00..0xFF80 => self.io_read(address),
            //high ram/zero page
            0xFF80..0xFFFF => self.hram_read(address),
            //CPU Interrupt enable register
            0xFFFF => self.get_ie_register(),
            _ => panic!("something wrong here! address: {address}"),
        }
        // return 0;
    }
    pub fn write(&mut self, address: usize, value: u16) {
        self.write8(address, value as u8);
        if (value >> 8) != 0 {
            self.write8(address + 1 as usize, (value >> 8) as u8);
        }
    }
    pub fn write8(&mut self, address: usize, value: u8) {
        match address {
            //cartriidge ROM
            0..0x8000 => self.cart.write(address, value),
            0x8000..0xA000 => {
                //Char/BG
                // todo!("OAM not implemented yet")
            }
            0xA000..0xC000 => {
                //External RAM
                // self.cart.write(address, value);
            }
            //Working RAM
            0xC000..0xE000 => self.wram_write(address, value),
            0xE000..0xFE00 => {}
            0xFE00..0xFEA0 => {
                //OAM
                // todo!("OAM not implemented yet")
            }
            //unused part
            0xFEA0..0xFF00 => {}
            //IO data
            0xFF00..0xFF80 => self.io_write(address, value),
            //high ram/zero page
            0xFF80..0xFFFF => self.hram_write(address, value),
            //CPU Interrupt enable register
            0xFFFF => self.set_ie_register(value),
            _ => println!("wrote nothing, sorry..."),
        }
    }
    pub fn read16(&mut self, address: usize) -> u16 {
        let mut val: u16 = 0;
        val.set_low(self.read(address));
        val.set_high(self.read(address + 1));
        return val;
    }
    // pub fn write16(&mut self, address: usize, value: u16) {
    //     self.write8(address + 1, (value >> 8) as u8);
    //     self.write8(address, value as u8);
    // }
    pub fn wram_write(&mut self, address: usize, value: u8) {
        self.wram[(address - 0xC000) as usize] = value;
    }
    pub fn wram_read(&self, address: usize) -> u8 {
        return self.wram[(address - 0xC000) as usize];
    }

    pub fn hram_write(&mut self, address: usize, value: u8) {
        self.hram[(address - 0xFF80) as usize] = value;
    }
    pub fn hram_read(&self, address: usize) -> u8 {
        let val = self.hram[(address - 0xFF80) as usize];
        // println!("{address:#04X}:{val:#04X}");
        val
    }
    pub fn io_write(&mut self, address: usize, value: u8) {
        // println!("WRITING TO IO RAM: {address:#04x}: {value}");
        if address == 0xFF01 {
            self.serial_data[0] = value;
            return;
        }
        if address == 0xFF02 {
            self.serial_data[1] = value;
            return;
        }
        if address == 0xFF0F {
            let mut flags = INT_FLAGS.lock().unwrap();
            *flags = value;
            return;
        }
        if address >= 0xFF04 && address <= 0xFF07 {
            println!("writing timer to address {address:04X}...");
            write_timer_byte(address, value);
            println!("timer: {value:08b} written");
            return;
        }
        // println!("Unsupported memory write  {address:#4X}");
        // self.ioram[address as usize] = value;
    }
    pub fn io_read(&mut self, address: usize) -> u8 {
        // println!("READING IO RAM: {address}",);
        if address == 0xFF01 {
            let res = self.serial_data[0];
            // println!("READ 0XFF01: {res}");
            return res;
        } else if address == 0xFF02 {
            let res = self.serial_data[1];
            // println!("READ 0XFF02: {res}");
            return res;
        } else if address == 0xFF00 {
            return 0;
        } else if address == 0xFF0F {
            // println!("reading interrupt flags {:010b}", self.interrupt_flags);
            // exit(0);
            return *INT_FLAGS.lock().unwrap();
        } else if address == 0xFF44 {
            // let res = self.ly;
            // self.ly = self.ly.wrapping_add(1);

            // println!("0xFF04: {:#04X}", res);
            // exit(0);
            return 0x90;
        } else if address >= 0xFF04 && address <= 0xFF07 {
            println!("reading  timer...");
            let val = read_timer_byte(address);
            println!("timer at {address:04X}: {val}");

            return val;
        } else {
            // panic!("Unsupported memory read  {address:#4X}");
            // panic!();
            return 0;
        }
        // return self.ioram[address as usize];
    }

    pub fn get_ie_register(&self) -> u8 {
        self.ie_register
    }
    pub fn set_ie_register(&mut self, val: u8) {
        self.ie_register = val;
    }

    pub fn stack_push(&mut self, sp: &mut u16, value: u8) {
        *sp = sp.wrapping_sub(1);
        self.write8(*sp as usize, value);
    }
    pub fn stack_pop(&mut self, sp: &mut u16) -> u8 {
        let val = self.read(*sp as usize);
        *sp = sp.wrapping_add(1);
        val
    }
    pub fn stack_push16(&mut self, sp: &mut u16, value: u16) {
        self.stack_push(sp, (value >> 8) as u8);
        self.stack_push(sp, (value & 0xff) as u8);
    }
    pub fn stack_pop16(&mut self, sp: &mut u16) -> u16 {
        let val = u16::from_pair(self.stack_pop(sp), self.stack_pop(sp));
        // println!("popped {:#X}", val);
        val
    }
}
