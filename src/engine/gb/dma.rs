use std::ops::{AddAssign, SubAssign};

use super::bus::Bus;

pub struct DMA {
    pub active: bool,
    pub byte_address: usize,
    pub value: u8,
    pub start_delay: u8,
}
impl DMA {
    pub fn new() -> DMA {
        DMA {
            active: true,
            byte_address: 0,
            value: 0,
            start_delay: 0,
        }
    }

    pub fn start(&mut self, start: u8) {
        println!("dma start");
        self.active = false;
        self.byte_address = 0;
        self.start_delay = 2;
        self.value = start;
    }
    pub fn tick(&mut self) -> Option<(usize, usize)> {
        if self.active {
            return None;
        }
        if self.start_delay > 0 {
            self.start_delay.sub_assign(1);
            return None;
        }

        let source_address = self.value as usize * 0x100 + self.byte_address;
        let dest_address = 0xFE00 + self.byte_address;

        self.byte_address.add_assign(1);
        if self.byte_address >= 0xA0 {
            self.active = false;
        }

        Some((source_address, dest_address))
    }
    pub fn transferring(&self) -> bool {
        self.active
    }
}
