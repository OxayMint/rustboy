use crate::libs::gameboy::{bus::Bus, interrupts::InterruptType};

use super::{CPU, INT_FLAGS};

impl CPU {
    pub fn handle_interrupts(&mut self) {
        // println!("handling interrupt {}", MAIN_BUS.interrupt_flags);

        println!("int checking");
        let mut flags = INT_FLAGS.lock().unwrap();
        if self.int_check(&mut flags, 0x40, InterruptType::VBLANK) {
            println!("int checked for VBLANK");
        } else if self.int_check(&mut flags, 0x48, InterruptType::LCD_STAT) {
            println!("int checked for LCD_STAT");
        } else if self.int_check(&mut flags, 0x50, InterruptType::TIMER) {
            println!("int checked for TIMER");
        } else if self.int_check(&mut flags, 0x58, InterruptType::SERIAL) {
            println!("int checked for SERIAL");
        } else if self.int_check(&mut flags, 0x60, InterruptType::JOYPAD) {
            println!("int checked for JOYPAD");
        }
    }

    fn int_check(&mut self, flags: &mut u8, addr: u16, int_type: InterruptType) -> bool {
        if (*flags & int_type as u8) > 0 && (Bus::get_ie_register() & int_type as u8) > 0 {
            self.int_handle(addr);
            *flags &= !(int_type as u8);
            self.halted = false;
            self.int_master_enabled = false;
            true
        } else {
            false
        }
    }
    fn int_handle(&mut self, addr: u16) {
        Bus::stack_push16(&mut self.regs.sp, self.regs.pc);
        self.regs.pc = addr;
    }
}
