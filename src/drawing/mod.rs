mod color;
mod vertex;
mod view_state;


use gfx;
use gfx_device_gl;


pub use self::color::Color;
pub use self::vertex::VertexCtor;
pub use self::view_state::ViewState;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
type Resources = gfx_device_gl::Resources;


gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "position",
    }

    constant Locals {
        color: [f32; 4] = "color",
        perspective: [[f32; 4]; 4] = "perspective",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}