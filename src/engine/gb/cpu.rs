use crate::libs::instruction::{self, *};

use super::{memory::Memory, SetBytes};
pub struct CPU {
    pub regs: Registers,
    pub fetched_data: u16,
    pub mem_dest: usize,
    pub destination_is_mem: bool,
    pub cur_opcode: u8,
    pub halted: bool,
    pub stepping: bool,
    pub current_instruction: Instruction,
    pub int_master_enabled: bool,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0x100,
            sp: 0,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        // Instruction::start_opcodetests();
        CPU {
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            destination_is_mem: false,
            cur_opcode: 0,
            halted: false,
            stepping: true,
            int_master_enabled: false,
            current_instruction: Instruction {
                ..Default::default()
            },
        }
    }

    // pub fn step(&mut self) -> bool {
    //     if !self.halted {
    //         self.execute();
    //         return true;
    //     }
    //     return false;
    // }

    pub fn emu_cycles(&mut self, cycles: u8) {}
    pub fn execute(&mut self, memory: &mut Memory) {
        match self.current_instruction.instruction_type {
            InstructionType::NONE => panic!("Something not right here"),
            InstructionType::NOP => {}
            InstructionType::LD => {
                if self.destination_is_mem {
                    if self.current_instruction.register_2 >= RegisterType::AF {
                        self.emu_cycles(1);
                        memory.write16(self.mem_dest as usize, self.fetched_data);
                    } else {
                        memory.write(self.mem_dest as usize, self.fetched_data.separate_bytes().0);
                    }
                    return;
                }
                if self.current_instruction.address_mode == AddressMode::HL_SPR {
                    let hl = self.get_register_value(&self.current_instruction.register_2);

                    let hflag: bool = (hl & 0xf) + (self.fetched_data & 0xf) >= 0x10;
                    let cflag: bool = (hl & 0xff) + (self.fetched_data & 0xff) >= 0x100;

                    self.set_flags(0, 0, hflag as i8, cflag as i8);
                    self.set_register_value(
                        self.current_instruction.register_1.clone(),
                        self.get_register_value(&self.current_instruction.register_2)
                            .wrapping_add(self.fetched_data as i8 as u16),
                    );
                    return;
                }
                // println!(
                //     "LD to Flag {} value {:#x}",
                //     instruction::Instruction::register_mnemonic(
                //         &self.current_instruction.register_1
                //     ),
                //     self.fetched_data
                // );
                self.set_register_value(
                    self.current_instruction.register_1.clone(),
                    self.fetched_data,
                );
            }
            InstructionType::INC => {
                let new_val = self.fetched_data + 1;
                if self.current_instruction.register_1 == RegisterType::SP {
                    self.regs.sp = new_val
                } else {
                    if self.destination_is_mem {
                        memory.write(self.mem_dest, new_val as u8)
                    } else {
                        self.set_register_value(
                            self.current_instruction.register_1.clone(),
                            new_val,
                        )
                    }
                    if (self.current_instruction.opcode | 0x3) != 0x3 {
                        self.set_flags(
                            (new_val == 0) as i8,
                            0,
                            ((self.fetched_data & 0xf) == 0xf) as i8,
                            -1,
                        )
                    }
                }

                // Z  Set if result is 0.
                // N  0
                // H Set if overflow from bit 3.
            }
            InstructionType::DEC => {
                let new_val = self.fetched_data - 1;
                if self.current_instruction.register_1 == RegisterType::SP {
                    self.regs.sp = new_val
                } else {
                    if self.destination_is_mem {
                        memory.write(self.mem_dest, new_val as u8)
                    } else {
                        self.set_register_value(
                            self.current_instruction.register_1.clone(),
                            new_val,
                        )
                    }
                    if (self.current_instruction.opcode | 0x3) != 0x3 {
                        self.set_flags(
                            (new_val == 0) as i8,
                            1,
                            ((self.fetched_data & 0xf) == 0x0) as i8,
                            -1,
                        )
                    }
                }
            }
            InstructionType::RLCA => todo!(),
            InstructionType::ADD => todo!(),
            InstructionType::RRCA => todo!(),
            InstructionType::STOP => todo!(),
            InstructionType::RLA => todo!(),
            InstructionType::JR => {
                let offset = self.fetched_data as i16;
                let new_address = (self.regs.pc as i16 + offset) as u16;
                self.goto_addr(new_address, memory, false);
            }
            InstructionType::RRA => todo!(),
            InstructionType::DAA => todo!(),
            InstructionType::CPL => {
                let a = self.regs.a as u16;
                let c = self.fetched_data;
                self.set_flags(
                    (a == c) as i8,
                    1,
                    ((a & 0xF) < (c & 0xF)) as i8,
                    (a > c) as i8,
                );
            }
            InstructionType::SCF => todo!(),
            InstructionType::CCF => todo!(),
            InstructionType::HALT => todo!(),
            InstructionType::ADC => todo!(),
            InstructionType::SUB => todo!(),
            InstructionType::SBC => todo!(),
            InstructionType::AND => todo!(),
            InstructionType::XOR => {
                self.regs.a = self.regs.a ^ (self.fetched_data as u8 & 0xff);
                self.set_flags((self.regs.a == 1) as i8, 0, 0, 0);
            }
            InstructionType::OR => todo!(),
            InstructionType::CP => todo!(),
            InstructionType::POP => {
                let val = memory.stack_pop16(&mut self.regs.sp);
                self.emu_cycles(2);
                if self.current_instruction.register_1 == RegisterType::AF {
                    self.set_register_value(RegisterType::AF, val & 0xFFF0);
                } else {
                    self.set_register_value(self.current_instruction.register_1.clone(), val);
                }
            }
            InstructionType::JP => self.goto_addr(self.fetched_data, memory, false),
            InstructionType::PUSH => {
                memory.stack_push16(&mut self.regs.sp, self.fetched_data);
                self.emu_cycles(2);
            }
            InstructionType::RET => {
                if self.current_instruction.condition != ConditionType::NONE {
                    let addr = memory.stack_pop16(&mut self.regs.sp);
                    self.regs.pc = addr;
                    self.emu_cycles(3)
                }
            }
            InstructionType::CB => todo!(),
            InstructionType::CALL => self.goto_addr(self.fetched_data, memory, true),
            InstructionType::RETI => {
                self.int_master_enabled = true;
                if self.current_instruction.condition != ConditionType::NONE {
                    let addr = memory.stack_pop16(&mut self.regs.sp);
                    self.regs.pc = addr;
                    self.emu_cycles(3)
                }
            }
            InstructionType::LDH => {
                self.mem_dest = self.mem_dest.wrapping_add(0xFF00);
                //Load value in register A from the byte at address n16, provided the address is between $FF00 and $FFFF.
                if self.destination_is_mem {
                    println!(
                        "writing to address {:#x} value {:#x}",
                        self.mem_dest, self.fetched_data
                    );
                    memory.write(self.mem_dest, self.fetched_data as u8);
                    self.emu_cycles(1);
                } else {
                    self.regs.a = self.fetched_data as u8;
                }
            }
            InstructionType::JPHL => todo!(),
            InstructionType::DI => {
                self.int_master_enabled = false;
            }
            InstructionType::EI => todo!(),
            InstructionType::RST => {
                self.goto_addr(self.current_instruction.rst_vec as u16, memory, true);
            }
            InstructionType::ERR => todo!(),
            InstructionType::RLC => todo!(),
            InstructionType::RRC => todo!(),
            InstructionType::RL => todo!(),
            InstructionType::RR => todo!(),
            InstructionType::SLA => todo!(),
            InstructionType::SRA => todo!(),
            InstructionType::SWAP => todo!(),
            InstructionType::SRL => todo!(),
            InstructionType::BIT => todo!(),
            InstructionType::RES => todo!(),
            InstructionType::SET => todo!(),
        }
        // self.emu_cycles(if self.check_condition() {
        //     self.current_instruction.cycles
        // } else {
        //     self.current_instruction.no_action_cycles
        // });
    }

    pub fn increment_pointer(&mut self, by: u16) {
        self.regs.pc = self.regs.pc.wrapping_add(by);
    }

    // #define BIT(a, n) ((a & (1 << n)) ? 1 : 0)

    // #define BIT_SET(a, n, on) {if (on) (a) |= (1 << n); else (a) &= ~(1 << n);}

    // #define BETWEEN(a, b, c) ((a >= b) && (a <= c))
    fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        // println!("z:{} n:{} h:{} c:{}", z, n, h, c);
        if z != -1 {
            self.regs.f = if z == 1 {
                self.regs.f | (1 << 7)
            } else {
                self.regs.f & !(1 << 7)
            };
        }

        if n != -1 {
            self.regs.f = if n == 1 {
                self.regs.f | (1 << 6)
            } else {
                self.regs.f & !(1 << 6)
            };
        }

        if h != -1 {
            self.regs.f = if h == 1 {
                self.regs.f | (1 << 5)
            } else {
                self.regs.f & !(1 << 5)
            };
        }

        if c != -1 {
            self.regs.f = if c == 1 {
                self.regs.f | (1 << 4)
            } else {
                self.regs.f & !(1 << 4)
            };
        }
    }
    pub fn get_register_value(&self, register: &RegisterType) -> u16 {
        match register {
            RegisterType::A => self.regs.a as u16,
            RegisterType::B => self.regs.b as u16,
            RegisterType::C => self.regs.c as u16,
            RegisterType::D => self.regs.d as u16,
            RegisterType::E => self.regs.e as u16,
            RegisterType::H => self.regs.h as u16,
            RegisterType::L => self.regs.l as u16,
            RegisterType::F => self.regs.f as u16,
            RegisterType::AF => ((self.regs.a as u16) << 8) | self.regs.f as u16,
            RegisterType::BC => ((self.regs.b as u16) << 8) | self.regs.c as u16,
            RegisterType::DE => ((self.regs.d as u16) << 8) | self.regs.e as u16,
            RegisterType::HL => ((self.regs.h as u16) << 8) | self.regs.l as u16,
            RegisterType::SP => self.regs.sp.clone(),
            RegisterType::PC => self.regs.pc.clone(),
            RegisterType::NONE => todo!("Non existent register occured"),
        }
    }

    pub fn set_register_value(&mut self, register: RegisterType, val: u16) {
        let (low_byte, high_byte) = val.clone().separate_bytes();
        match register {
            RegisterType::A => self.regs.a = low_byte,
            RegisterType::B => self.regs.b = low_byte,
            RegisterType::C => self.regs.c = low_byte,
            RegisterType::D => self.regs.d = low_byte,
            RegisterType::E => self.regs.e = low_byte,
            RegisterType::H => self.regs.h = low_byte,
            RegisterType::L => self.regs.l = low_byte,
            RegisterType::F => self.regs.f = low_byte,
            RegisterType::AF => {
                self.regs.a = high_byte;
                self.regs.f = low_byte;
            }
            RegisterType::BC => {
                self.regs.b = high_byte;
                self.regs.c = low_byte;
            }
            RegisterType::DE => {
                self.regs.d = high_byte;
                self.regs.e = low_byte;
            }
            RegisterType::HL => {
                self.regs.h = high_byte;
                self.regs.l = low_byte;
            }
            // RegisterType::NONE => todo!(),
            _ => {}
        }
    }
    fn check_condition(&self) -> bool {
        let z: bool = (self.regs.f & 0b01000000) != 0;
        let c: bool = (self.regs.f & 0b00001000) != 0;
        match self.current_instruction.condition {
            ConditionType::NONE => return true,
            ConditionType::NZ => return !z,
            ConditionType::Z => return z,
            ConditionType::NC => return !c,
            ConditionType::C => return c,
        }
    }
    fn goto_addr(&mut self, addr: u16, memory: &mut Memory, push_pc: bool) {
        if self.check_condition() {
            if push_pc {
                memory.stack_push16(&mut self.regs.sp, self.regs.pc);
                self.emu_cycles(2);
            }
            self.regs.pc = addr;
            self.emu_cycles(1);
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8, //flags here
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}
