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
}

pub const SCREEN: Screen = Screen {
    x: 10,
    y: 22,
};

impl Screen {
    pub const fn width(&self) -> u32 {
        self.x * BOX.size
    }

    pub const fn height(&self) -> u32 {
        self.y * BOX.size
    }
}

pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
    pub size: u32,
}

macro_rules! width {
    () => (1.0 / SCREEN.width() as f32)
}

pub const BOX: Quad = Quad {
    vertices: [
        Vertex { pos: [-width!(), -width!()] },
        Vertex { pos: [-width!(),  width!()] },
        Vertex { pos: [ width!(), -width!()] },
        Vertex { pos: [ width!(),  width!()] },
    ],
    indices: [0, 1, 2, 1, 2, 3],
    size: 20,
};

pub const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
