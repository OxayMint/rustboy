pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod dma;
pub mod input;
pub mod instruction;
pub mod interrupts;
pub mod io;
pub mod ppu;
pub mod rendering;
pub mod timer;
use bus::Bus;
use cartridge::Cartridge;
use cpu::CPU;
use crossbeam_channel::bounded;
use fps_counter::FPSCounter;
use rendering::Renderer;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self};
use std::time::Duration;

pub struct GBCore {
    pub paused: Arc<AtomicBool>,
    pub running: bool,
}

impl GBCore {
    pub fn new() -> GBCore {
        GBCore {
            paused: Arc::new(AtomicBool::new(false)),
            running: true,
        }
    }

    pub fn start(&mut self, path: String) {
        let paused = Arc::clone(&self.paused);
        let (running_sender, running_receiver) = channel();
        let (buffer_sender, buffer_receiver) = channel();
        // CPU thread
        let cartridge = Cartridge::from_path(path).unwrap();
        println!("{}", cartridge.info.to_string());
        // exit(0);
        let (input_sender, input_receiver) = bounded(1);
        let (save_requester, save_maker) = bounded(1);
        let cpu_thread = thread::spawn(move || {
            let mut bus = Bus::new();
            bus.set_cartridge(cartridge);
            bus.set_request_interrupt_fn();
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
                    cpu.bus.ioram.borrow_mut().input.last_input = input_receiver.recv().unwrap();
                }
                if !save_maker.is_empty() {
                    _ = save_maker.recv();
                    if let Some(ref cart) = cpu.bus.cart {
                        cart.save_ram();
                    }
                }
                if step_result < 0 {
                    println!("CPU Error: step_result = {}", step_result);
                    if let Some(ref cart) = cpu.bus.cart {
                        cart.save_ram();
                    }
                    _ = running_sender.send(false);

                    break;
                }

                thread::yield_now();
            }
        });

        let mut ui = Renderer::new();

        let mut fps = FPSCounter::default();

        while self.running {
            // calc FPS...
            if let Ok(buffer) = buffer_receiver.recv() {
                let input = ui.update(buffer);
                if let Some(input) = input {
                    _ = input_sender.send(input.clone());
                }
            }
            let fps = fps.tick();
            println!("FPS: {fps}");

            if ui.exited {
                _ = save_requester.send(true);
                exit(0);
            }
            if let Some(running) = running_receiver.try_iter().next() {
                if !running {
                    exit(0);
                }
            }
            thread::yield_now();
        }
        println!("Finished app");
        // }
        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();
    }
}
