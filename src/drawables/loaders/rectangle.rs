use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::FillOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};


use schema_parser::geometry;
use drawables;
use drawing;


pub fn load_rectangle(
    color: drawing::Color,
    rectangle: &geometry::SchemaRect,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);

    if fill {
        let _ = fill_rounded_rectangle(&rectangle.to_untyped(), &r, &FillOptions::default(), &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));
    } else {
        let w = StrokeOptions::default().with_line_width(6.5);
        let _ = stroke_rounded_rectangle(&rectangle.to_untyped(), &r, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));
    }

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex { position: v.position, color: color.color }).collect(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect()
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}