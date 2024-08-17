#[path = "lcd.rs"]
pub mod lcd;

use std::sync::{Arc, Mutex};

use super::{
    cpu::INT_FLAGS,
    input::{Input, InputManager},
};

pub struct IOManager {
    // pub ram: [u8; 0x80],
    pub input: InputManager,
    pub input_requested: bool,
    pub serial_data: [u8; 2],
}

impl IOManager {
    pub fn new() -> Self {
        IOManager {
            // ram: [0; 0x80],
            input: InputManager::new(),
            input_requested: false,
            serial_data: [0; 2],
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0xFF00 => {
                self.input.set_mode(value);
            }
            0xFF01 => self.serial_data[0] = value,
            0xFF02 => self.serial_data[1] = value,
            0xFF0F => *INT_FLAGS.lock().unwrap() = value,
            _ => {}
        }
    }
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0xFF00 => self.input.gamepad_get_output(),
            0xFF01 => self.serial_data[0],
            0xFF02 => self.serial_data[1],
            0xFF0F => *INT_FLAGS.lock().unwrap(),
            _ => 0,
        }
    }
    pub fn update_input(&self, input: Arc<Input>) {}
}
