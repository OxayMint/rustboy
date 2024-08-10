// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     pub static ref TIMER: Mutex<Timer> = Mutex::new(Timer::new());
// }

// pub struct Timer {
//     div: u8,          // FF04 - Divider register
//     tima: u8,         // FF05 - Timer counter
//     tma: u8,          // FF06 - Timer modulo
//     tac: u8,          // FF07 - Timer control
//     div_cycles: u32,  // Internal counter for DIV
//     tima_cycles: u32, // Internal counter for TIMA
// }

// impl Timer {
//     fn new() -> Self {
//         Timer {
//             div: 0,
//             tima: 0,
//             tma: 0,
//             tac: 0,
//             div_cycles: 0,
//             tima_cycles: 0,
//         }
//     }

//     pub fn tick(&mut self, cycles: u32) -> bool {
//         let mut interrupt_requested = false;

//         // Update DIV
//         self.div_cycles += cycles;
//         while self.div_cycles >= 256 {
//             self.div_cycles -= 256;
//             self.div = self.div.wrapping_add(1);
//         }

//         // Update TIMA if enabled
//         if self.tac & 0x04 != 0 {
//             let tima_freq = match self.tac & 0x03 {
//                 0 => 1024,
//                 1 => 16,
//                 2 => 64,
//                 3 => 256,
//                 _ => unreachable!(),
//             };

//             self.tima_cycles += cycles;
//             while self.tima_cycles >= tima_freq {
//                 self.tima_cycles -= tima_freq;
//                 self.tima = self.tima.wrapping_add(1);

//                 if self.tima == 0 {
//                     self.tima = self.tma;
//                     interrupt_requested = true;
//                 }
//             }
//         }

//         interrupt_requested
//     }

//     pub fn read_byte(&self, address: usize) -> u8 {
//         match address {
//             0xFF04 => self.div,
//             0xFF05 => self.tima,
//             0xFF06 => self.tma,
//             0xFF07 => self.tac,
//             _ => panic!("Invalid timer address: {:04X}", address),
//         }
//     }

//     pub fn write_byte(&mut self, address: usize, value: u8) {
//         match address {
//             0xFF04 => {
//                 self.div = 0;
//                 self.div_cycles = 0;
//             }
//             0xFF05 => self.tima = value,
//             0xFF06 => self.tma = value,
//             0xFF07 => self.tac = value & 0x07,
//             _ => panic!("Invalid timer address: {:04X}", address),
//         }
//     }

//     pub fn reset(&mut self) {
//         self.div = 0;
//         self.tima = 0;
//         self.tma = 0;
//         self.tac = 0;
//         self.div_cycles = 0;
//         self.tima_cycles = 0;
//     }
// }

// // Helper functions to easily access the timer
// pub fn tick_timer(cycles: u32) -> bool {
//     TIMER.lock().unwrap().tick(cycles)
// }

// pub fn read_timer_byte(address: usize) -> u8 {
//     TIMER.lock().unwrap().read_byte(address)
// }

// pub fn write_timer_byte(address: usize, value: u8) {
//     TIMER.lock().unwrap().write_byte(address, value)
// }

// pub fn reset_timer() {
//     TIMER.lock().unwrap().reset()
// }
