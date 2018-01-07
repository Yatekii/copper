use std::cell::RefCell;
use std::rc::Rc;


use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::StrokeOptions;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;


use geometry;
use drawables;
use drawing;
use resource_manager;


type Resources = gfx_device_gl::Resources;


pub fn load_circle(
    resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    color: drawing::Color,
    center: geometry::SchemaPoint2D,
    radius: f32,
    fill: bool
) -> drawables::ShapeDrawable<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    if fill {
        let _ = fill_circle(
            center.to_untyped(),
            radius,
            0.1,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_circle(
            center.to_untyped(),
            radius,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

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
    drawables::ShapeDrawable::new(bundle, color)
}