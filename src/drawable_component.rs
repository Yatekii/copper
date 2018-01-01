use glium;
use euclid;

use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::lyon_tessellation::FillTessellator;
use lyon::lyon_tessellation::basic_shapes::*;


use drawing;
use schema_parser::component;
use schema_parser::component::geometry;
use resource_manager::{ResourceManager};

use schema_parser::component::geometry::{SchemaSpace, SchemaPoint};
use schema_parser::component::geometry::Point;


pub struct DrawableComponent<'a> {
    component: component::Component,
    drawables: Vec<Box<drawing::Drawable + 'a>>,
    bounding_box: (Point, Point)

}

impl<'a> DrawableComponent<'a> {
    pub fn new(resource_manager: &'a ResourceManager, component: component::Component) -> 
    DrawableComponent<'a> {
        let mut drawables: Vec<Box<drawing::Drawable + 'a>> = component.graphic_elements.iter()
                                                        .filter_map(|shape| ge_to_drawable(resource_manager, &shape))
                                                        .collect();
        drawables.extend(
            component.fields.iter()
                                 .filter(|field| field.visible)
                                 .map(|shape| field_to_drawable(resource_manager, &shape))
        );
        let bb = component.get_boundingbox();

        DrawableComponent {
            component: component,
            drawables: drawables,
            bounding_box: bb
        }
    }

    pub fn draw(&self, target: &mut glium::Frame, perspective: &drawing::Transform2D){
        for drawable in &self.drawables {
            drawable.draw(target, perspective.clone());
        }
    }

    pub fn get_bounding_box(&self) -> &(Point, Point) {
        &self.bounding_box
    }
}

pub fn ge_to_drawable<'a>(resource_manager: &'a ResourceManager, shape: &geometry::GraphicElement) -> Option<Box<drawing::Drawable + 'a>> {
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

pub fn field_to_drawable<'a>(resource_manager: &'a ResourceManager, field: &component::Field) -> Box<drawing::Drawable + 'a> {
    Box::new(load_text(resource_manager, &field.position, &field.text, field.dimension as f32, &field.orientation, field.hjustify.clone(), field.vjustify.clone()))
}

pub fn load_rectangle(resource_manager: &ResourceManager, rectangle: &euclid::TypedRect<f32, SchemaSpace>, fill: bool) -> drawing::DrawableObject {
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

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    drawing::DrawableObject::new(vertex_buffer, indices, program, drawing::
    Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_circle(resource_manager: &ResourceManager, center: SchemaPoint, radius: f32, fill: bool) -> drawing::DrawableObject {
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

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    drawing::DrawableObject::new(vertex_buffer, indices, program, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

const PIN_RADIUS: f32 = 10.0;

fn load_pin(resource_manager: &ResourceManager, position: SchemaPoint, length: f32, orientation: &geometry::PinOrientation) -> drawing::GroupDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let circle = load_circle(resource_manager, position, PIN_RADIUS, false);

    let orientation_vec = orientation.unit_vec();
    let end_position = position + (orientation_vec * length);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(position.to_untyped());
    points.push(end_position.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, drawing::VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let line = drawing::DrawableObject::new(vertex_buffer, indices, program, drawing::Color::new(0.61, 0.05, 0.04, 1.0));

    let mut group = drawing::GroupDrawable::default();

    group.add(line);
    group.add(circle);

    group
}

pub fn load_polygon(resource_manager: &ResourceManager, points: &Vec<geometry::Point>, fill: bool) -> drawing::DrawableObject {
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

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    drawing::DrawableObject::new(vertex_buffer, indices, program, drawing::Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_text<'a>(resource_manager: &'a ResourceManager, position: &geometry::Point, content: &String, dimension: f32, orientation: &geometry::TextOrientation, hjustify: component::Justify, vjustify: component::Justify) -> drawing::TextDrawable<'a> {
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

pub static VERTEX_SHADER: &'static str = r#"
    #version 140
    in vec2 position;
    uniform mat3 perspective;

    void main() {
        vec3 pos = vec3(position, 1.0);
        gl_Position = vec4(perspective * pos, 1.0);
    }
"#;

pub static FRAGMENT_SHADER: &'static str = r#"
    #version 140

    uniform vec4 color;

    out vec4 col;
    void main() {
        col = color;
    }
"#;