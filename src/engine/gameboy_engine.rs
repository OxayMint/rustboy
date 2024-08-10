use crate::libs::cartridge::Cartridge;
use crate::libs::cpu::CPU;
use crate::libs::memory::Memory;
use crate::libs::rendering::Renderer;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::AddAssign;
use std::sync::atomic::AtomicU32;
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
    // pub ticks: Arc<Mutex<u32>>,
    // pub stack: Vec<u8>,
    pub ui: Renderer,
    pub ticks: Arc<AtomicU32>,
}

impl GameBoyEngine {
    pub fn new(path: &str) -> GameBoyEngine {
        let cartridge = Cartridge::from_path(path).unwrap();
        let memory = Memory::new(cartridge);
        GameBoyEngine {
            ui: Renderer::new(),
            memory: Arc::new(Mutex::new(memory)),
            paused: Arc::new(AtomicBool::new(false)),
            running: Arc::new(AtomicBool::new(true)),
            ticks: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn start(&mut self) {
        let memory = Arc::clone(&self.memory);
        let running = Arc::clone(&self.running);
        let paused = Arc::clone(&self.paused);
        let ticks = Arc::clone(&self.ticks);

        // UI thread (main thread)
        // self.ui.init();
        // CPU thread

        let cpu_thread = thread::spawn(move || {
            let mut cpu = CPU::new();
            // let running =
            let mut i: i64 = 1;

            let mut data = "Some data!";
            let f = File::create("./log.log").expect("Unable to create file");
            let mut f = BufWriter::new(f);
            println!("{}", data);
            while running.load(Ordering::Relaxed) {
                if paused.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                // print!("{i:04}: ");
                // if i > 151400 + 2 {
                //     break;
                // }
                i.add_assign(1);
                //0xff is checkpoint
                // Single CPU step
                let step_result = {
                    let mut memory_lock = memory.lock().unwrap();
                    let res = cpu.cpu_step(&mut memory_lock);

                    f.write_all(res.as_bytes()).expect("Unable to write data");
                    res
                };
                // if step_result < 0 {
                //     println!("CPU Error: step_result = {}", step_result);
                //     running.store(false, Ordering::Relaxed);
                //     break;
                // }

                ticks.fetch_add(1, Ordering::Relaxed);

                // Update EmuDebug
                {
                    // let mut memory_lock = memory.lock().unwrap();
                    // let mut debug = EMU_DEBUG.lock().unwrap();
                    // debug.update(&mut *memory_lock);
                }
                //  010011101 - 157
                //  111011001 - 473
                //step 1
                //  010011001 - c
                //  101000100 - a
                //  100110010 - b
                //step 2
                //  0100000000 - c
                //  0001110110 - a
                //  1000000000 - b

                //step 3
                //  0000000000 - c
                //  1001110110 - a
                //  0000000000 - b
                //  1001110110 = 630

                // Yield to other threads
                // thread::yield_now();
            }
            f.flush().unwrap();
        });
        // let frame_duration = Duration::from_millis(16);

        // while self.running.load(Ordering::Relaxed) {
        //     println!("HELLLOO ####################################################################################");
        //     // let frame_start = Instant::now();

        //     self.ui.tick();

        //     // Print debug information
        //     let debug = EMU_DEBUG.lock().unwrap();
        //     debug.print();

        //     // Check for exit condition
        //     if self.ui.exited {
        //         self.running.store(false, Ordering::Relaxed);
        //     }
        //     thread::sleep(Duration::from_millis(10));

        //     // Sleep for the remaining frame time
        //     // if let Some(sleep_time) = frame_duration.checked_sub(frame_start.elapsed()) {
        //     // thread::sleep(sleep_time);
        //     // }
        // }

        // Wait for CPU thread to finish
        cpu_thread.join().unwrap();

        println!("Finished app");
    }
    // pub fn start(&mut self) {
    //     let memory = Arc::clone(&self.memory);
    //     let running = Arc::clone(&self.running);
    //     let paused = Arc::clone(&self.paused);
    //     let ticks = Arc::clone(&self.ticks);

    //     let (tx, rx) = mpsc::channel();

    //     // CPU thread
    //     let cpu_thread = thread::spawn(move || {
    //         let result = panic::catch_unwind(AssertUnwindSafe(|| {
    //             let mut cpu = CPU::new();
    //             while running.load(Ordering::Relaxed) {
    //                 if paused.load(Ordering::Relaxed) {
    //                     thread::sleep(Duration::from_millis(10));
    //                     continue;
    //                 }

    //                 let mut memory_lock = memory.lock().unwrap();
    //                 if cpu.cpu_step(&mut memory_lock) < 0 {
    //                     println!("CPU Error");
    //                     running.store(false, Ordering::Relaxed);
    //                     break;
    //                 }

    //                 // Update EmuDebug after each CPU step
    //                 {
    //                     let mut debug = EMU_DEBUG.lock().unwrap();
    //                     debug.update(&mut *memory_lock);
    //                 }

    //                 drop(memory_lock);

    //                 let mut tick_lock = ticks.lock().unwrap();
    //                 *tick_lock += 1;

    //                 // Signal the UI thread that a step is complete
    //                 tx.send(()).unwrap();
    //             }
    //             // Ok(())
    //         }));

    //         // ... (rest of the error handling code)
    //     });
    //     println!("Cpu thread created ####################################################################################");

    //     // UI thread (main thread)
    //     let mut ui = Renderer::new();
    //     ui.init();
    //     println!("HELLLOO ####################################################################################");

    //     while self.running.load(Ordering::Acquire) {
    //         println!("HELLLOO ####################################################################################");
    //         // Wait for the CPU to complete a step
    //         rx.recv().unwrap();

    //         ui.tick();

    //         // Now we can safely access memory
    //         {
    //             let memory_lock = self.memory.lock().unwrap();
    //             // Perform any necessary UI updates using memory data
    //             let mut debug = EMU_DEBUG.lock().unwrap();
    //             debug.print();
    //         }

    //         // Check for exit condition (e.g., user input)
    //         if ui.exited {
    //             self.running.store(false, Ordering::Release);
    //             break;
    //         }
    //     }

    //     // Final debug print before exiting
    //     let debug = EMU_DEBUG.lock().unwrap();
    //     debug.print();

    //     // Wait for CPU thread to finish
    //     if let Err(e) = cpu_thread.join() {
    //         println!("Failed to join CPU thread: {:?}", e);
    //     }

    //     println!("Finished app");
    // }
}
