use gfx;
use gfx::traits::FactoryExt;

use gfx_window_glutin;
use gfx_device_gl as gfx_gl;

use glutin::{Window, WindowBuilder};
use glutin::Event::{Closed, KeyboardInput};
use glutin::VirtualKeyCode::{Left, Right, Up, Down, Space, Escape};
use glutin::ElementState::Pressed;

use common::{ColorFormat, DepthFormat, Screen, pipe};

enum GameState {
    Quit,
    Running,
}

pub struct App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
    window: Window,
    encoder: gfx::Encoder<R, C>,
    device: D,
    slice: gfx::Slice<R>,
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    screen: Screen,
    state: GameState,
}

impl<R, C, D> App<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device<Resources=R, CommandBuffer=C> {

    pub fn handle_events(&mut self) {
        // loop over events
        for event in self.window.poll_events() {
            match event {
                KeyboardInput(_, _, Some(Escape)) | Closed => {
                    self.state = GameState::Quit
                }
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
                _ => (),
            }
        }
    }

    pub fn draw_frame(&mut self) {
        self.encoder.clear(&self.data.out, self.screen.clear_color);
        self.encoder.draw(&self.slice, &self.pso, &self.data);
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }

    pub fn is_running(&self) -> bool {
        match self.state {
            GameState::Quit => false,
            _ => true,
        }
    }
}
   
pub type GlApp = App<gfx_gl::Resources, gfx_gl::CommandBuffer, gfx_gl::Device>;

pub fn new() -> GlApp {
    let screen = Screen::new();
    
    let builder = WindowBuilder::new()
        .with_title("Tetris!".to_string())
        .with_dimensions(screen.width(), screen.height());

    let (window, device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let encoder = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/tetris_150.glslv"),
        include_bytes!("shader/tetris_150.glslf"),
        pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&screen.elem.vertices, &screen.elem.indices as &[u16]);

    let data = pipe::Data {
        color: [1.0, 0.0, 0.0],
        center: [0.0, 0.0],
        vbuf: vertex_buffer,
        out: main_color
    };

    App { window: window,
          encoder: encoder,
          device: device,
          slice: slice,
          pso: pso,
          data: data,
          screen: screen,
          state: GameState::Running,
    }
}
