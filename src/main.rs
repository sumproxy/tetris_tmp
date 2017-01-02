#![feature(const_fn)]
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::Event::{Closed, KeyboardInput};
use glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use glutin::ElementState::Pressed;

mod common;
use common::{ColorFormat, DepthFormat, BOX, SCREEN, CLEAR_COLOR, pipe};


fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Tetris!".to_string())
        .with_dimensions(SCREEN.width(), SCREEN.height());

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&BOX.vertices, &BOX.indices as &[u16]);

    let data = pipe::Data {
        color: [1.0, 0.0, 0.0],
        center: [0.5, 0.5],
        vbuf: vertex_buffer,
        out: main_color
    };

    'main: loop {
        // loop over events
        for event in window.poll_events() {
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
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
