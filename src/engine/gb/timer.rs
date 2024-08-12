use lazy_static::lazy_static;
use std::{ops::AddAssign, process::exit, sync::Mutex};

lazy_static! {
    pub static ref TIMER: Mutex<Timer> = Mutex::new(Timer::new());
}

pub struct Timer {
    div: u16,         // FF04 - Divider register
    tima: u8,         // FF05 - Timer counter
    tma: u8,          // FF06 - Timer modulo
    tac: u8,          // FF07 - Timer control
    div_cycles: u32,  // Internal counter for DIV
    tima_cycles: u32, // Internal counter for TIMA
}

impl Timer {
    fn new() -> Self {
        Timer {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            div_cycles: 0,
            tima_cycles: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut interrupt_requested = false;
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);
        let mut timer_update = false;

        // if self.tac & 0x04 != 0 {
        match self.tac & 0b11 {
            0b00 => timer_update = (prev_div & (1 << 9)) > 0 && ((self.div & (1 << 9)) == 0),
            0b01 => timer_update = (prev_div & (1 << 3)) > 0 && ((self.div & (1 << 3)) == 0),
            0b10 => timer_update = (prev_div & (1 << 5)) > 0 && ((self.div & (1 << 5)) == 0),
            0b11 => timer_update = (prev_div & (1 << 7)) > 0 && ((self.div & (1 << 7)) == 0),
            _ => println!("this can't be"),
        }
        // }

        if timer_update && ((self.tac & 0x4) > 0) {
            // exit(0);
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0xFF {
                self.tima = self.tma;
                interrupt_requested = true;
            }
        }

        interrupt_requested
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Invalid timer address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            // 0xFF07 => self.tac = value & 0x07,
            0xFF07 => self.tac = value,
            _ => panic!("Invalid timer address: {:04X}", address),
        }
    }

    pub fn reset(&mut self) {
        self.div = 0;
        self.tima = 0;
        self.tma = 0;
        self.tac = 0;
        self.div_cycles = 0;
        self.tima_cycles = 0;
    }
}

// Helper functions to easily access the timer
pub fn tick_timer() -> bool {
    TIMER.lock().unwrap().tick()
}

pub fn read_timer_byte(address: usize) -> u8 {
    TIMER.lock().unwrap().read_byte(address)
}

pub fn write_timer_byte(address: usize, value: u8) {
    TIMER.lock().unwrap().write_byte(address, value)
}

pub fn reset_timer() {
    TIMER.lock().unwrap().reset()
}
