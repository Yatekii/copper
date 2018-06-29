use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::FillOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};


use geometry;
use drawing::drawables;
use drawing;
use euclid;


pub fn load_circle(
    color: drawing::Color,
    center: &geometry::Point2D,
    radius: f32,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    if fill {
        let _ = fill_circle(
            euclid::point2(center.x, center.y),
            radius,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_circle(
            euclid::point2(center.x, center.y),
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