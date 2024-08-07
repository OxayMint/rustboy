use crate::libs::cartridge::Cartridge;
use crate::libs::cpu::CPU;
use crate::libs::memory::Memory;
use crate::libs::rendering::Renderer;
use crate::libs::timer::Timer;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

pub struct GameBoyEngine {
    pub paused: Arc<AtomicBool>,
    pub running: Arc<AtomicBool>,
    // pub renderer: Renderer,
    pub memory: Arc<Mutex<Memory>>,
    // pub timer: Timer,
    pub ticks: Arc<Mutex<u32>>,
    // pub stack: Vec<u8>,
}

impl GameBoyEngine {
    pub fn new(path: &str) -> GameBoyEngine {
        let cartridge = Cartridge::from_path(path).unwrap();
        let memory = Memory::new(cartridge);

        GameBoyEngine {
            // renderer: Renderer::new(),
            memory: Arc::new(Mutex::new(memory)),
            paused: Arc::new(AtomicBool::new(false)),
            running: Arc::new(AtomicBool::new(true)),
            // timer: Timer::new(),
            // stack: vec![],
            ticks: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start(&mut self) {
        let memory = Arc::clone(&self.memory);
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

                let mut memory_lock = memory.lock().unwrap();
                if !cpu.cpu_step(&mut memory_lock) {
                    println!("CPU Stopped");
                    running.store(false, Ordering::Relaxed);
                    break;
                }
                drop(memory_lock);

                let mut tick_lock = ticks.lock().unwrap();
                *tick_lock += 1;
            }
        });

        // UI thread (main thread)
        let mut ui = Renderer::new();
        ui.init();

        while self.running.load(Ordering::Relaxed) {
            ui.tick();
            // Here you can add logic to read from memory if needed
            // let memory_lock = self.memory.lock().unwrap();
            // ... read from memory_lock ...
            // drop(memory_lock);

            thread::sleep(Duration::from_millis(10));

            // Check for exit condition (e.g., user input)
            if ui.exit {
                self.running.store(false, Ordering::Relaxed);
            }
        }

        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();

        println!("Finished app");
    }
}
