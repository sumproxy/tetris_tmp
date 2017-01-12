#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;

use gfx::Device;

mod common;
mod app;

use app::GameState;

fn main() {
    let mut app = app::new();
    loop {
        match app.handle_events() {
            GameState::Quit => break,
            GameState::Running => (),
        }
        app.draw_frame();
    }
}
