use std::{
    ops::{AddAssign, SubAssign},
    sync::Mutex,
    time::Duration,
};
// use crate::GameBoyEngine::
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};

use super::{input::Input, io::lcd::COLORS};
pub static SCALE: u32 = 3;

pub struct Renderer {
    // pub tick: u64,
    pub exited: bool,
    event_pump: Option<sdl2::EventPump>,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            // tick: 0,
            exited: false,
            event_pump: None,
            canvas: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Rustboy", 160 * SCALE, 256 * SCALE)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        self.canvas = Some(window.into_canvas().build().map_err(|e| e.to_string())?);
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.present();
            _ = canvas.set_scale(3.0, 3.0);
        }
        self.event_pump = Some(sdl_context.event_pump()?);
        Ok(())
    }

    pub fn update(&mut self, buffer: Vec<Color>) -> Input {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(COLORS[0]);
            canvas.clear();
        }

        self.draw_main(buffer);
        // self.update_debug_window(debug_vram);
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
        let mut input = Input::None;
        if let Some(event_pump) = &mut self.event_pump {
            for event in event_pump.poll_iter() {
                input = match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.exited = true;
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Z),
                        ..
                    } => Input::A,
                    Event::KeyDown {
                        keycode: Some(Keycode::X),
                        ..
                    } => Input::B,
                    Event::KeyDown {
                        keycode: Some(Keycode::SPACE),
                        ..
                    } => Input::Start,
                    Event::KeyDown {
                        keycode: Some(Keycode::V),
                        ..
                    } => Input::Select,
                    Event::KeyDown {
                        keycode: Some(Keycode::UP),
                        ..
                    } => Input::Up,
                    Event::KeyDown {
                        keycode: Some(Keycode::DOWN),
                        ..
                    } => Input::Down,
                    Event::KeyDown {
                        keycode: Some(Keycode::LEFT),
                        ..
                    } => Input::Left,
                    Event::KeyDown {
                        keycode: Some(Keycode::RIGHT),
                        ..
                    } => Input::Right,
                    _ => Input::None,
                }
            }
        }
        input

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        // The rest of the game loop goes here...
    }

    fn draw_main(&mut self, pixels: Vec<Color>) {
        if let Some(canvas) = &mut self.canvas {
            for row in 0..144i32 {
                for col in 0..160i32 {
                    canvas.set_draw_color(pixels[col as usize + (row * 160) as usize]);
                    _ = canvas.draw_point(Point::new(col, row));
                }
            }
        }
    }
    //
    fn update_debug_window(&mut self, debug_vram: [u8; 8192]) {
        let (mut x_draw, mut y_draw, mut tile_index) = (160, 0, 0);
        //384 tile. 16x24

        for y in 0..24 {
            for x in 0..16 {
                self.display_tile(debug_vram, tile_index, x_draw + x, y_draw + y);
                x_draw.add_assign(8);
                tile_index.add_assign(1);
            }
            y_draw.add_assign(8);
            x_draw = 160;
        }
    }

    fn display_tile(&mut self, debug_vram: [u8; 8192], tile_index: usize, x: i32, y: i32) {
        if let Some(canvas) = &mut self.canvas {
            for row in 0..16 {
                let b1 = debug_vram[(tile_index * 16) + row];
                let b2 = debug_vram[(tile_index * 16) + row + 1];
                let mut bit = 7i32;
                while bit >= 0 {
                    let hi = (b1 >> bit & 1) << 1;
                    let low = b2 >> bit & 1;
                    let col = COLORS[(hi | low) as usize];
                    // let r = Rect::new(
                    //     x + (7 - bit),
                    //     y + (row as i32 / 2),
                    //     SCALE as u32,
                    //     SCALE as u32,
                    // );
                    canvas.set_draw_color(col);
                    _ = canvas.draw_point(Point::new(x + (7 - bit), y + (row as i32 / 2)));
                    // _ = canvas.fill_rect(r);
                    bit.sub_assign(1);
                }
            }
        }
    }
}
