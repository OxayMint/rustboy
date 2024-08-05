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

    // pub fn cpu_step(&self) -> bool {
    //     return self.gb_engine.cpu.step();
    // }

    // pub fn run_loop(&mut self) {
    //     while self.gb_engine.running {
    //         if self.gb_engine.paused {
    //             self.sleep(10);
    //             continue;
    //         }
    //         if !self.cpu_step() {
    //             println!("CPU Stopped");
    //             std::process::exit(-1);
    //         }
    //         self.gb_engine.ticks += 1;
    //     }
    // }
}
