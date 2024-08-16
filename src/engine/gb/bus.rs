use super::{
    cartridge,
    io::{lcd::LCD, IO_Ram},
    ppu::PPU,
    timer::Timer,
};

use std::sync::Mutex;

pub struct Bus {
    pub cart: Option<cartridge::Cartridge>,
    pub ppu: PPU,
    pub timer: Timer,
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x80],
    pub ioram: Mutex<IO_Ram>,
    pub ie_register: u8,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cart: None,
            timer: Timer::new(),
            ppu: PPU::new(),
            wram: [0; 0x2000],
            hram: [0; 0x80],
            ioram: Mutex::new(IO_Ram::new()),
            ie_register: 0,
        }
    }
    pub fn read8(&self, address: usize) -> u8 {
        match address {
            //Char/BG
            0x8000..0xA000 => self.ppu.vram_read(address),
            //cartriidge ROM or Ext ram
            0..0xC000 => {
                if let Some(cart) = &self.cart {
                    cart.read(address)
                } else {
                    panic!("Won't work without a cartridge")
                }
            }
            //Working RAM
            0xC000..0xE000 => self.wram_read(address),
            //reserved echo RAM. useless for us
            0xE000..0xFE00 => 0,
            //OAM
            0xFE00..0xFEA0 => {
                // if LCD_IN
                if self.ppu.lcd.dma_active() {
                    return 0xFF;
                }

                self.ppu.oam_read(address)
            }
            //useless part
            0xFEA0..0xFF00 => 0,

            //IO section. LCD and TIMER are separated from it
            0xFF40..=0xFF4B => self.ppu.lcd.read(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF00..0xFF80 => {
                let mut ioram = self.ioram.lock().unwrap();
                ioram.read(address)
            }

            //high ram/zero page
            0xFF80..0xFFFF => self.hram_read(address),
            //CPU Interrupt enable register
            0xFFFF => self.ie_register,
            _ => panic!("something wrong here! address: {address}"),
        }
        // return 0;
    }
    pub fn read16(&self, address: usize) -> u16 {
        let val: u16 = self.read8(address) as u16 | ((self.read8(address + 1) as u16) << 8);
        return val;
    }
    pub fn write8(&mut self, address: usize, value: u8) {
        match address {
            //cartriidge ROM
            0x8000..0xA000 => self.ppu.vram_write(address, value),
            //order matters here since ppu wram addres range is inside the cart range
            0..0xC000 => {
                if let Some(cart) = &self.cart {
                    cart.write(address, value)
                } else {
                    panic!("Won't work without a cartridge")
                }
            }
            //Working RAM
            0xC000..0xE000 => self.wram_write(address, value),
            0xE000..0xFE00 => {}
            0xFE00..0xFEA0 => {
                // let lcd = LCD_INSTANCE.lock().unwrap();
                if self.ppu.lcd.dma_active() {
                    return;
                }
                // drop(lcd);
                self.ppu.oam_write(address, value)
            }
            //unused part
            0xFEA0..0xFF00 => {}
            //lcd part of io
            0xFF40..=0xFF4B => self.ppu.lcd.write(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            //IO data
            0xFF00..0xFF80 => self.ioram.lock().unwrap().write(address, value),
            //high ram/zero page
            0xFF80..0xFFFF => self.hram_write(address, value),
            //CPU Interrupt enable register
            0xFFFF => self.ie_register = value,
            _ => println!("wrote nothing, sorry..."),
        }
    }
    pub fn write16(&mut self, address: usize, value: u16) {
        self.write8(address, value as u8);
        if (value >> 8) != 0 {
            self.write8(address + 1 as usize, (value >> 8) as u8);
        }
    }

    fn wram_write(&mut self, address: usize, value: u8) {
        self.wram[(address - 0xC000) as usize] = value;
    }
    fn wram_read(&self, address: usize) -> u8 {
        return self.wram[(address - 0xC000) as usize];
    }

    fn hram_write(&mut self, address: usize, value: u8) {
        self.hram[(address - 0xFF80) as usize] = value;
    }
    fn hram_read(&self, address: usize) -> u8 {
        self.hram[(address - 0xFF80) as usize]
    }

    pub fn set_cartridge(&mut self, cart: cartridge::Cartridge) {
        // println!("set_cartridge",);
        self.cart = Some(cart);
    }
    pub fn get_ie_register(&self) -> u8 {
        // println!("get_ie_register",);
        self.ie_register
    }

    pub fn stack_push8(&mut self, sp: &mut u16, value: u8) {
        // println!("stack_push8 {}", sp);
        *sp = sp.wrapping_sub(1);
        self.write8(*sp as usize, value);
    }

    pub fn stack_pop8(&mut self, sp: &mut u16) -> u8 {
        // println!("stack_pop8 {}", sp);
        let val = self.read8(*sp as usize);
        *sp = sp.wrapping_add(1);
        val
    }

    pub fn stack_push16(&mut self, sp: &mut u16, value: u16) {
        // println!("stack_push16 {}", sp);
        self.stack_push8(sp, (value >> 8) as u8);
        self.stack_push8(sp, (value & 0xff) as u8);
    }

    pub fn stack_pop16(&mut self, sp: &mut u16) -> u16 {
        self.stack_pop8(sp) as u16 | ((self.stack_pop8(sp) as u16) << 8)
    }

    pub fn write_oam(&mut self, address: usize, value: u8) {
        self.ppu.oam_write(address, value)
    }

    pub fn dma_tick(&mut self) {
        if let Some((src, dest)) = self.ppu.lcd.dma.tick() {
            let val = self.read8(src);
            self.write_oam(dest, val);
        }
    }
}
