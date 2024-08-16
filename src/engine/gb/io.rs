#[path = "lcd.rs"]
pub mod lcd;

use super::{cpu::INT_FLAGS, timer::Timer};

pub struct IO_Ram {
    // pub ram: [u8; 0x80],
    pub input: u8,
    pub serial_data: [u8; 2],
}

impl IO_Ram {
    pub fn new() -> Self {
        IO_Ram {
            // ram: [0; 0x80],
            input: 0xFF,
            serial_data: [0; 2],
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0xFF00 => self.input = value,
            0xFF01 => self.serial_data[0] = value,
            0xFF02 => self.serial_data[1] = value,
            0xFF0F => *INT_FLAGS.lock().unwrap() = value,
            _ => {}
        }
    }
    pub fn read(&mut self, address: usize) -> u8 {
        match address {
            0xFF00 => self.input,
            0xFF01 => self.serial_data[0],
            0xFF02 => self.serial_data[1],
            0xFF0F => *INT_FLAGS.lock().unwrap(),
            _ => 0,
        }
    }
}
