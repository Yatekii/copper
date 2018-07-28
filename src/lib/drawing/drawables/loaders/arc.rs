use lyon::tessellation::{
    StrokeOptions,
    FillOptions,
    FillTessellator,
    StrokeTessellator,
    geometry_builder::{
        VertexBuffers,
        BuffersBuilder,
    }
};
use lyon::path::default::Path;
use lyon::path::builder::PathBuilder;
use lyon::path::builder::FlatPathBuilder;
use lyon::math::{
    Point,
    Vector,
    Angle
};

use geometry;
use drawing::drawables;
use drawing;


pub fn load_arc(
    component_id: u32,
    color: drawing::Color,
    center: &geometry::Point2,
    radius: f32,
    _fill: bool,
    start_angle: f32,
    end_angle: f32,
) -> drawables::ShapeDrawable {
    let mut mesh = VertexBuffers::new();
println!("1");
    let mut builder = Path::builder();
    builder.arc(
        Point::new(center.x, center.y), 
        Vector::new(radius, radius), 
        Angle::radians(end_angle - start_angle), 
        Angle::radians(0.0)
    );
println!("2");
    let path = builder.build();
println!("3");
    if false {
        let mut tessellator = FillTessellator::new();
        let _ = tessellator.tessellate_path(
            path.path_iter(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let mut tessellator = StrokeTessellator::new();
        let _ = tessellator.tessellate_path(
            path.path_iter(),
            &StrokeOptions::default().with_line_width(6.5),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }
println!("4");
    let buffers = drawing::Buffers {
        vbo: mesh.vertices.iter().map(|v| drawing::Vertex {
            position: v.position.clone(),
            color: color.color,
            id: component_id,
        }).collect(),
        ibo: mesh.indices.iter().map(|i: &u16| *i as u32).collect(),
        abo: vec![]
    };

    println!("KEK");
    
    drawables::ShapeDrawable::new(buffers, color)
}