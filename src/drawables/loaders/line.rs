use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};


use schema_parser::geometry;
use drawables;
use drawing;


pub fn load_line(
    color: drawing::Color,
    start: &geometry::SchemaPoint2D,
    end: &geometry::SchemaPoint2D
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(start.to_untyped());
    points.push(end.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex { position: v.position, color: color.color }).collect(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect()
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}