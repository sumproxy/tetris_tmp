pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx;

use bit_vec::BitVec;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline pipe {
        center: gfx::Global<[f32; 2]> = "u_center",
        color: gfx::Global<[f32; 3]> = "u_color",
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "target",
    }
}

pub struct Screen {
    pub clear_color: [f32; 4],
    pub elem: Quad,
    frame: Frame,
}

impl Screen {
    pub fn new() -> Self {
        let elem = Quad::new();
        let frame = Frame::new(10, 22);
        let clear_color = [0.1, 0.1, 0.1, 1.0];
        let mut screen = Screen { clear_color: clear_color, elem: elem, frame: frame };
        let width = 1.0 / screen.frame.x() as f32;
        let height = 1.0 / screen.frame.y() as f32;
        screen.elem.set_vertices(width, height);
        screen
    }
    
    pub fn width(&self) -> u32 {
        self.elem.size * self.frame.x() as u32
    }

    pub fn height(&self) -> u32 {
        self.elem.size * self.frame.y() as u32
    }
}

pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
    size: u32,
}

impl Quad {
    fn new() -> Self {
        Quad {
            vertices: [Vertex { pos: [0.0, 0.0] } ; 4],
            indices: [0, 1, 2, 1, 2, 3],
            size: 20,
        }
    }
    fn set_vertices(&mut self, width: f32, height: f32) {
        self.vertices = [
            Vertex { pos: [-width, -height] },
            Vertex { pos: [-width,  height] },
            Vertex { pos: [ width, -height] },
            Vertex { pos: [ width,  height] },
        ]
    }
}

struct Frame {
    inner: Vec<BitVec>
}

impl Frame {
    fn new(x: usize, y: usize) -> Self {
        let zeroes = BitVec::from_fn(x, |_| { false });
        Frame {
            inner: vec![zeroes; y],
        }
    }

    fn x(&self) -> usize {
        self.inner[0].len()
    }

    fn y(&self) -> usize {
        self.inner.len()
    }
}
