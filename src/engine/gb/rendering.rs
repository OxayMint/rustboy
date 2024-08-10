use raylib::{color::Color, ffi::CloseWindow, prelude::RaylibDraw};

// use sdl2::{self, Error};
pub struct Renderer {
    pub tick: u32,
    pub exited: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            tick: 0,
            exited: false,
        }
    }
    pub fn init(&mut self) {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        }
        // rl.
        self.exited = true;
    }
    pub fn handle_events(&mut self) {}
    pub fn tick(&mut self) {}
    pub fn exit(&mut self) {
        unsafe { CloseWindow() };
    }
}
