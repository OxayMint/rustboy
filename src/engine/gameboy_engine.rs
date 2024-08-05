use crate::libs::cartridge::Cartridge;
use crate::libs::cpu::CPU;
use crate::libs::instruction::*;
use crate::libs::memory::Memory;
use crate::libs::rendering::Renderer;
use crate::libs::timer::Timer;

use super::SetBytes;

extern crate sdl2;

/*
  Emulator components:

  |Cart|
  |CPU|
  |Address Bus|
  |PPU|
  |Timer|

*/
pub struct GameBoyEngine {
    pub paused: bool,
    pub running: bool,
    pub cpu: CPU,
    pub renderer: Renderer,
    pub timer: Timer,
    pub memory: Memory,
    pub ticks: u32,
    pub stack: Vec<u8>,
}

impl GameBoyEngine {
    pub fn new(path: &str) -> GameBoyEngine {
        let cartridge = Cartridge::from_path(path).unwrap();
        let engine = GameBoyEngine {
            cpu: CPU::new(),
            renderer: Renderer::new(),
            timer: Timer::new(),
            memory: Memory::new(cartridge),
            paused: false,
            running: true,
            stack: vec![],
            ticks: 0,
        };
        return engine;
    }

    pub fn start(&mut self) {
        // let mut n = 0;
        while self.running {
            print!("{:#04x} ", self.cpu.regs.pc);
            if !self.cpu_step() {
                self.running = false;
            }
            // n += 1;
        }
    }

    pub fn cpu_step(&mut self) -> bool {
        let opcode = self.memory.read(self.cpu.regs.pc as usize);
        let following_byte = self.memory.read((self.cpu.regs.pc + 1) as usize);
        let second_byte = self.memory.read((self.cpu.regs.pc + 2) as usize);
        let inst = Instruction::from_opcode(&opcode);
        println!(
            "{} ({:02X} {:02X} {:02X}) A:{:02X}, BC:{:02X}{:02X}, DE:{:02X}{:02X}, HL:{:02X}{:02X}",
            inst.to_string(),
            opcode,
            following_byte,
            second_byte,
            self.cpu.regs.a,
            self.cpu.regs.b,
            self.cpu.regs.c,
            self.cpu.regs.d,
            self.cpu.regs.e,
            self.cpu.regs.h,
            self.cpu.regs.l,
        );
        self.cpu.current_instruction = inst;
        self.cpu.destination_is_mem = false;
        self.cpu.increment_pointer(1);
        self.fetch_data();
        self.cpu.execute(&mut self.memory);

        return true;
    }

    pub fn fetch_data(&mut self) {
        let pc = self.cpu.regs.pc as usize;
        self.cpu.fetched_data = match self.cpu.current_instruction.address_mode {
            // Instruction::AddressMode::IMPLIED => {}
            AddressMode::IMPLIED => 0,
            AddressMode::R => self
                .cpu
                .get_register_value(&self.cpu.current_instruction.register_1),

            AddressMode::R_R => self
                .cpu
                .get_register_value(&self.cpu.current_instruction.register_2),
            AddressMode::MR_R => {
                self.cpu.mem_dest = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;
                self.cpu.destination_is_mem = true;
                if self.cpu.current_instruction.register_1 == RegisterType::C {
                    self.cpu.mem_dest |= 0xff00;
                }
                self.cpu
                    .get_register_value(&self.cpu.current_instruction.register_2)
            }

            AddressMode::R_D8 => {
                let result = self.memory.read(pc);
                self.cpu.emu_cycles(1);
                self.cpu.increment_pointer(1);
                result as u16
            }

            AddressMode::D16 | AddressMode::R_D16 => {
                let result: u16 = self.memory.read16(self.cpu.regs.pc as usize);
                // result.set_low(self.memory.read(pc));
                // result.set_high(self.memory.read(pc + 1));
                self.cpu.increment_pointer(2);
                result
            }
            AddressMode::R_MR => {
                let mut addr: u16 = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2);
                if self.cpu.current_instruction.register_2 == RegisterType::C {
                    addr |= 0xFF00;
                }
                let res = self.memory.read(addr as usize);
                self.cpu.emu_cycles(1);
                res as u16
            }
            AddressMode::R_HLI => {
                let addr = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2);
                let res = self.memory.read(addr as usize) as u16;
                self.cpu.emu_cycles(1);
                self.cpu.set_register_value(
                    RegisterType::HL,
                    self.cpu.get_register_value(&RegisterType::HL) + 1,
                );
                res
            }
            AddressMode::R_HLD => {
                let addr = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2);
                let res = self.memory.read(addr as usize) as u16;
                self.cpu.emu_cycles(1);
                self.cpu.set_register_value(
                    RegisterType::HL,
                    self.cpu.get_register_value(&RegisterType::HL) - 1,
                );
                res
            }
            AddressMode::HLI_R => {
                self.cpu.mem_dest = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;
                self.cpu.destination_is_mem = true;
                let result = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2);
                self.cpu.set_register_value(
                    RegisterType::HL,
                    self.cpu.get_register_value(&RegisterType::HL) + 1,
                );
                result
            }
            AddressMode::HLD_R => {
                self.cpu.mem_dest = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;
                self.cpu.destination_is_mem = true;
                let result = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2);
                self.cpu.set_register_value(
                    RegisterType::HL,
                    self.cpu.get_register_value(&RegisterType::HL) - 1,
                );
                result
            }
            AddressMode::R_A8 => {
                self.cpu.emu_cycles(1);
                let res = self.memory.read(self.cpu.regs.pc as usize) as u16;
                self.cpu.increment_pointer(1);
                res
            }
            AddressMode::A8_R => {
                self.cpu.mem_dest = self.memory.read(self.cpu.regs.pc as usize) as usize;
                self.cpu.destination_is_mem = true;
                self.cpu.emu_cycles(1);
                self.cpu.increment_pointer(1);
                let addr = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_2)
                    as usize;

                self.memory.read(addr) as u16
            }
            AddressMode::D8 | AddressMode::HL_SPR => {
                let res = self.memory.read(self.cpu.regs.pc as usize) as u16;
                self.cpu.emu_cycles(1);
                self.cpu.increment_pointer(1);
                res
            }
            AddressMode::A16_R | AddressMode::D16_R => {
                let low = self.memory.read(pc);
                let hi = self.memory.read(pc + 1);
                self.cpu.mem_dest = (((hi as u16) << 8) | (low as u16)) as usize;
                self.cpu.destination_is_mem = true;
                self.cpu.increment_pointer(2);
                self.cpu
                    .get_register_value(&self.cpu.current_instruction.register_2)
            }
            AddressMode::MR_D8 => {
                let res = self.memory.read(self.cpu.regs.pc as usize) as u16;
                self.cpu.emu_cycles(1);
                self.cpu.increment_pointer(1);
                self.cpu.mem_dest = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;
                self.cpu.destination_is_mem = true;
                res
            }
            AddressMode::MR => {
                self.cpu.mem_dest = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;
                self.cpu.destination_is_mem = true;
                self.cpu.emu_cycles(1);
                let addr = self
                    .cpu
                    .get_register_value(&self.cpu.current_instruction.register_1)
                    as usize;

                self.memory.read(addr) as u16
            }
            AddressMode::R_A16 => {
                let low = self.memory.read(pc);
                let hi = self.memory.read(pc + 1);

                let addr = ((hi as u16) << 8) | (low as u16);
                self.cpu.increment_pointer(2);
                self.cpu.emu_cycles(3);
                let res = self.memory.read(addr as usize) as u16;
                res
            } // _ => 0,
        }
    }
}
