use std::ops;


use glium;
use glium_text_rusttype;
use euclid;


use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{StrokeOptions};
use lyon::tessellation;

use lyon::lyon_tessellation::basic_shapes::*;

use schema_parser::component;
use schema_parser::component::geometry;
use resource_manager::{FontKey, ResourceManager};
use schema_parser::component::geometry::{SchemaSpace, SchemaPoint};

pub struct KicadSpace {

}

pub struct ScreenSpace {

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn x(&self) -> f32 { self.position[0] }
    pub fn y(&self) -> f32 { self.position[1] }
    // pub fn new(x: f32, y: f32) -> Vertex { Vertex { position: [x, y] } }
}

// A very simple vertex constructor that only outputs the vertex position
struct VertexCtor;
impl VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());
        
        Vertex {
            position: vertex.position.to_array(),
        }
    }
}
impl VertexConstructor<tessellation::StrokeVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());
        Vertex {
            position: vertex.position.to_array(),
        }
    }
}

impl ops::Add<Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, _rhs: Vertex) -> Vertex {
        Vertex {
            position: [
                self.x() + _rhs.x(),
                self.y() + _rhs.y()
            ]
        }
    }
}

impl ops::Sub<Vertex> for Vertex {
    type Output = Vertex;

    fn sub(self, _rhs: Vertex) -> Vertex {
        Vertex {
            position: [
                self.x() - _rhs.x(),
                self.y() - _rhs.y()
            ]
        }
    }
}

impl glium::uniforms::AsUniformValue for Vertex {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec2(self.position)
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub color: [f32; 4]
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color { Color { color: [r, g, b, a] } }
}

implement_uniform_block!(Color, color);

impl glium::uniforms::AsUniformValue for Color {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec4(self.color)
    }
}

#[derive(Debug, Clone)]
pub struct Transform2D(pub euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>);

impl ops::Deref for Transform2D {
    type Target = euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>;
    fn deref(&self) -> &euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace> {
        let &Transform2D(ref mat) = self;
        mat
    }
}

impl ops::DerefMut for Transform2D {
    fn deref_mut(&mut self) -> &mut euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace> {
        let &mut Transform2D(ref mut mat) = self;
        mat
    }
}

impl glium::uniforms::AsUniformValue for Transform2D {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        let &Transform2D(ref mat) = self;
        glium::uniforms::UniformValue::Mat3([
            // [mat.m11, mat.m21, mat.m31 ],
            // [mat.m12, mat.m22, mat.m32 ],
            // [  0.0  ,   0.0  ,   0.0   ]
            [mat.m11, mat.m21, 0.0 ],
            [mat.m21, mat.m22, 0.0 ],
            [  mat.m31  ,   mat.m32  ,   0.0   ]
        ])
    }
}

impl From<euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>> for Transform2D {
    fn from(t: euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>) -> Transform2D {
        Transform2D(t)
    }
}

pub fn ge_to_drawable<'a>(resource_manager: &'a ResourceManager, shape: &geometry::GraphicElement) -> Option<Box<Drawable + 'a>> {
    match shape {
        &geometry::GraphicElement::Rectangle { ref start, ref end, .. } => {
            let r = euclid::TypedRect::from_points(
                &[start.to_euclid(), end.to_euclid()]
            );
            Some(Box::new(load_rectangle(resource_manager, &r)))
        }
        &geometry::GraphicElement::Circle { ref center, radius, .. } => {
            let center = center.to_euclid();
            Some(Box::new(load_circle(resource_manager, center, radius)))
        },
        &geometry::GraphicElement::Pin { ref orientation, ref position, length, .. } => {
            let pos = position.to_euclid();
            Some(Box::new(load_pin(resource_manager, pos, length as f32, orientation)))
        },
        &geometry::GraphicElement::Polygon { ref points, .. } => {
            Some(Box::new(load_polygon(resource_manager, points)))
        },
        &geometry::GraphicElement::TextField { ref content, ref position, .. } => {
            Some(Box::new(load_text(resource_manager, position, content, 30.0)))
        }
        _ => None
    }
}

pub fn field_to_drawable<'a>(resource_manager: &'a ResourceManager, field: &component::Field) -> Box<Drawable + 'a> {
    Box::new(load_text(resource_manager, &field.position, &field.text, field.dimension))
}

pub fn load_rectangle(resource_manager: &ResourceManager, rectangle: &euclid::TypedRect<f32, SchemaSpace>) -> DrawableObject {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);
    let w = StrokeOptions::default().with_line_width(3.0);

    let _ = stroke_rounded_rectangle(&rectangle.to_untyped(), &r, &w, &mut BuffersBuilder::new(&mut mesh, VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    DrawableObject::new(vertex_buffer, indices, program, Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_circle(resource_manager: &ResourceManager, center: SchemaPoint, radius: f32) -> DrawableObject {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let _ = stroke_circle(center.to_untyped(), radius, &w, &mut BuffersBuilder::new(&mut mesh, VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    DrawableObject::new(vertex_buffer, indices, program, Color::new(0.61, 0.05, 0.04, 1.0))
}

const PIN_RADIUS: f32 = 10.0;

fn load_pin(resource_manager: &ResourceManager, position: SchemaPoint, length: f32, orientation: &geometry::PinOrientation) -> GroupDrawable {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let circle = load_circle(resource_manager, position, PIN_RADIUS);

    let orientation_vec = orientation.unit_vec();
    let end_position = position + (orientation_vec * length);

    let is_closed = false;

    let mut points = Vec::new();

    points.push(position.to_untyped());
    points.push(end_position.to_untyped());

    let _ = stroke_polyline(points.into_iter(), is_closed, &w, &mut BuffersBuilder::new(&mut mesh, VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let line = DrawableObject::new(vertex_buffer, indices, program, Color::new(0.61, 0.05, 0.04, 1.0));

    let mut group = GroupDrawable::default();

    group.add(line);
    group.add(circle);

    group
}

pub fn load_polygon(resource_manager: &ResourceManager, points: &Vec<geometry::Point>) -> DrawableObject {
    let mut mesh = VertexBuffers::new();

    let w = StrokeOptions::default().with_line_width(3.0);

    let is_closed = false;

    let _ = stroke_polyline(
        points.iter().map(|p| p.to_euclid().to_untyped() ),
        is_closed,
        &w,
        &mut BuffersBuilder::new(&mut mesh, VertexCtor)
    );

    let vertex_buffer = glium::VertexBuffer::new(resource_manager.display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        resource_manager.display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(resource_manager.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    DrawableObject::new(vertex_buffer, indices, program, Color::new(0.61, 0.05, 0.04, 1.0))
}

pub fn load_text<'a>(resource_manager: &'a ResourceManager, &geometry::Point { x, y }: &geometry::Point, content: &String, dimension: f32) -> TextDrawable<'a> {
    let font = resource_manager.get_font(&FontKey {
        size: 100,
        path: "/Users/yatekii/repos/schema_renderer/test_data/Inconsolata-Regular.ttf".into()
    }).unwrap();

    TextDrawable {
        system: &resource_manager.text_system,
        text: glium_text_rusttype::TextDisplay::new(&resource_manager.text_system, font, content),
        transform: euclid::TypedTransform3D::<f32, KicadSpace, KicadSpace>::create_translation(
                                                                                x as f32 - dimension / 2.0,
                                                                                y as f32 - dimension / 2.0,
                                                                                0.0
                                                                            )
                                                                           .pre_scale(dimension, dimension, dimension)
    }
}

pub struct DrawableObject {
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
    color: Color
}

impl DrawableObject {
    pub fn new(vertices: glium::VertexBuffer<Vertex>, indices: glium::IndexBuffer<u16>, program: glium::Program, color: Color) -> Self {
        DrawableObject {
            vertices: vertices,
            indices: indices,
            program: program,
            color: color
        }
    }
}

impl Drawable for DrawableObject{
    fn draw(&self, target: &mut glium::Frame, perspective: Transform2D){

        let uniforms  = uniform!{
            perspective: perspective,
            color: self.color
        };

        use glium::Surface;
        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniforms,
            &Default::default(),
        ).unwrap();
    }
}

pub struct GroupDrawable {
    drawables: Vec<Box<Drawable>>
}

impl GroupDrawable {
    fn default() -> Self {
        GroupDrawable {
            drawables: Vec::new()
        }
    }

    fn add<T: 'static + Drawable>(&mut self, drawable: T) {
        self.drawables.push(Box::new(drawable));
    }
}

impl Drawable for GroupDrawable {
    fn draw(&self, target: &mut glium::Frame, perspective: Transform2D) {
        for drawable in &self.drawables {
            drawable.draw(target, perspective.clone());
        }
    }
}

pub struct TextDrawable<'a> {
    system: &'a glium_text_rusttype::TextSystem,
    text: glium_text_rusttype::TextDisplay<&'a glium_text_rusttype::FontTexture>,
    transform: euclid::TypedTransform3D<f32, KicadSpace, KicadSpace>
}

impl<'a> Drawable for TextDrawable<'a> {
    fn draw(&self, target: &mut glium::Frame, perspective: Transform2D) {
        let p = perspective.to_3d();
        let t = &self.transform;
        let transform = t.post_mul(&p);

        let _ = glium_text_rusttype::draw(
            &self.text,
            &self.system,
            target,
            transform.to_row_arrays(),
            (1.0, 0.0, 0.0, 1.0)
        );
    }
}

pub trait Drawable {
    fn draw(&self, target: &mut glium::Frame, perspective: Transform2D);
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

pub struct ViewState {
    pub current_perspective: Transform2D,
    width: isize,
    height: isize,
    scale: f32,
    center: euclid::TypedPoint2D<f32, KicadSpace>
}

impl ViewState {
    pub fn new(w: u32, h: u32) -> ViewState {
        let mut vs = ViewState {
            current_perspective: euclid::TypedTransform2D::<f32, KicadSpace, ScreenSpace>::identity().into(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 200.0,
            center: euclid::TypedPoint2D::origin()
        };
        vs.update_perspective();
        vs
    }

    pub fn update_from_resize(&mut self, width: u32, height: u32) {
        self.width = width as isize;
        self.height = height as isize;
        self.update_perspective();
    }

    pub fn update_from_box_pan(&mut self, (min, max): (component::geometry::Point, component::geometry::Point)) {
        let m = (max.x - min.x).max(max.y - min.y);
        if m > 0.0 {
            self.scale = 1.9 / m;
            let w = max.x + min.x;
            let h = max.y + min.y;
            self.center = euclid::TypedPoint2D::new(
                -w / 2.0,
                -h / 2.0
            );
            self.update_perspective();
        }
    }

    pub fn update_perspective(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);

        self.current_perspective = euclid::TypedTransform2D::<f32, KicadSpace, ScreenSpace>::create_scale(self.scale * aspect_ratio, self.scale)
                                                            .pre_translate(self.center - euclid::TypedPoint2D::origin())
                                                            .into();
    }
}