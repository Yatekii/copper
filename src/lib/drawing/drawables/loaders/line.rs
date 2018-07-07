use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math::Point;


use geometry;
use drawing;
use drawing::drawables;


pub fn load_line(
    component_id: u32,
    color: drawing::Color,
    start: &geometry::Point2D,
    end: &geometry::Point2D,
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(Point::new(start.x, start.y));
    points.push(Point::new(end.x, end.y));

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex {
            position: v.position.clone(),
            color: color.color,
            component_id,
        }).collect(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect(),
        abo: vec![]
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}