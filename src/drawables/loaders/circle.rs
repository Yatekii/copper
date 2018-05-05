use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};


use schema_parser::geometry;
use drawables;
use drawing;


pub fn load_circle(
    color: drawing::Color,
    center: &geometry::SchemaPoint2D,
    radius: f32,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    if fill {
        let _ = fill_circle(
            center.to_untyped(),
            radius,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_circle(
            center.to_untyped(),
            radius,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }


    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex { position: v.position, color: color.color }).collect(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect()
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}