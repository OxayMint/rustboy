use std::ops::{AddAssign, SubAssign};

use sdl2::libc::sleep;

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
        self.active = false;
        self.value = start;
        self.start_delay = 2;
    }
    pub fn tick(&mut self) {
        if self.active {
            return;
        }
        if self.start_delay > 0 {
            self.start_delay.sub_assign(1);
            return;
        }
        Bus::write8(
            self.byte_address,
            Bus::read8(self.value as usize * 0x100 + self.byte_address),
        );
        self.byte_address.add_assign(1);
        self.active = self.byte_address < 0xA0;
        if !self.active {
            unsafe {
                sleep(2);
            }
        }
    }
    pub fn transferring(&self) -> bool {
        self.active
    }
}
