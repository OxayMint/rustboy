use std::{
    ops::{AddAssign, SubAssign},
    time::Duration,
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    rect::Rect,
};

use super::bus::Bus;
pub static SCALE: i32 = 2;

pub struct Renderer {
    pub tick: i32,
    pub exited: bool,
    event_pump: Option<sdl2::EventPump>,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

static TILE_COLORS: [Color; 4] = [
    Color::WHITE,
    Color::RGBA(0xAA, 0xAA, 0xAA, 0xFF),
    Color::RGBA(0x55, 0x55, 0x55, 0xFF),
    Color::RGBA(0x00, 0x00, 0x00, 0xFF),
];
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            tick: 0,
            exited: false,
            event_pump: None,
            canvas: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(
                "Rustboy",
                640 + (SCALE as u32 * 8 * 16),
                SCALE as u32 * 8 * 32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        self.canvas = Some(window.into_canvas().build().map_err(|e| e.to_string())?);
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.present();
        }
        self.event_pump = Some(sdl_context.event_pump()?);

        Ok(())
    }

    pub fn tick(&mut self) {
        self.tick.add_assign(1);
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
        }
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
                    _ => {}
                }
            }
        }

        // if let Some(canvas) = &mut self.canvas {
        //     canvas.clear();
        //     canvas.present();
        // }
        self.update_debug_window();
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        // The rest of the game loop goes here...
    }

    fn update_debug_window(&mut self) {
        let (mut x_draw, mut y_draw, mut tile_index) = (640, 0, 0);
        //384 tile. 24x16
        let addr = 0x8000usize;

        for y in 0..24 {
            for x in 0..16 {
                self.display_tile(addr, tile_index, x_draw + x * SCALE, y_draw + y * SCALE);
                x_draw.add_assign(8 * SCALE);
                tile_index.add_assign(1);
            }
            y_draw.add_assign(8 * SCALE);
            x_draw = 640;
        }
    }

    fn display_tile(&mut self, addr: usize, tile_index: usize, x: i32, y: i32) {
        if let Some(canvas) = &mut self.canvas {
            for row in 0..16 {
                let b1 = Bus::read8(addr + (tile_index * 16) + row);
                let b2 = Bus::read8(addr + (tile_index * 16) + row + 1);
                let mut bit = 7i32;
                while bit >= 0 {
                    let hi = (b1 >> bit & 1) << 1;
                    let low = b2 >> bit & 1;
                    let col = TILE_COLORS[(hi | low) as usize];
                    let r = Rect::new(
                        x + (7 - bit) * SCALE,
                        y + (row as i32 / 2 * SCALE),
                        SCALE as u32,
                        SCALE as u32,
                    );

                    canvas.set_draw_color(col);
                    _ = canvas.fill_rect(r);
                    bit.sub_assign(1);
                }
            }
        }
    }
}
