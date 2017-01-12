extern crate gfx;
extern crate glutin;

use self::glutin::Event::{Closed, KeyboardInput};
use self::glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use self::glutin::ElementState::Pressed;

pub struct App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
    pub window: glutin::Window,
    pub encoder: gfx::Encoder<R, C>,
    pub device: D,
    pub slice: gfx::Slice<R>,
}

impl<R, C, D> App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
    pub fn new(w: glutin::Window,
               e: gfx::Encoder<R, C>,
               d: D,
               s: gfx::Slice<R>) -> Self {
        App { window: w,
              encoder: e,
              device: d,
              slice: s,
        }
    }
}
   
