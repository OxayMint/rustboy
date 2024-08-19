// use crate::GameBoyEngine::
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use super::{input::Input, io::lcd::COLORS};
pub static SCALE: u32 = 3;

pub struct Renderer {
    // pub tick: u64,
    last_input: Input,
    pub exited: bool,
    event_pump: Option<sdl2::EventPump>,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Renderer {
            last_input: Input::new(),
            exited: false,
            event_pump: None,
            canvas: None,
        };
        _ = renderer.init();
        renderer
    }

    pub fn init(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Rustboy", 160 * SCALE, 144 * SCALE)
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

    pub fn update(&mut self, buffer: Vec<Color>) -> (&Input, bool, bool) {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(COLORS[0]);
            canvas.clear();
        }

        self.draw_main(buffer);
        // self.update_debug_window(debug_vram);
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
        let mut save = false;
        let mut load = false;
        if let Some(event_pump) = &mut self.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.exited = true;
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Z => self.last_input.A = true,
                        Keycode::X => self.last_input.B = true,
                        Keycode::SPACE => self.last_input.Start = true,
                        Keycode::V => self.last_input.Select = true,
                        Keycode::UP => self.last_input.Up = true,
                        Keycode::DOWN => self.last_input.Down = true,
                        Keycode::LEFT => self.last_input.Left = true,
                        Keycode::RIGHT => self.last_input.Right = true,
                        Keycode::I => save = true,
                        Keycode::O => load = true,
                        _ => {}
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Z => self.last_input.A = false,
                        Keycode::X => self.last_input.B = false,
                        Keycode::SPACE => self.last_input.Start = false,
                        Keycode::V => self.last_input.Select = false,
                        Keycode::UP => self.last_input.Up = false,
                        Keycode::DOWN => self.last_input.Down = false,
                        Keycode::LEFT => self.last_input.Left = false,
                        Keycode::RIGHT => self.last_input.Right = false,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        (&self.last_input, save, load)

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
                x_draw += 8;
                tile_index += 1;
            }
            y_draw += 8;
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
                    bit -= 1;
                }
            }
        }
    }
}
