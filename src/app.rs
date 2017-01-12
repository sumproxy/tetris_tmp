extern crate gfx;
extern crate glutin;

use self::glutin::Event::{Closed, KeyboardInput};
use self::glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use self::glutin::ElementState::Pressed;

use gfx::Device;

pub struct App<R, C>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>
{
    pub window: glutin::Window,
    pub encoder: gfx::Encoder<R, C>,
    pub device: gfx::Device,
}

impl<R, C> App<R, C>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>
{
    pub fn new(w: glutin::Window,
               e: gfx::Encoder<R, C>,
               d: gfx::Device) -> Self {
        App { window: w,
              encoder: e,
              device: d,
        }
    }
}
   
