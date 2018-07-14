use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::lyon_tessellation::FillTessellator;
use lyon::math::Point;

use drawing;
use drawing::drawables;
use geometry;


pub fn load_polygon(
    component_id: u32,
    color: drawing::Color,
    points: &Vec<geometry::Point2D>,
    fill: bool,
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    if fill {
        let _ = fill_polyline(
            points.iter().map(|p| Point::new(p.x, p.y)),
            &mut FillTessellator::new(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let is_closed = false;
        let w = StrokeOptions::default().with_line_width(6.5);
        let _ = stroke_polyline(
            points.iter().map(|p| Point::new(p.x, p.y)),
            is_closed,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex {
            position: v.position.clone(),
            color: color.color,
            component_id,
        }).collect(),
        ibo: mesh.indices.iter().map(|i: &u16| *i as u32).collect(),
        abo: vec![]
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}