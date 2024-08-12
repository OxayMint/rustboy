#[path = "emu_debug.rs"]
pub mod EmuDebug;
#[path = "gb/bus.rs"]
pub mod bus;
#[path = "gb/cartridge.rs"]
pub mod cartridge;
#[path = "gb/cpu.rs"]
pub mod cpu;
#[path = "gb/dma.rs"]
pub mod dma;
#[path = "gb/instruction.rs"]
pub mod instruction;
#[path = "gb/interrupts.rs"]
pub mod interrupts;
#[path = "gb/io.rs"]
pub mod io;
#[path = "gb/ppu.rs"]
pub mod ppu;
#[path = "gb/rendering.rs"]
pub mod rendering;
#[path = "gb/timer.rs"]
pub mod timer;

use bus::Bus;
use cartridge::Cartridge;
use cpu::CPU;
use ppu::PPU;
use rendering::Renderer;
use std::sync::atomic::AtomicU32;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;
use EmuDebug::EMU_DEBUG;

pub struct GameBoyEngine {
    pub paused: Arc<AtomicBool>,
    pub running: Arc<AtomicBool>,
    pub ui: Renderer,
    pub ticks: Arc<AtomicU32>,
}

impl GameBoyEngine {
    pub fn new(path: &str) -> GameBoyEngine {
        let cartridge = Cartridge::from_path(path).unwrap();
        Bus::set_cartridge(cartridge);
        GameBoyEngine {
            ui: Renderer::new(),
            paused: Arc::new(AtomicBool::new(false)),
            running: Arc::new(AtomicBool::new(true)),
            ticks: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn start(&mut self) {
        let running = Arc::clone(&self.running);
        let paused = Arc::clone(&self.paused);
        let ticks = Arc::clone(&self.ticks);
        // CPU thread
        let cpu_thread = thread::spawn(move || {
            let mut cpu = CPU::new();
            while running.load(Ordering::Relaxed) {
                if paused.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                // Single CPU step
                let step_result = cpu.cpu_step();

                // if ticks.load(Ordering::Relaxed) % 1000 == 0 {
                //     println!("CPU tick: {}", ticks.load(Ordering::Relaxed));
                // }
                if step_result < 0 {
                    println!("CPU Error: step_result = {}", step_result);
                    running.store(false, Ordering::Relaxed);
                    break;
                }

                ticks.fetch_add(1, Ordering::Relaxed);
                // Yield to other threads
                thread::yield_now();
            }
        });

        _ = self.ui.init(); // Initialize the UI

        while self.running.load(Ordering::Relaxed) {
            self.ui.tick(); // Handle UI updates
            if self.ui.exited {
                self.running.store(false, Ordering::Relaxed);
            }
        }
        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();

        println!("Finished app");
    }
}
