#[path = "lcd.rs"]
pub mod lcd;
use lcd::{LCD, LCD_INSTANCE};

use super::{
    cpu::INT_FLAGS,
    timer::{read_timer_byte, write_timer_byte},
};

pub struct IO_Ram {
    // pub ram: [u8; 0x80],
    pub serial_data: [u8; 2],
}

impl IO_Ram {
    pub fn new() -> Self {
        IO_Ram {
            // ram: [0; 0x80],
            serial_data: [0; 2],
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0xFF01 => self.serial_data[0] = value,
            0xFF02 => self.serial_data[1] = value,
            0xFF04..=0xFF07 => write_timer_byte(address, value),
            0xFF0F => *INT_FLAGS.lock().unwrap() = value,
            0xFF40..=0xFF4B => LCD_INSTANCE.lock().unwrap().write(address, value),
            _ => {}
        }
    }
    pub fn read(&mut self, address: usize) -> u8 {
        match address {
            0xFF01 => self.serial_data[0],
            0xFF02 => self.serial_data[1],
            0xFF0F => *INT_FLAGS.lock().unwrap(),
            0xFF04..=0xFF07 => read_timer_byte(address),
            0xFF40..=0xFF4B => LCD_INSTANCE.lock().unwrap().read(address),
            _ => 0,
        }
    }
}
