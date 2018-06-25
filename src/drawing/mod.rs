pub mod color;
mod vertex;
mod view_state;
mod buffers;


use gfx;


pub use self::color::Color;
pub use self::vertex::VertexCtor;
pub use self::view_state::ViewState;
pub use self::buffers::Buffers;


pub type ColorFormat = gfx::format::Rgba8;


gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "position",
        color: [f32; 4] = "color",
    }

    constant Locals {
        perspective: [[f32; 4]; 4] = "perspective",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target1",
    }
}

gfx_defines!{
    vertex VertexRender {
        position: [f32; 2] = "position",
    }

    pipeline pipe_render {
        vbuf: gfx::VertexBuffer<VertexRender> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "Render",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}