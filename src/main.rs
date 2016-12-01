// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::Event::{Closed, KeyboardInput};
use glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use glutin::ElementState::Pressed;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const SQUARE: [Vertex; 6] = [
    Vertex { pos: [ -0.5,  0.5 ], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [  0.5,  0.5 ], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [  0.5,  0.5 ], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 1.0, 1.0] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

const BOX_SIZE: u32 = 20;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Tetris!".to_string())
        .with_dimensions(10*BOX_SIZE, 22*BOX_SIZE);

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, ());

    let data = pipe::Data {
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
