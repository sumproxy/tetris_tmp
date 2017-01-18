#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate bit_vec;

mod common;
mod app;

fn main() {
    let mut app = app::new();
    while app.is_running() {
        app.handle_events();
        app.draw_frame();
    }
}
