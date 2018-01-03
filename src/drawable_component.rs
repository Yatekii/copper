use std::cell::RefCell;
use std::rc::Rc;


use euclid;
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::lyon_tessellation::FillTessellator;
use lyon::lyon_tessellation::basic_shapes::*;
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;


use drawing;
use schema_parser::component;
use schema_parser::component::geometry;
use resource_manager;
use schema_parser::component::geometry::{SchemaSpace, SchemaPoint};
use schema_parser::component::geometry::Point;
use schema_parser::schema_file::ComponentInstance;


type Resources = gfx_device_gl::Resources;


const vs_code: &[u8] = include_bytes!("shaders/shape.glslv");
const fs_code: &[u8] = include_bytes!("shaders/shape.glslf");


pub struct DrawableComponent {
    pub component: component::Component,
    drawables: Vec<Box<drawing::Drawable>>,
    pub bounding_box: (Point, Point),
    pub instance: Option<ComponentInstance>
}

impl DrawableComponent {
    pub fn new(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, component: component::Component) -> 
    DrawableComponent {
        let mut drawables: Vec<Box<drawing::Drawable>> = component.graphic_elements.iter()
                                                        .filter_map(|shape| ge_to_drawable(resource_manager.clone(), &shape))
                                                        .collect::<Vec<_>>();
        drawables.extend(
            component.fields.iter()
                                 .filter(|field| field.visible)
                                 .map(|shape| field_to_drawable(resource_manager.clone(), &shape))
        );
        let bb = component.get_boundingbox();

        DrawableComponent {
            component: component,
            drawables: drawables,
            bounding_box: bb,
            instance: None
        }
    }

    pub fn draw(&self, encoder: &mut gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>, perspective: &drawing::Transform2D){
        for drawable in &self.drawables {
            drawable.draw(encoder, perspective.clone());
        }
    }

    pub fn get_bounding_box(&self) -> &(Point, Point) {
        &self.bounding_box
    }
}

pub fn ge_to_drawable(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, shape: &geometry::GraphicElement) -> Option<Box<drawing::Drawable>> {
    match shape {
        &geometry::GraphicElement::Rectangle { ref start, ref end, filled, .. } => {
            let r = euclid::TypedRect::from_points(
                &[start.to_euclid(), end.to_euclid()]
            );
            Some(Box::new(load_rectangle(resource_manager, &r, filled)))
        }
        &geometry::GraphicElement::Circle { ref center, radius, filled, .. } => {
            let center = center.to_euclid();
            Some(Box::new(load_circle(resource_manager, center, radius, filled)))
        },
        &geometry::GraphicElement::Pin { ref orientation, ref position, length, .. } => {
            let pos = position.to_euclid();
            Some(Box::new(load_pin(resource_manager, pos, length as f32, orientation)))
        },
        &geometry::GraphicElement::Polygon { ref points, filled, .. } => {
            Some(Box::new(load_polygon(resource_manager, points, filled)))
        },
        &geometry::GraphicElement::TextField { ref content, ref position, ref orientation, .. } => {
            Some(Box::new(load_text(resource_manager, position, content, 30.0, orientation, component::Justify::Center, component::Justify::Center)))
        }
        _ => None
    }
}

pub fn field_to_drawable<'a>(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, field: &component::Field) -> Box<drawing::Drawable> {
    Box::new(load_text(resource_manager, &field.position, &field.text, field.dimension as f32, &field.orientation, field.hjustify.clone(), field.vjustify.clone()))
}

pub fn load_rectangle(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, rectangle: &euclid::TypedRect<f32, SchemaSpace>, fill: bool) -> drawing::DrawableObject<Resources> {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);
    let w = StrokeOptions::default().with_line_width(3.0);

    if fill {
        let _ = fill_rounded_rectangle(
            &rectangle.to_untyped(),
            &r,
            0.1,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_rounded_rectangle(
            &rectangle.to_untyped(),
            &r,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let program = resource_manager.borrow_mut().factory.create_pipeline_simple(&vs_code.to_vec(), &fs_code.to_vec(), drawing::pipe::new()).unwrap();

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data { vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });
    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_circle(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, center: SchemaPoint, radius: f32, fill: bool) -> drawing::DrawableObject<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

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

    let program = resource_manager.borrow_mut().factory.create_pipeline_simple(&vs_code.to_vec(), &fs_code.to_vec(), drawing::pipe::new()).unwrap();

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data { vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });
    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

const PIN_RADIUS: f32 = 10.0;

fn load_pin(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, position: SchemaPoint, length: f32, orientation: &geometry::PinOrientation) -> drawing::GroupDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let circle = load_circle(resource_manager.clone(), position, PIN_RADIUS, false);

    let orientation_vec = orientation.unit_vec();
    let end_position = position + (orientation_vec * length);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(position.to_untyped());
    points.push(end_position.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let program = resource_manager.borrow_mut().factory.create_pipeline_simple(&vs_code.to_vec(), &fs_code.to_vec(), drawing::pipe::new()).unwrap();

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data { vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });
    let line = drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0));

    let mut group = drawing::GroupDrawable::default();

    group.add(line);
    group.add(circle);

    group
}

pub fn load_polygon(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, points: &Vec<geometry::Point>, fill: bool) -> drawing::DrawableObject<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let is_closed = false;

    if fill {
        let _ = fill_polyline(
            points.iter().map(|p| p.to_euclid().to_untyped()),
            &mut FillTessellator::new(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    } else {
        let _ = stroke_polyline(
            points.iter().map(|p| p.to_euclid().to_untyped() ),
            is_closed,
            &w,
            &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor)
        );
    }

    let program = resource_manager.borrow_mut().factory.create_pipeline_simple(&vs_code.to_vec(), &fs_code.to_vec(), drawing::pipe::new()).unwrap();

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

    let bundle = gfx::pso::bundle::Bundle::new(ibo, program, drawing::pipe::Data {vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });

    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_text(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, position: &geometry::Point, content: &String, dimension: f32, orientation: &geometry::TextOrientation, hjustify: component::Justify, vjustify: component::Justify) -> drawing::TextDrawable {
    drawing::TextDrawable {
        position: position.clone(),
        content: content.clone(),
        dimension: dimension,
        orientation: orientation.clone(),
        resource_manager: resource_manager,
        hjustify: hjustify,
        vjustify: vjustify
    }
}