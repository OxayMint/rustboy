use crate::libs::gameboy::*;
/*
  Emulator components:

  |Cart|
  |CPU|
  |Address Bus|
  |PPU|
  |Timer|

*/

pub struct Emulator {
    pub gb_engine: GameBoyEngine,
}

impl Emulator {
    pub fn new(path: &str) -> Emulator {
        Emulator {
            gb_engine: GameBoyEngine::new(path),
        }
    }
    pub fn start(&mut self) {
        self.gb_engine.start();
    }
    pub fn sleep(&self, ticks: i16) {
        println!("Sleep {}", ticks);
    }
}
