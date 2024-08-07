// use crate::libs::gameboy::*;
// /*
//   Emulator components:

//   |Cart|
//   |CPU|
//   |Address Bus|
//   |PPU|
//   |Timer|

// */
// pub struct Emulator {
//     pub gb_engine: GameBoyEngine,
// }

// impl Emulator {
//     pub fn new(cart_path: &str) -> Emulator {
//         Emulator {
//             gb_engine: GameBoyEngine::new(),
//         }
//     }
//     pub fn start(&mut self, path: &str) {
//         self.gb_engine.start(path);
//     }
//     pub fn sleep(&self, ticks: i16) {
//         println!("Sleep {}", ticks);
//     }
// }
