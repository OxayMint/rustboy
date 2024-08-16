use super::{
    cartridge,
    dma::DMA,
    io::{
        lcd::{LCD, LCD_INSTANCE},
        IO_Ram,
    },
    ppu::{self, PPU},
};
use crate::libs::gameboy::ppu::PPU_INSTANCE;
use std::{process::exit, sync::Mutex};

lazy_static! {
    pub static ref MAIN_BUS: Mutex<Bus> = Mutex::new(Bus::new());
    // pub static ref dMAIN_BUS: ssMutex<Bus> = Mutex::new(Bus::new());
}

pub struct Bus {
    pub cart: Option<cartridge::Cartridge>,
    pub wram: Mutex<[u8; 0x2000]>,
    pub hram: Mutex<[u8; 0x80]>,
    pub ioram: Mutex<IO_Ram>,
    pub ie_register: u8,
}

impl Bus {
    fn new() -> Self {
        Bus {
            cart: None,
            wram: Mutex::new([0; 0x2000]),
            hram: Mutex::new([0; 0x80]),
            ioram: Mutex::new(IO_Ram::new()),
            ie_register: 0,
        }
    }
    pub fn _read8(&self, address: usize) -> u8 {
        match address {
            //Char/BG
            0x8000..0xA000 => {
                let ppu = PPU_INSTANCE.lock().unwrap();
                ppu.vram_read(address)
            }
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
                // panic!("read oam from oam!");
                let lcd = LCD_INSTANCE.lock().unwrap();
                if lcd.dma_active() {
                    return 0xFF;
                }
                drop(lcd);
                let ppu = PPU_INSTANCE.lock().unwrap();
                ppu.oam_read(address)
            }
            //useless part
            0xFEA0..0xFF00 => 0,
            //IO
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
    pub fn _read16(&self, address: usize) -> u16 {
        let val: u16 = self._read8(address) as u16 | ((self._read8(address + 1) as u16) << 8);
        return val;
    }
    pub fn _write8(&mut self, address: usize, value: u8) {
        match address {
            //cartriidge ROM
            0x8000..0xA000 => {
                let mut ppu = PPU_INSTANCE.lock().unwrap();
                ppu.vram_write(address, value)
            }
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
                // if lcd.dma_active() {
                //     return;
                // }
                // drop(lcd);
                let mut ppu = PPU_INSTANCE.lock().unwrap();
                ppu.oam_write(address, value)
            }
            //unused part
            0xFEA0..0xFF00 => {}
            0xFF40..=0xFF4B => LCD_INSTANCE.lock().unwrap().write(address, value),
            //IO data
            0xFF00..0xFF80 => self.ioram.lock().unwrap().write(address, value),
            //high ram/zero page
            0xFF80..0xFFFF => self.hram_write(address, value),
            //CPU Interrupt enable register
            0xFFFF => self.ie_register = value,
            _ => println!("wrote nothing, sorry..."),
        }
    }
    pub fn _write16(&mut self, address: usize, value: u16) {
        self._write8(address, value as u8);
        if (value >> 8) != 0 {
            self._write8(address + 1 as usize, (value >> 8) as u8);
        }
    }

    fn wram_write(&mut self, address: usize, value: u8) {
        let mut wram = self.wram.lock().unwrap();
        wram[(address - 0xC000) as usize] = value;
    }
    fn wram_read(&self, address: usize) -> u8 {
        let wram = self.wram.lock().unwrap();
        return wram[(address - 0xC000) as usize];
    }

    fn hram_write(&mut self, address: usize, value: u8) {
        let mut hram = self.hram.lock().unwrap();
        hram[(address - 0xFF80) as usize] = value;
    }
    fn hram_read(&self, address: usize) -> u8 {
        let hram = self.hram.lock().unwrap();
        hram[(address - 0xFF80) as usize]
    }

    pub fn set_cartridge(cart: cartridge::Cartridge) {
        // println!("set_cartridge",);
        MAIN_BUS.lock().unwrap().cart = Some(cart);
    }
    pub fn get_ie_register() -> u8 {
        // println!("get_ie_register",);
        MAIN_BUS.lock().unwrap().ie_register
    }

    pub fn stack_push8(sp: &mut u16, value: u8) {
        // println!("stack_push8 {}", sp);
        let mut bus = MAIN_BUS.lock().unwrap();
        *sp = sp.wrapping_sub(1);
        bus._write8(*sp as usize, value);
    }

    pub fn stack_pop8(sp: &mut u16) -> u8 {
        // println!("stack_pop8 {}", sp);
        let bus = MAIN_BUS.lock().unwrap();
        let val = bus._read8(*sp as usize);
        *sp = sp.wrapping_add(1);
        val
    }

    pub fn stack_push16(sp: &mut u16, value: u16) {
        // println!("stack_push16 {}", sp);
        Bus::stack_push8(sp, (value >> 8) as u8);
        Bus::stack_push8(sp, (value & 0xff) as u8);
    }

    pub fn stack_pop16(sp: &mut u16) -> u16 {
        // println!("stack_pop16 {}", sp);
        Bus::stack_pop8(sp) as u16 | ((Bus::stack_pop8(sp) as u16) << 8)
    }

    // pub fn read
    pub fn read8(address: usize) -> u8 {
        MAIN_BUS.lock().unwrap()._read8(address)
    }
    pub fn read(address: usize) -> u16 {
        // println!("read {}", address);
        MAIN_BUS.lock().unwrap()._read16(address)
    }
    pub fn write8(address: usize, value: u8) {
        // println!("write8 {}", address);
        MAIN_BUS.lock().unwrap()._write8(address, value);
    }
    pub fn write(address: usize, value: u16) {
        // println!("write {}", address);
        MAIN_BUS.lock().unwrap()._write16(address, value);
    }
    pub fn write_oam(&mut self, address: usize, value: u8) {
        let mut ppu = PPU_INSTANCE.lock().unwrap();
        ppu.oam_write(address, value)
    }

    pub fn dma_tick(&mut self) {
        let mut lcd = LCD_INSTANCE.lock().unwrap();
        if let Some((src, dest)) = lcd.dma.tick() {
            drop(lcd);
            let val = self._read8(src);
            self.write_oam(dest, val);
        }
    }
}
