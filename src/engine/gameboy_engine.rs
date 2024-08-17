#[path = "gb/bus.rs"]
pub mod bus;
#[path = "gb/cartridge.rs"]
pub mod cartridge;
#[path = "gb/cpu.rs"]
pub mod cpu;
#[path = "gb/dma.rs"]
pub mod dma;
#[path = "gb/input.rs"]
pub mod input;
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
use crossbeam_channel::bounded;
use fps_counter::FPSCounter;

use input::{Input, InputManager};
use rendering::Renderer;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self};
use std::time::{Duration, Instant, SystemTime};

pub struct GameBoyEngine {
    pub paused: Arc<AtomicBool>,
    pub running: bool,
}

impl GameBoyEngine {
    pub fn new() -> GameBoyEngine {
        GameBoyEngine {
            paused: Arc::new(AtomicBool::new(false)),
            running: true,
        }
    }

    pub fn start(&mut self, path: &str) {
        let paused = Arc::clone(&self.paused);
        let (running_sender, running_receiver) = channel();
        let (buffer_sender, buffer_receiver) = channel();
        // CPU thread
        let cartridge = Cartridge::from_path(path).unwrap();

        let (input_sender, input_receiver) = bounded(1);
        let cpu_thread = thread::spawn(move || {
            let mut bus = Bus::new();
            bus.set_cartridge(cartridge);
            let mut cpu = CPU::new(bus);
            loop {
                if paused.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                // Single CPU step
                let step_result = cpu.cpu_step();
                if cpu.bus.ppu.have_update() {
                    _ = buffer_sender.send(cpu.bus.ppu.get_video_buffer());
                }
                if !input_receiver.is_empty() {
                    cpu.bus.ioram.input.last_input = input_receiver.recv().unwrap();
                }

                if step_result < 0 {
                    println!("CPU Error: step_result = {}", step_result);
                    _ = running_sender.send(false);
                    break;
                }

                thread::yield_now();
            }
        });

        let mut ui = Renderer::new();

        let mut fps = FPSCounter::default();

        // let mut fps_counter = FPSCounter::new();

        while self.running {
            // calc FPS...
            if let Ok(buffer) = buffer_receiver.recv() {
                _ = input_sender.send(ui.update(buffer).clone());
            }
            let fps = fps.tick();
            println!("FPS: {fps}");

            if ui.exited {
                exit(0);
            }
            if let Some(running) = running_receiver.try_iter().next() {
                if !running {
                    exit(0);
                }
            }
            thread::yield_now();
        }
        // }
        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();

        println!("Finished app");
    }
}
