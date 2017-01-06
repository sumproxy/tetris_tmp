pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx;

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
    x: u32,
    y: u32,
    pub clear_color: [f32; 4],
    pub elem: Quad,
}

impl Screen {
    pub fn new() -> Self {
        let elem = Quad::new();
        let mut screen = Screen { x: 10, y: 22, clear_color: [0.1, 0.1, 0.1, 1.0], elem: elem };
        let width = 1.0 / screen.x as f32;
        let height = 1.0 / screen.y as f32;
        screen.elem.set_vertices(width, height);
        screen
    }
    
    pub fn width(&self) -> u32 {
        self.x * self.elem.size
    }

    pub fn height(&self) -> u32 {
        self.y * self.elem.size
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
