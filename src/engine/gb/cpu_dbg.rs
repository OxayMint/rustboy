// let following_byte = Bus::read8((self.regs.pc) as usize);
// let third_byte = Bus::read8((self.regs.pc + 1) as usize);
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
//     Bus::read(pc as usize),
//     Bus::read(pc as usize + 1),
//     Bus::read(pc as usize + 2),
//     Bus::read(pc as usize + 3),
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
