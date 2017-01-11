extern crate gfx;
extern crate glutin;

use self::glutin::Event::{Closed, KeyboardInput};
use self::glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use self::glutin::ElementState::Pressed;

pub struct App<R, C>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>
{
    pub window: glutin::Window,
    pub encoder: gfx::Encoder<R, C>
}

impl<R, C> App<R, C>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>
{
    pub fn new(w: glutin::Window, e: gfx::Encoder<R, C>) -> Self {
        App { window: w,
              encoder: e,
        }
    }
}
   
