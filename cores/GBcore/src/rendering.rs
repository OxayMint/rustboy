// use crate::GameBoyEngine::
use super::{input::Input, io::lcd::COLORS};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, rect::Point};
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
            // imgbuf: image::ImageBuffer::new<>(160, 144),
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
            _ = canvas.set_scale(3.0, 3.0);
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.present();

            // let imgbuf: image::ImageBuffer<PixelFormatEnum::RGB24, 256, 256>,
            // let texture_creator = canvas.texture_creator();

            // let mut texture = texture_creator
            //     .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            //     .map_err(|e| e.to_string())?;
        }
        self.event_pump = Some(sdl_context.event_pump()?);
        Ok(())
    }

    pub fn update(&mut self, buffer: Vec<Color>) -> Option<&Input> {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(COLORS[0]);
            canvas.clear();
        }

        self.draw_main(buffer);
        // self.update_debug_window();
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
        let mut new_input = self.last_input.clone();
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
                        Keycode::Z => new_input.A = true,
                        Keycode::X => new_input.B = true,
                        Keycode::SPACE => new_input.Start = true,
                        Keycode::V => new_input.Select = true,
                        Keycode::UP => new_input.Up = true,
                        Keycode::DOWN => new_input.Down = true,
                        Keycode::LEFT => new_input.Left = true,
                        Keycode::RIGHT => new_input.Right = true,

                        _ => {}
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Z => new_input.A = false,
                        Keycode::X => new_input.B = false,
                        Keycode::SPACE => new_input.Start = false,
                        Keycode::V => new_input.Select = false,
                        Keycode::UP => new_input.Up = false,
                        Keycode::DOWN => new_input.Down = false,
                        Keycode::LEFT => new_input.Left = false,
                        Keycode::RIGHT => new_input.Right = false,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        if new_input == self.last_input {
            None
        } else {
            self.last_input = new_input;
            Some(&self.last_input)
        }

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        // The rest of the game loop goes here...
    }

    fn draw_main(&mut self, pixels: Vec<Color>) {
        // for (x, y, pixel) in self.imgbuf.enumerate_pixels_mut() {
        //     println!("x is {x}");
        //     let color = pixels[(y + ((x) * 160)) as usize];
        //     *pixel = image::Rgb([color.r, color.g, color.b]);
        // }

        if let Some(canvas) = &mut self.canvas {
            for row in 0..144 {
                for col in 0..160 {
                    // imgbuf.
                    let color = pixels[col as usize + (row * 160) as usize];
                    // let pixel = imgbuf.get_pixel_mut(col, row);
                    // *pixel = image::Rgb([color.r, color.g, color.b]);
                    canvas.set_draw_color(color);
                    _ = canvas.draw_point(Point::new(col, row));
                    // canvas.
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
                self.display_debug_tile(debug_vram, tile_index, x_draw + x, y_draw + y);
                x_draw += 8;
                tile_index += 1;
            }
            y_draw += 8;
            x_draw = 160;
        }
    }

    fn display_debug_tile(&mut self, debug_vram: [u8; 8192], tile_index: usize, x: i32, y: i32) {
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
