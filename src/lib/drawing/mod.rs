pub mod color;
pub mod drawables;
pub mod schema_drawer;
pub mod component_drawer;
mod gfx_machinery;

mod buffers;
mod vertex;

use gfx;

pub use self::color::Color;
pub use self::vertex::VertexCtor;
pub use self::buffers::Buffers;


pub type ColorFormat = gfx::format::Rgba8;


gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "position",
        color: [f32; 4] = "color",
        id: u32 = "id",
    }

    constant Globals {
        perspective: [[f32; 4]; 4] = "perspective",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target1",
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        attributes: gfx::ConstantBuffer<Attributes> = "u_attributes",
    }

    constant Attributes {
        transform: [[f32; 4]; 4] = "transform",
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