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


const VS_CODE: &[u8] = include_bytes!("shaders/shape.glslv");
const FS_CODE: &[u8] = include_bytes!("shaders/shape.glslf");


pub struct DrawableComponent {
    pub component: component::Component,
    drawables: Vec<Box<drawing::Drawable>>,
    pub bounding_box: (Point, Point),
    pub instance: Option<ComponentInstance>
}

impl DrawableComponent {
    pub fn new(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, component: component::Component) -> DrawableComponent {
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

    pub fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: &drawing::Transform3D){
        for drawable in &self.drawables {
            drawable.draw(resource_manager.clone(), perspective.clone());
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
        &geometry::GraphicElement::Pin { ref orientation, ref position, length, ref name, number, number_size, name_size, .. } => {
            let pos = position.to_euclid();
            Some(Box::new(load_pin(resource_manager, pos, length as f32, orientation, name.clone(), number, number_size, name_size)))
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
    let w = StrokeOptions::default().with_line_width(6.5);

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

    let shader = resource_manager.borrow_mut().factory.link_program(&VS_CODE, &FS_CODE).unwrap();
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
    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_circle(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, center: SchemaPoint, radius: f32, fill: bool) -> drawing::DrawableObject<Resources> {
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

    let shader = resource_manager.borrow_mut().factory.link_program(&VS_CODE, &FS_CODE).unwrap();
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
    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

const PIN_RADIUS: f32 = 10.0;

fn load_pin(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, position: SchemaPoint, length: f32, orientation: &geometry::PinOrientation, name: Option<String>, number: usize, number_size: usize, name_size: usize) -> drawing::GroupDrawable {
    // Create a new group drawable
    let mut group = drawing::GroupDrawable::default();

    let circle = load_circle(resource_manager.clone(), position, PIN_RADIUS, false);

    let orientation_vec = orientation.unit_vec();
    let end_position = position + (orientation_vec * length);

    let number_pos = end_position + (orientation_vec * -10.0);
    let number_pos = geometry::Point { x: number_pos.x, y: number_pos.y + 60.0 };

    let number_orientation = match orientation {
        &geometry::PinOrientation::Up => geometry::TextOrientation::Vertical,
        &geometry::PinOrientation::Down => geometry::TextOrientation::Vertical,
        &geometry::PinOrientation::Right => geometry::TextOrientation::Horizontal,
        &geometry::PinOrientation::Left => geometry::TextOrientation::Horizontal
    };

    let number_hjustify = match orientation {
        &geometry::PinOrientation::Up => component::Justify::Right,
        &geometry::PinOrientation::Down => component::Justify::Left,
        &geometry::PinOrientation::Right => component::Justify::Right,
        &geometry::PinOrientation::Left => component::Justify::Left
    };

    let number_text = load_text(resource_manager.clone(), &number_pos, &format!("{}", number), number_size as f32, &number_orientation, number_hjustify, component::Justify::Center);

    if let Some(name) = name {
        let name_pos = end_position + orientation_vec * 20.0;
        let name_pos = geometry::Point { x: name_pos.x, y: name_pos.y + 25.0 };
        let name_hjustify = match orientation {
            &geometry::PinOrientation::Up => component::Justify::Left,
            &geometry::PinOrientation::Down => component::Justify::Right,
            &geometry::PinOrientation::Right => component::Justify::Left,
            &geometry::PinOrientation::Left => component::Justify::Right
        };
        let name_text = load_text(resource_manager.clone(), &name_pos, &name, name_size as f32, &number_orientation, name_hjustify, component::Justify::Center);
        group.add(name_text);
    }

    let line = load_line(resource_manager, position, end_position);

    group.add(line);
    group.add(circle);
    group.add(number_text);

    group
}

pub fn load_line(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, start: SchemaPoint, end: SchemaPoint) -> drawing::DrawableObject<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(start.to_untyped());
    points.push(end.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
        &mesh.vertices[..],
        &mesh.indices[..]
    );

    let shader = resource_manager.borrow_mut().factory.link_program(&VS_CODE, &FS_CODE).unwrap();
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
    let line = drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0));

    line
}

pub fn load_polygon(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, points: &Vec<geometry::Point>, fill: bool) -> drawing::DrawableObject<Resources> {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(6.5);

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

    let shader = resource_manager.borrow_mut().factory.link_program(&VS_CODE, &FS_CODE).unwrap();
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

    drawing::DrawableObject::new(bundle, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_text(_resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, position: &geometry::Point, content: &String, dimension: f32, orientation: &geometry::TextOrientation, hjustify: component::Justify, vjustify: component::Justify) -> drawing::TextDrawable {
    drawing::TextDrawable {
        position: position.clone(),
        content: content.clone(),
        dimension: dimension,
        orientation: orientation.clone(),
        hjustify: hjustify,
        vjustify: vjustify
    }
}