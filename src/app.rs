extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

use gfx::traits::FactoryExt;

use self::glutin::Event::{Closed, KeyboardInput};
use self::glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use self::glutin::ElementState::Pressed;

use common::{ColorFormat, DepthFormat, Screen, pipe};

pub struct App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
    pub window: glutin::Window,
    pub encoder: gfx::Encoder<R, C>,
    pub device: D,
    pub slice: gfx::Slice<R>,
    pub pso: gfx::PipelineState<R, pipe::Meta>,
    pub data: pipe::Data<R>,
    pub screen: Screen,
}

impl<R, C, D> App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
}
   
pub fn new<R, C, D>(
    w: glutin::Window,
    e: gfx::Encoder<R, C>,
    d: D,
    s: gfx::Slice<R>,
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>
) -> App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{

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

    App::<R, C, D> { window: window,
                         encoder: encoder,
                         device: device,
                         slice: slice,
                         pso: pso,
                         data: data,
                         screen: screen,
    }
}
