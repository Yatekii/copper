use lyon::tessellation;


use super::Vertex;

/// NOTE: The actual struct is defined in the super module, because it is part of the
/// gfx_defines!() macro. Only the implementation itself is done in this file.

pub struct VertexCtor;
impl tessellation::VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        Vertex {
            position: vertex.position.to_array(),
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}
impl tessellation::VertexConstructor<tessellation::StrokeVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> Vertex {
        Vertex {
            position: vertex.position.to_array(),
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}