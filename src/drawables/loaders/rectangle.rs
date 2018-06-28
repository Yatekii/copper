use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::FillOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};


use schema_parser::geometry;
use drawables;
use drawing;

use euclid::{Point2D as ePoint2, Size2D as eSize2, Rect as eRect};


pub fn load_rectangle(
    color: drawing::Color,
    rectangle: &geometry::AABB,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);
    let euclid_rectangle = eRect::new(
        ePoint2::new(rectangle.center().x, rectangle.center().y),
        eSize2::<f32>::new(rectangle.half_extents().x, rectangle.half_extents().y)
    );

    if fill {
        let _ = fill_rounded_rectangle(&euclid_rectangle, &r, &FillOptions::default(), &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));
    } else {
        let w = StrokeOptions::default().with_line_width(6.5);
        let _ = stroke_rounded_rectangle(
            &euclid_rectangle,
            &r, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex { position: v.position, color: color.color }).collect(),
        ibo: mesh.indices.iter().map(|i| *i as u32).collect()
    };
    
    drawables::ShapeDrawable::new(buffers, color)
}