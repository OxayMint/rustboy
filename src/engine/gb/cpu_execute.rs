use crate::libs::gameboy::bus::Bus;

use super::{AddressMode, ConditionType, InstructionType, RegisterType, CPU};

impl CPU {
    fn run_cb(&mut self) {
        // println!("RUNNING CB OP");
        let operation = self.fetched_data;
        let register = RegisterType::decode(operation as usize & 0b111);
        let bit = (operation >> 3) & 0b111;
        let bit_op = (operation >> 6) & 0b11;
        let mut reg_val = self.cpu_read_reg8(register.clone());

        self.emu_cycles(1);

        if register == RegisterType::HL {
            self.emu_cycles(2);
        }
        match bit_op {
            1 => {
                //BIT
                let z: u8 = !(reg_val & (1 << bit) != 0) as u8;
                self.set_flags(z as i8, 0, 1, -1);
                return;
            }
            2 => {
                //RST
                reg_val &= !(1 << bit);
                self.cpu_set_reg8(register, reg_val);
                return;
            }

            3 => {
                //SET
                reg_val |= 1 << bit;
                self.cpu_set_reg8(register, reg_val);
                return;
            }
            _ => {}
        }

        let flag_c = self.flag_c();

        match bit {
            0 => {
                //RLC
                let mut set_c = false;
                let mut result = (reg_val << 1) & 0xFF;

                if (reg_val & (1 << 7)) != 0 {
                    result |= 1;
                    set_c = true;
                }

                self.cpu_set_reg8(register, result);
                self.set_flags((result == 0) as i8, 0, 0, set_c as i8);
                return;
            }

            1 => {
                //RRC
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= old << 7;
                self.cpu_set_reg8(register, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, old as i8 & 1);
                return;
            }

            2 => {
                //RL
                let old = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;
                self.cpu_set_reg8(register, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, (old & 0b10000000 > 0) as i8);
                return;
            }

            3 => {
                //RR
                let old = reg_val;
                reg_val >>= 1;

                reg_val |= (flag_c as u8) << 7;

                self.cpu_set_reg8(register, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, old as i8 & 1);
                return;
            }

            4 => {
                //SLA
                let old = reg_val;
                reg_val <<= 1;

                self.cpu_set_reg8(register, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, (old & 0b10000000 > 0) as i8);
                return;
            }

            5 => {
                //SRA
                let u = reg_val as i8 >> 1;
                self.cpu_set_reg8(register, u as u8);
                self.set_flags((u == 0) as i8, 0, 0, reg_val as i8 & 1);
                return;
            }

            6 => {
                //SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                self.cpu_set_reg8(register, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, 0);
                return;
            }

            7 => {
                //SRL
                let u = reg_val >> 1;
                self.cpu_set_reg8(register, u);
                self.set_flags((u == 0) as i8, 0, 0, ((reg_val & 1) > 0) as i8);
                return;
            }
            _ => {
                panic!("ERROR: INVALID CB: {operation}");
            }
        }
    }

    pub fn execute(&mut self) -> i8 {
        match self.current_instruction.instruction_type {
            InstructionType::NONE => {
                println!("just do nothings")
            } //panic!("Something not right here"),
            InstructionType::NOP => {
                // println!("NOP");
            }
            InstructionType::LD => {
                if self.destination_is_mem {
                    if self.current_instruction.register_2 >= RegisterType::AF {
                        // SP to MAIN_BUS
                        Bus::write8(self.mem_dest, self.fetched_data as u8);
                        self.emu_cycles(1);
                        Bus::write8(self.mem_dest + 1, (self.fetched_data >> 8) as u8);
                    } else {
                        Bus::write(self.mem_dest as usize, self.fetched_data);
                    }
                    self.emu_cycles(1);
                    return 0;
                } else if self.current_instruction.address_mode == AddressMode::HL_SPR {
                    // This is the LD HL,SP+e8 case
                    let sp = self.read_reg(&RegisterType::SP);
                    let e8 = self.fetched_data as i8 as i16; // Convert to signed 8-bit, then extend to 16-bit

                    let result = sp.wrapping_add(e8 as u16);

                    // Calculate flags
                    let h = (sp & 0xF) + (e8 as u16 & 0xF) > 0xF;
                    let c = (sp & 0xFF) + (e8 as u16 & 0xFF) > 0xFF;

                    self.set_reg(RegisterType::HL, result);
                    self.set_flags(0, 0, h as i8, c as i8);
                    return 0;
                }
                self.set_reg(
                    self.current_instruction.register_1.clone(),
                    self.fetched_data,
                );
            }
            InstructionType::INC => {
                let mut new_val = self
                    .read_reg(&self.current_instruction.register_1)
                    .wrapping_add(1);

                if self.current_instruction.register_1 >= RegisterType::AF {
                    self.emu_cycles(1);
                }
                if self.current_instruction.register_1 == RegisterType::HL
                    && self.current_instruction.address_mode == AddressMode::MR
                {
                    let hl_val = self.read_reg(&RegisterType::HL);
                    new_val = Bus::read(hl_val as usize).wrapping_add(1);
                    new_val &= 0xFF;
                    Bus::write(hl_val as usize, new_val);
                } else {
                    self.set_reg(self.current_instruction.register_1.clone(), new_val);
                    new_val = self.read_reg(&self.current_instruction.register_1);
                }
                if (self.current_instruction.opcode & 0x3) != 0x3 {
                    self.set_flags((new_val == 0) as i8, 0, ((new_val & 0xf) == 0) as i8, -1)
                }
                // Z  Set if result is 0.
                // N  0
                // H Set if overflow from bit 3.
            }
            InstructionType::DEC => {
                let new_val = self.fetched_data.wrapping_sub(1);
                if self.current_instruction.register_1 >= RegisterType::AF {
                    self.emu_cycles(1);
                }
                if self.current_instruction.register_1 == RegisterType::SP {
                    self.regs.sp = new_val
                } else {
                    if self.destination_is_mem {
                        Bus::write(self.mem_dest, new_val);
                    } else {
                        self.set_reg(self.current_instruction.register_1.clone(), new_val);
                    }
                    if (self.current_instruction.opcode & 0xb) != 0xb {
                        self.set_flags(
                            (new_val == 0) as i8,
                            1,
                            ((self.fetched_data & 0xf) == 0x0) as i8,
                            -1,
                        )
                    }
                }
            }
            InstructionType::RLCA => {
                let mut val: u8 = self.regs.a;
                let c = (val >> 7) & 1; // = 1 or 0. Basically a bool thats says whether the last bit of A is 1
                val = (val << 1) | c;
                self.regs.a = val;
                self.set_flags(0, 0, 0, c as i8);
            }
            InstructionType::ADD => {
                let reg_val: u32 = self.read_reg(&self.current_instruction.register_1) as u32;
                let mut val: u32;
                let is_16_bit = self.current_instruction.register_1 >= RegisterType::AF;
                let is_sp = self.current_instruction.register_1 == RegisterType::SP;

                if is_16_bit {
                    self.emu_cycles(1);
                }

                let mut z: i8 = -1;
                let mut h: i8 = 0;
                let mut c: i8 = 0;

                if is_sp && self.fetched_data <= 0xFF {
                    // This is the ADD SP,e8 case
                    let e8 = (self.fetched_data as i8) as i32;
                    val = (reg_val as i32).wrapping_add(e8) as u32;

                    // Set flags for ADD SP,e8
                    z = 0;
                    h = (((reg_val & 0xF) as i32 + (e8 & 0xF)) > 0xF) as i8;
                    c = (((reg_val & 0xFF) as i32 + (e8 & 0xFF)) > 0xFF) as i8;

                    self.emu_cycles(2); // Additional 2 cycles for ADD SP,e8
                } else if is_16_bit {
                    val = reg_val.wrapping_add(self.fetched_data as u32);

                    // Set flags for 16-bit ADD
                    h = ((reg_val & 0xFFF) + (self.fetched_data & 0xFFF) as u32 >= 0x1000) as i8;
                    c = (val >= 0x10000) as i8;
                } else {
                    // 8-bit ADD
                    val = reg_val + self.fetched_data as u32;

                    z = ((val & 0xFF) == 0) as i8;
                    h = ((reg_val & 0xF) + (self.fetched_data & 0xF) as u32 >= 0x10) as i8;
                    c = ((reg_val & 0xFF).wrapping_add((self.fetched_data & 0xFF) as u32) >= 0x100)
                        as i8;
                }

                self.set_reg(
                    self.current_instruction.register_1.clone(),
                    val as u16 & 0xFFFF,
                );
                self.set_flags(z, 0, h, c);
            }
            // InstructionType::ADD => {
            //     let reg_val: u32 = self.read_reg(&self.current_instruction.register_1) as u32;
            //     let mut val: u32 = reg_val + self.fetched_data as u32;
            //     let is_16_bit = self.current_instruction.register_1 >= RegisterType::AF;
            //     if is_16_bit {
            //         self.emu_cycles(1);
            //         if self.current_instruction.register_1 == RegisterType::SP {
            //             val = reg_val.wrapping_add(self.fetched_data as u32) as u32;
            //         }
            //     }
            //     let mut z: i8 = ((val & 0xff) == 0) as i8;
            //     let mut h: i8 = ((reg_val & 0xf) + (self.fetched_data & 0xf) as u32 >= 0x10) as i8;
            //     let mut c: i8 = ((reg_val & 0xff).wrapping_add((self.fetched_data & 0xff) as u32)
            //         >= 0x100) as i8;
            //     if is_16_bit {
            //         z = -1;
            //         h = ((reg_val & 0xfff) + (self.fetched_data & 0xfff) as u32 >= 0x1000) as i8;
            //         c = (val >= 0x10000) as i8;
            //     }
            //     if self.current_instruction.register_1 == RegisterType::SP {
            //         z = 0;
            //         h = ((reg_val & 0xf) as i8 + (self.fetched_data & 0xf) as i8 >= 0x10) as i8;
            //         c = ((reg_val & 0xff) as i16 + (self.fetched_data & 0xff) as i16 >= 0x100)
            //             as i8;
            //     }

            //     self.set_reg(
            //         self.current_instruction.register_1.clone(),
            //         val as u16 & 0xFFFF,
            //     );
            //     self.set_flags(z, 0, h, c);
            // }
            InstructionType::RRCA => {
                let mut val: u8 = self.regs.a;
                let c = val & 1; // = 1 or 0. Basically a bool thats says whether the first bit of A is 1
                val = (val >> 1) | (c << 7);
                self.regs.a = val;
                self.set_flags(0, 0, 0, c as i8);
            }
            InstructionType::STOP => {
                // println!("Stop executed");
                // exit(1);
            }
            InstructionType::RLA => {
                let old_c = self.flag_c() as u8;
                let new_c = self.regs.a >> 7;
                self.regs.a = (self.regs.a << 1) | old_c;
                self.set_flags(0, 0, 0, new_c as i8);
            }
            InstructionType::JR => {
                let offset = (self.fetched_data & 0xFF) as i8;

                // Calculate the new address
                let new_address = self.regs.pc.wrapping_add(offset as u16);

                // Call goto_addr
                self.goto_addr(new_address, false);

                // Add extra cycles if the condition is true
                if self.check_condition() {
                    self.emu_cycles(1); // Add 1 more cycle for a total of 3
                }
            }
            InstructionType::RRA => {
                let old_c = self.flag_c() as u8;
                let new_c = self.regs.a & 1;
                self.regs.a = (self.regs.a >> 1) | (old_c << 7);
                self.set_flags(0, 0, 0, new_c as i8);
            }
            InstructionType::DAA => {
                let mut adjust = 0;
                let mut carry = false;
                let mut a = self.regs.a;

                if self.flag_h() || (!self.flag_n() && (a & 0xf) > 9) {
                    adjust |= 0x06;
                }

                if self.flag_c() || (!self.flag_n() && a > 0x99) {
                    adjust |= 0x60;
                    carry = true;
                }

                a = if self.flag_n() {
                    a.wrapping_sub(adjust)
                } else {
                    a.wrapping_add(adjust)
                };

                self.regs.a = a;

                self.set_flags((a == 0) as i8, -1, 0, carry as i8);
            }
            InstructionType::CPL => {
                self.regs.a = !self.regs.a;
                self.set_flags(-1, 1, 1, -1)
            }
            InstructionType::SCF => self.set_flags(-1, 0, 0, 1),
            InstructionType::CCF => self.set_flags(-1, 0, 0, !self.flag_c() as i8),
            InstructionType::HALT => self.halted = true,
            InstructionType::ADC => {
                let d = self.fetched_data;
                let a = self.regs.a as u16;
                let c = self.flag_c() as u16;

                self.regs.a = ((d + a + c) & 0xff) as u8;

                self.set_flags(
                    (self.regs.a == 0) as i8,
                    0,
                    (((a & 0xf) + (d & 0xf) + c) > 0xf) as i8,
                    ((a + d + c) > 0xff) as i8,
                )
            }
            InstructionType::SUB => {
                let left = self.read_reg(&self.current_instruction.register_1) as u8; // u8
                let right = self.fetched_data as u8; // u8

                let result = left.wrapping_sub(right);

                self.set_reg(self.current_instruction.register_1.clone(), result as u16);
                self.set_flags(
                    (result == 0) as i8,
                    1,
                    ((left & 0xF) < (right & 0xF)) as i8,
                    (left < right) as i8,
                );
            }
            InstructionType::SBC => {
                let a = self.regs.a;
                let r8 = self.fetched_data as u8;
                let carry = self.flag_c() as u8;

                let (result, carry1) = a.overflowing_sub(r8);
                let (result, carry2) = result.overflowing_sub(carry);

                let h = (a & 0xF) < ((r8 & 0xF) + carry);
                let c = carry1 || carry2;

                self.regs.a = result;
                self.set_flags((result == 0) as i8, 1, h as i8, c as i8);
            }
            InstructionType::AND => {
                let res = self.regs.a as u16 & self.fetched_data;
                self.cpu_set_reg8(RegisterType::A, res as u8);
                self.set_flags((res == 0) as i8, 0, 1, 0)
            }
            InstructionType::XOR => {
                self.regs.a ^= self.fetched_data as u8;
                self.set_flags((self.regs.a == 0) as i8, 0, 0, 0)
            }
            InstructionType::OR => {
                self.regs.a |= self.fetched_data as u8;
                self.set_flags((self.regs.a == 0) as i8, 0, 0, 0)
            }
            InstructionType::CP => {
                let a = self.regs.a as i32;
                let c = self.fetched_data as i32;
                let n = a.wrapping_sub(c);
                self.set_flags(
                    (n == 0) as i8,
                    1,
                    ((self.regs.a as i32 & 0x0F).wrapping_sub(self.fetched_data as i32 & 0x0F) < 0)
                        as i8,
                    (n < 0) as i8,
                );
                // self.set_flags(
                //     (a == c) as i8,
                //     1,
                //     ((a & 0xF) < (c & 0xF)) as i8,
                //     (a > c) as i8,
                // );
            }
            InstructionType::POP => {
                let low = Bus::stack_pop8(&mut self.regs.sp);
                self.emu_cycles(1);
                let hi = Bus::stack_pop8(&mut self.regs.sp);
                self.emu_cycles(1);
                let reg = self.current_instruction.register_1.clone();
                self.set_reg(reg, ((hi as u16) << 8) | low as u16);
                // }
                if self.current_instruction.register_1 == RegisterType::AF {
                    self.set_flags(
                        ((low >> 7) & 1 > 0) as i8,
                        ((low >> 6) & 1 > 0) as i8,
                        ((low >> 5) & 1 > 0) as i8,
                        ((low >> 4) & 1 > 0) as i8,
                    );
                }
            }
            InstructionType::JP => self.goto_addr(self.fetched_data, false),
            InstructionType::PUSH => {
                let hi = (self.fetched_data >> 8) as u8;
                self.emu_cycles(1);
                Bus::stack_push8(&mut self.regs.sp, hi);

                let low = (self.fetched_data) as u8;
                self.emu_cycles(1);
                Bus::stack_push8(&mut self.regs.sp, low);

                self.emu_cycles(1);
            }
            InstructionType::RET => {
                if !self.check_condition() {
                    self.emu_cycles(1);
                }
                if self.check_condition() {
                    let low = Bus::stack_pop8(&mut self.regs.sp);
                    self.emu_cycles(1);
                    let hi = Bus::stack_pop8(&mut self.regs.sp);
                    self.emu_cycles(1);
                    let addr = ((hi as u16) << 8) | low as u16;
                    self.regs.pc = addr;
                    self.emu_cycles(1);
                }
            }
            InstructionType::CB => self.run_cb(),
            InstructionType::CALL => self.goto_addr(self.fetched_data, true),
            InstructionType::RETI => {
                self.int_master_enabled = true;
                if !self.check_condition() {
                    self.emu_cycles(1);
                }
                if self.check_condition() {
                    let low = Bus::stack_pop8(&mut self.regs.sp);
                    self.emu_cycles(1);
                    let hi = Bus::stack_pop8(&mut self.regs.sp);
                    self.emu_cycles(1);
                    let addr = ((hi as u16) << 8) | low as u16;
                    self.regs.pc = addr;
                    self.emu_cycles(1);
                }
            }
            InstructionType::LDH => {
                if self.current_instruction.register_1 == RegisterType::A {
                    let address = (0xFF00 | self.fetched_data) as usize;
                    let val = Bus::read8(address);
                    self.cpu_set_reg8(self.current_instruction.register_1.clone(), val);
                } else {
                    Bus::write8(self.mem_dest, self.regs.a);
                }
                self.emu_cycles(1);
            }
            // InstructionType::JPHL => todo!(),
            InstructionType::DI => {
                self.int_master_enabled = false;
            }
            InstructionType::EI => self.ime_enabling = true,
            InstructionType::RST => {
                self.goto_addr(self.current_instruction.rst_vec as u16, true);
            }
            InstructionType::ERR => todo!(),
            // InstructionType::RLC => todo!(),
            // InstructionType::RRC => todo!(),
            // InstructionType::RL => todo!(),
            // InstructionType::RR => todo!(),
            // InstructionType::SLA => todo!(),
            // InstructionType::SRA => todo!(),
            // InstructionType::SWAP => todo!(),
            // InstructionType::SRL => todo!(),
            // InstructionType::BIT => todo!(),
            // InstructionType::RES => todo!(),
            // InstructionType::SET => todo!(),
        }
        return 0;
    }
    fn check_condition(&self) -> bool {
        // println!("curr condition: {}", self.current_instruction.condition);
        match self.current_instruction.condition {
            ConditionType::NONE => return true,
            ConditionType::NZ => return !self.flag_z(),
            ConditionType::Z => return self.flag_z(),
            ConditionType::NC => return !self.flag_c(),
            ConditionType::C => return self.flag_c(),
        }
    }

    fn goto_addr(&mut self, addr: u16, push_pc: bool) {
        if self.check_condition() {
            if push_pc {
                self.emu_cycles(2);
                Bus::stack_push16(&mut self.regs.sp, self.regs.pc);
            }
            self.regs.pc = addr;
            self.emu_cycles(1);
        }
    }
    fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z != -1 {
            CPU::bit_set(&mut self.regs.f, 7, z > 0);
        }

        if n != -1 {
            CPU::bit_set(&mut self.regs.f, 6, n > 0);
        }

        if h != -1 {
            CPU::bit_set(&mut self.regs.f, 5, h > 0);
        }

        if c != -1 {
            CPU::bit_set(&mut self.regs.f, 4, c > 0);
        }
    }
    fn flag_z(&self) -> bool {
        (self.regs.f & 0b10000000) != 0
    }
    fn flag_n(&self) -> bool {
        (self.regs.f & 0b01000000) != 0
    }
    fn flag_h(&self) -> bool {
        (self.regs.f & 0b00100000) != 0
    }
    fn flag_c(&self) -> bool {
        (self.regs.f & 0b00010000) != 0
    }
    pub fn read_reg(&self, register: &RegisterType) -> u16 {
        match register {
            RegisterType::A => self.regs.a as u16,
            RegisterType::B => self.regs.b as u16,
            RegisterType::C => self.regs.c as u16,
            RegisterType::D => self.regs.d as u16,
            RegisterType::E => self.regs.e as u16,
            RegisterType::H => self.regs.h as u16,
            RegisterType::L => self.regs.l as u16,
            RegisterType::F => self.regs.f as u16,
            RegisterType::AF => u16::from_be_bytes([self.regs.a, self.regs.f]),
            RegisterType::BC => u16::from_be_bytes([self.regs.b, self.regs.c]),
            RegisterType::DE => u16::from_be_bytes([self.regs.d, self.regs.e]),
            RegisterType::HL => u16::from_be_bytes([self.regs.h, self.regs.l]),
            RegisterType::SP => self.regs.sp,
            RegisterType::PC => self.regs.pc,
            RegisterType::N => panic!("Non existent register occurred"),
        }
    }
    pub fn set_reg(&mut self, register: RegisterType, val: u16) {
        match register {
            RegisterType::A => self.regs.a = val as u8,
            RegisterType::B => self.regs.b = val as u8,
            RegisterType::C => self.regs.c = val as u8,
            RegisterType::D => self.regs.d = val as u8,
            RegisterType::E => self.regs.e = val as u8,
            RegisterType::H => self.regs.h = val as u8,
            RegisterType::L => self.regs.l = val as u8,
            RegisterType::F => self.regs.f = val as u8,
            RegisterType::AF => {
                self.regs.a = (val >> 8) as u8;
                self.regs.f = val as u8 & 0xF0; // Only upper 4 bits are used in F
            }
            RegisterType::BC => {
                self.regs.b = (val >> 8) as u8;
                self.regs.c = val as u8;
            }
            RegisterType::DE => {
                self.regs.d = (val >> 8) as u8;
                self.regs.e = val as u8;
            }
            RegisterType::HL => {
                self.regs.h = (val >> 8) as u8;
                self.regs.l = val as u8;
            }
            RegisterType::SP => self.regs.sp = val,
            RegisterType::PC => self.regs.pc = val,
            RegisterType::N => panic!("Attempt to set non-existent register"),
        }
    }
    pub fn cpu_read_reg8(&self, rt: RegisterType) -> u8 {
        match rt {
            RegisterType::A => self.regs.a,
            RegisterType::B => self.regs.b,
            RegisterType::C => self.regs.c,
            RegisterType::D => self.regs.d,
            RegisterType::E => self.regs.e,
            RegisterType::H => self.regs.h,
            RegisterType::L => self.regs.l,
            RegisterType::F => self.regs.f,
            RegisterType::HL => {
                let address = self.read_reg(&RegisterType::HL);
                Bus::read8(address as usize)
            }
            _ => panic!("Invalid 8-bit register type: {:?}", rt),
        }
    }
    pub fn cpu_set_reg8(&mut self, rt: RegisterType, val: u8) {
        match rt {
            RegisterType::A => self.regs.a = val,
            RegisterType::B => self.regs.b = val,
            RegisterType::C => self.regs.c = val,
            RegisterType::D => self.regs.d = val,
            RegisterType::E => self.regs.e = val,
            RegisterType::H => self.regs.h = val,
            RegisterType::L => self.regs.l = val,
            RegisterType::F => self.regs.f = val & 0xF0, // Ensure lower 4 bits are always 0
            RegisterType::HL => Bus::write8(self.read_reg(&RegisterType::HL) as usize, val),
            _ => panic!("Invalid 8-bit register type: {:?}", rt),
        }
    }
}
