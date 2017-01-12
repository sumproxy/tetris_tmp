#[macro_use]
extern crate gfx;
extern crate glutin;

use gfx::Device;

use glutin::Event::{Closed, KeyboardInput};
use glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use glutin::ElementState::Pressed;

mod common;
mod app;

use app::App;

fn main() {
    let mut app = App::new();//window, encoder, device, slice, pso, data);

    'main: loop {
        // loop over events
        for event in app.window.poll_events() {
            match event {
                KeyboardInput(_, _, Some(Escape)) | Closed => break 'main,
                
                KeyboardInput(Pressed, _, Some(Left)) => {
                    println!("Left");
                },
                KeyboardInput(Pressed, _, Some(Right)) => {
                    println!("Right");
                },
                KeyboardInput(Pressed, _, Some(Up)) => {
                    println!("Up");
                },
                KeyboardInput(Pressed, _, Some(Down)) => {
                    println!("Down");
                },
                KeyboardInput(Pressed, _, Some(Space)) => {
                    println!("Space");
                },
                
                _ => {},
            }
        }
        // draw a frame
        app.encoder.clear(&app.data.out, app.screen.clear_color);
        app.encoder.draw(&app.slice, &app.pso, &app.data);
        app.encoder.flush(&mut app.device);
        app.window.swap_buffers().unwrap();
        app.device.cleanup();
    }
}
