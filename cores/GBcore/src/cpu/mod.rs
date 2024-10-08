#[path = "cpu_execute.rs"]
pub mod execute;
#[path = "cpu_fetch.rs"]
pub mod fetch;
#[path = "cpu_interrupts.rs"]
pub mod interrupts;
use super::instruction::*;
use super::Bus;
pub struct CPU {
    pub regs: Registers,
    pub fetched_data: u16,
    pub mem_dest: usize,
    pub destination_is_mem: bool,
    pub halted: bool,
    pub current_instruction: Instruction,
    pub int_master_enabled: bool,
    pub ime_enabling: bool,
    pub bus: Bus,
    af_count: u32,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x100,
        }
    }
    pub fn get_flags_mnemonic(&self) -> String {
        let z = if self.f & 0b10000000 > 0 { "Z" } else { "-" };
        let n = if self.f & 0b01000000 > 0 { "N" } else { "-" };
        let h = if self.f & 0b00100000 > 0 { "H" } else { "-" };
        let c = if self.f & 0b00010000 > 0 { "C" } else { "-" };
        return format!("{z}{n}{h}{c}");
    }
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            destination_is_mem: false,
            halted: false,
            int_master_enabled: false,
            ime_enabling: false,
            af_count: 0,
            bus: bus,
            current_instruction: Instruction {
                ..Default::default()
            },
        }
    }

    pub fn cpu_step(&mut self) -> i8 {
        // println!("cpu step");
        let mut res = 0i8;
        if !self.halted {
            let opcode: u8 = self.bus.read8(self.regs.pc as usize);
            // let pc = self.regs.pc.clone();
            self.current_instruction = Instruction::from_opcode(&opcode);
            self.increment_pointer(1);
            self.destination_is_mem = false;

            // let following_byte = self.bus.read8((self.regs.pc) as usize);
            // let third_byte = self.bus.read8((self.regs.pc + 1) as usize);
            // res = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
            //     self.regs.a,
            //     self.regs.f,
            //     self.regs.b,
            //     self.regs.c,
            //     self.regs.d,
            //     self.regs.e,
            //     self.regs.h,
            //     self.regs.l,
            //     self.regs.sp,
            //     pc,
            //     self.bus.read(pc as usize),
            //     self.bus.read(pc as usize + 1),
            //     self.bus.read(pc as usize + 2),
            //     self.bus.read(pc as usize + 3),
            // );
            // println!(
            //     "{:04X} {} ({:02X} {:02X} {:02X}) A:{:02X} F:{} BC:{:02X}{:02X} DE:{:02X}{:02X} HL:{:02X}{:02X} SP: {:04X}",
            //     pc,
            //     self.current_instruction.to_string(),
            //     opcode,
            //     following_byte,
            //     third_byte,
            //     self.regs.a,
            //     self.regs.get_flags_mnemonic(),
            //     self.regs.b,
            //     self.regs.c,
            //     self.regs.d,
            //     self.regs.e,
            //     self.regs.h,
            //     self.regs.l,
            //     self.regs.sp
            // );

            self.emu_cycles(1);
            self.fetch_data();
            // self.emu_dbg.update();

            res = self.execute();
        } else {
            // println!("halted");
            self.emu_cycles(1);
            if self.bus.ioram.borrow().interrupt_flags > 0 {
                self.halted = false;
            } else {
                if self.af_count > 10 {
                    // exit(0);
                }
                self.af_count += 1;
            }
        }
        if self.int_master_enabled {
            let flags = self.bus.ioram.borrow().interrupt_flags;
            self.bus.ioram.borrow_mut().interrupt_flags = self.handle_interrupts(flags);
            self.ime_enabling = false;
        }
        if self.ime_enabling {
            self.int_master_enabled = true;
        }
        return res;
    }
    pub fn emu_cycles(&mut self, cycles: u32) {
        for _ in 0..cycles {
            for _ in 0..4 {
                // ctx.ticks++;
                // println!("call tick from CPU");

                self.bus.timer.tick();
                self.bus.ppu.tick();
            }
            self.bus.dma_tick();
        }
    }

    // pub fn request_interrupt(&mut self, int_type: InterruptType) {
    //     self.bus.ioram.interrupt_flags |= int_type;
    // }
    pub fn increment_pointer(&mut self, by: u16) {
        self.regs.pc += by;
    }

    fn bit_set(a: &mut u8, n: u8, on: bool) {
        if on {
            *a |= 1 << n;
        } else {
            *a &= !(1 << n);
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
