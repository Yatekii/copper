use std::cell::RefCell;
use std::rc::Rc;


use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::lyon_tessellation::FillTessellator;
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;


use drawables;
use drawing;
use geometry;
use resource_manager;


type Resources = gfx_device_gl::Resources;


pub fn load_polygon(
    resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    color: drawing::Color,
    points: &Vec<geometry::SchemaPoint2D>,
    fill: bool
) -> drawables::ShapeDrawable<Resources> {
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

    let shader = resource_manager.borrow_mut().factory.link_program(&super::VS_CODE, &super::FS_CODE).unwrap();
    let mut rasterizer = gfx::state::Rasterizer::new_fill();
    rasterizer.samples = Some(gfx::state::MultiSample);
    let program = resource_manager.borrow_mut().factory.create_pipeline_from_program(
        &shader,
        gfx::Primitive::TriangleList,
        rasterizer,
        drawing::pipe::new()
    ).unwrap();

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data {vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });

    drawables::ShapeDrawable::new(bundle, color)
}