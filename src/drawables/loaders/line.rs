use std::cell::RefCell;
use std::rc::Rc;


use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;


use schema_parser::geometry;
use drawables;
use drawing;
use resource_manager;


type Resources = gfx_device_gl::Resources;


pub fn load_line(
    resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    vbo: &mut Vec<drawing::Vertex>,
    vbi: &mut Vec<u32>,
    color: drawing::Color,
    start: &geometry::SchemaPoint2D,
    end: &geometry::SchemaPoint2D
) -> drawables::ShapeDrawable<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(start.to_untyped());
    points.push(end.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let len = vbo.len();
    vbo.extend(&mesh.vertices);
    vbi.extend(&(mesh.indices.iter().map(|i| *i as u32 + len as u32).collect()) as &Vec<u32>);

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let shader = resource_manager.borrow_mut().factory.link_program(&super::VS_CODE, &super::FS_CODE).unwrap();
    let mut rasterizer = gfx::state::Rasterizer::new_fill();
    rasterizer.samples = Some(gfx::state::MultiSample);
    let program = resource_manager.borrow_mut().factory.create_pipeline_from_program(
        &shader,
        gfx::Primitive::TriangleList,
        rasterizer,
        drawing::pipe::new()
    ).unwrap();

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data { vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });
    let line = drawables::ShapeDrawable::new(bundle, color);

    line
}