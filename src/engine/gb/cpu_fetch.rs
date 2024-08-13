use crate::libs::gameboy::bus::Bus;

use super::{AddressMode, RegisterType, CPU};

impl CPU {
    pub fn fetch_data(&mut self) {
        let pc = self.regs.pc as usize;
        self.fetched_data = match self.current_instruction.address_mode {
            AddressMode::IMPLIED => 0,
            AddressMode::R => self.read_reg(&self.current_instruction.register_1),

            AddressMode::R_R => self.read_reg(&self.current_instruction.register_2),
            AddressMode::MR_R => {
                self.mem_dest = self.read_reg(&self.current_instruction.register_1) as usize;
                self.destination_is_mem = true;
                if self.current_instruction.register_1 == RegisterType::C {
                    self.mem_dest |= 0xff00;
                }
                self.read_reg(&self.current_instruction.register_2)
            }

            AddressMode::R_D8 => {
                let result = Bus::read8(pc);
                self.emu_cycles(1);
                self.increment_pointer(1);
                result as u16
            }

            AddressMode::D16 | AddressMode::R_D16 => {
                let low = Bus::read8(self.regs.pc as usize) as u16;
                self.emu_cycles(1);
                let hi = Bus::read8(self.regs.pc as usize + 1) as u16;
                self.emu_cycles(1);
                self.increment_pointer(2);
                hi << 8 | low
            }
            AddressMode::R_MR => {
                let mut addr: u16 = self.read_reg(&self.current_instruction.register_2);
                if self.current_instruction.register_2 == RegisterType::C {
                    addr |= 0xFF00;
                }
                let res = Bus::read8(addr as usize);
                self.emu_cycles(1);
                res as u16
            }
            AddressMode::R_HLI => {
                let addr = self.read_reg(&self.current_instruction.register_2);
                let res = Bus::read8(addr as usize);
                self.emu_cycles(1);
                self.set_reg(
                    RegisterType::HL,
                    self.read_reg(&RegisterType::HL).wrapping_add(1),
                );
                res as u16
            }
            AddressMode::R_HLD => {
                let addr = self.read_reg(&self.current_instruction.register_2);
                let res = Bus::read8(addr as usize) as u16;
                self.emu_cycles(1);
                self.set_reg(
                    RegisterType::HL,
                    self.read_reg(&RegisterType::HL).wrapping_sub(1),
                );
                res
            }
            AddressMode::HLI_R => {
                self.mem_dest = self.read_reg(&self.current_instruction.register_1) as usize;
                self.destination_is_mem = true;
                let result = self.read_reg(&self.current_instruction.register_2);
                self.set_reg(
                    RegisterType::HL,
                    self.read_reg(&RegisterType::HL).wrapping_add(1),
                );
                result
            }
            AddressMode::HLD_R => {
                self.mem_dest = self.read_reg(&self.current_instruction.register_1) as usize;
                self.destination_is_mem = true;
                let result = self.read_reg(&self.current_instruction.register_2);
                self.set_reg(
                    RegisterType::HL,
                    self.read_reg(&RegisterType::HL).wrapping_sub(1),
                );
                result
            }
            AddressMode::R_A8 => {
                let res = Bus::read8(self.regs.pc as usize) as u16;
                self.emu_cycles(1);
                self.increment_pointer(1);
                res
            }
            AddressMode::A8_R => {
                self.mem_dest = 0xFF00 | Bus::read8(self.regs.pc as usize) as usize;
                self.destination_is_mem = true;
                self.emu_cycles(1);
                self.increment_pointer(1);
                0
            }
            AddressMode::D8 | AddressMode::HL_SPR => {
                let res = Bus::read8(self.regs.pc as usize) as u16;
                self.emu_cycles(1);
                self.increment_pointer(1);
                res
            }
            AddressMode::A16_R | AddressMode::D16_R => {
                let low = Bus::read8(pc);
                self.emu_cycles(1);
                let hi = Bus::read8(pc + 1);
                self.emu_cycles(1);
                self.mem_dest = (((hi as u16) << 8) | (low as u16)) as usize;
                self.destination_is_mem = true;
                self.increment_pointer(2);
                self.read_reg(&self.current_instruction.register_2)
            }
            AddressMode::MR_D8 => {
                let res = Bus::read8(self.regs.pc as usize) as u16;
                self.emu_cycles(1);
                self.increment_pointer(1);
                self.mem_dest = self.read_reg(&self.current_instruction.register_1) as usize;
                self.destination_is_mem = true;
                res
            }
            AddressMode::MR => {
                self.mem_dest = self.read_reg(&self.current_instruction.register_1) as usize;
                self.destination_is_mem = true;
                let addr = self.read_reg(&self.current_instruction.register_1) as usize;
                let res = Bus::read8(addr) as u16;
                self.emu_cycles(1);

                res
            }
            AddressMode::R_A16 => {
                let low = Bus::read8(pc);
                self.emu_cycles(1);
                let hi = Bus::read8(pc.wrapping_add(1));
                self.emu_cycles(1);

                let addr = ((hi as u16) << 8) | (low as u16);
                self.increment_pointer(2);
                let res = Bus::read8(addr as usize) as u16;
                // println!("R_A16: val {res:04X} at {addr:04X}",);
                self.emu_cycles(1);
                res
            } // _ => 0,
        }
    }
}
