use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::lyon_tessellation::FillTessellator;
use gfx;
use gfx_device_gl;


use drawables;
use drawing;
use schema_parser::geometry;


type Resources = gfx_device_gl::Resources;


pub fn load_polygon(
    color: drawing::Color,
    points: &Vec<geometry::SchemaPoint2D>,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    let is_closed = false;

    if fill {
        let _ = fill_polyline(
            points.iter().map(|p| p.to_untyped()),
            &mut FillTessellator::new(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_polyline(
            points.iter().map(|p| p.to_untyped() ),
            is_closed,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.clone(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect()
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}