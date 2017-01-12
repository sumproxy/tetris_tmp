#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::Event::{Closed, KeyboardInput};
use glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use glutin::ElementState::Pressed;

mod common;
mod app;

use app::App;

use common::{ColorFormat, DepthFormat, Screen, pipe};


fn main() {
    let screen = Screen::new();
    
    let builder = glutin::WindowBuilder::new()
        .with_title("Tetris!".to_string())
        .with_dimensions(screen.width(), screen.height());

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&screen.elem.vertices, &screen.elem.indices as &[u16]);

    let data = pipe::Data {
        color: [1.0, 0.0, 0.0],
        center: [0.0, 0.0],
        vbuf: vertex_buffer,
        out: main_color
    };

    let mut app = App::new(window, encoder, device, slice, pso);

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
        app.encoder.clear(&data.out, screen.clear_color);
        app.encoder.draw(&app.slice, &app.pso, &data);
        app.encoder.flush(&mut app.device);
        app.window.swap_buffers().unwrap();
        app.device.cleanup();
    }
}
