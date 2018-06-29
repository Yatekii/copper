use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::FillOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};

use euclid::{
    Point2D as ePoint2,
    Size2D as eSize2,
    Rect as eRect
};

use geometry;
use drawing;
use drawing::drawables;


pub fn load_rectangle(
    color: drawing::Color,
    rectangle: &geometry::AABB,
    fill: bool
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);
    // Euclid rectangles have the origin at the top left which means
    //      X = leftmost point in normal notation
    //      Y = bottommost point in normal notation as Y is inverted
    //          (Y positive points downwards on the screen)
    let euclid_rectangle = eRect::new(
        ePoint2::new(rectangle.mins().x, rectangle.mins().y),
        eSize2::<f32>::new(rectangle.half_extents().x, rectangle.half_extents().y) * 2.0
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