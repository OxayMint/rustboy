#[path = "gb/bus.rs"]
pub mod bus;
#[path = "gb/cartridge.rs"]
pub mod cartridge;
#[path = "gb/cpu.rs"]
pub mod cpu;
#[path = "gb/dma.rs"]
pub mod dma;
#[path = "gb/model/input.rs"]
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
use rendering::Renderer;

use crate::libs::gameboy::ppu::PPU_INSTANCE;
use fps_counter::FPSCounter;
use std::sync::atomic::AtomicU32;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self};
use std::time::{Duration, Instant, SystemTime};

pub struct GameBoyEngine {
    pub paused: Arc<AtomicBool>,
    pub running: Arc<AtomicBool>,
    pub ui: Renderer,
    pub ticks: Arc<AtomicU32>,
    pub current_frame: u32,

    pub target_frame_time: u128,
    pub frame_count: u64,
    pub last_frame_time: SystemTime,
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

            current_frame: 0,
            frame_count: 0,
            last_frame_time: SystemTime::now(),
            target_frame_time: 1000 / 60,
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

        let mut fps = FPSCounter::default();

        // let mut fps_counter = FPSCounter::new();

        let target_fps = 60.0;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);

        let mut frame_start = Instant::now();
        while self.running.load(Ordering::Relaxed) {
            let mut ppu = PPU_INSTANCE.lock().unwrap();
            if ppu.have_update() {
                // if true {
                // calc FPS...
                let video_buffer = ppu.get_video_buffer();
                let debug_ram = ppu.vram;
                drop(ppu);

                let input = self.ui.update(video_buffer, debug_ram); // Handle UI updates
                let input = input as u8;
                println!("input: {input:b}");
                CPU::request_interrupt(interrupts::InterruptType::JOYPAD);
                let elapsed = frame_start.elapsed();
                let sleep_time: Duration;
                if elapsed < frame_duration {
                    sleep_time = frame_duration - elapsed;
                    // println!("sleeping {sleep_time:?}");
                    thread::sleep(sleep_time);
                }
                // let fps = fps.tick();
                let fps = fps.tick();
                // Update and print FPS
                println!("FPS: {fps}",);
                // println!("FPS: {fps}. Frame duration: {elapsed:?}. Slept {sleep_time:?}",);

                // self.current_frame = self.current_frame.wrapping_add(1);

                // let end = SystemTime::now();

                // let frame_duration = end.duration_since(self.last_frame_time).unwrap();
                // if fps > 60 {
                //     println!("FPS: {fps}. Sleeping");
                //     let sleep_time = fps % 60 / 60;
                //     //
                //     sleep(Duration::from_millis(sleep_time as u64));
                // } else {
                //     println!("FPS: {fps}. Not sleeping");
                // }

                // // unsafe { sleep(1) };
                // if end
                //     .duration_since(self.last_frame_time)
                //     .unwrap()
                //     .as_millis()
                //     >= 1000
                // {
                //     println!("FPS: {}", self.frame_count);
                //     self.last_frame_time = end;
                //     self.frame_count = 0;
                // }
                // self.frame_count += 1;

                // // if true {
                frame_start = Instant::now();
            }
            if self.ui.exited {
                self.running.store(false, Ordering::Relaxed);
            }
            thread::yield_now();
        }
        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();

        println!("Finished app");
    }
}
