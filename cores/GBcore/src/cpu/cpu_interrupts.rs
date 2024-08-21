use crate::interrupts::InterruptType;

use super::CPU;

impl CPU {
    pub fn handle_interrupts(&mut self, flags: u8) -> u8 {
        for addr in [0x40, 0x48, 0x50, 0x58, 0x60] {
            if let Some(flags) = self.int_check(flags, addr) {
                return flags;
            }
        }
        return flags;
    }

    fn int_check(&mut self, flags: u8, addr: u16) -> Option<u8> {
        let res: u8;
        let int_type = InterruptType::from_address(addr) as u8;
        if (flags & int_type) > 0 && (self.bus.get_ie_register() & int_type) > 0 {
            self.int_handle(addr);
            res = flags & !(int_type);
            self.halted = false;
            self.int_master_enabled = false;
            Some(res)
        } else {
            None
        }
    }
    fn int_handle(&mut self, addr: u16) {
        self.bus.stack_push16(&mut self.regs.sp, self.regs.pc);
        self.regs.pc = addr;
    }
}
