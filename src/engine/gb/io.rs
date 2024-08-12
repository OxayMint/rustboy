use super::{
    cpu::INT_FLAGS,
    timer::{read_timer_byte, write_timer_byte},
};

pub struct IO_Ram {
    pub ram: [u8; 0x80],

    pub serial_data: [u8; 2],

    pub ly: u8,
}

impl IO_Ram {
    pub fn new() -> Self {
        IO_Ram {
            ram: [0; 0x80],

            serial_data: [0; 2],
            ly: 0,
        }
    }

    pub fn get_ly(&mut self) -> u8 {
        self.ly = self.ly.wrapping_add(1);
        self.ly
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0xFF01 => self.serial_data[0] = value,
            0xFF02 => self.serial_data[1] = value,
            0xFF0F => *INT_FLAGS.lock().unwrap() = value,
            0xFF04..=0xFF07 => write_timer_byte(address, value),
            _ => {}
        }
    }
    pub fn read(&mut self, address: usize) -> u8 {
        match address {
            0xFF01 => self.serial_data[0],
            0xFF02 => self.serial_data[1],
            0xFF00 => 0,
            0xFF0F => *INT_FLAGS.lock().unwrap(),
            0xFF44 => self.get_ly(),
            0xFF04..=0xFF07 => read_timer_byte(address),
            _ => 0,
        }
        // return self.ioram[address as usize];
    }
}
