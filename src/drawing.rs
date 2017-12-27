use std::ops;


use glium;
use euclid;


use lyon::extra::rust_logo::build_logo_path;
use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{FillTessellator, FillOptions, StrokeOptions};
use lyon::tessellation;

use lyon::tessellation::geometry_builder::SimpleBuffersBuilder;
use lyon::lyon_tessellation::basic_shapes::*;
use lyon::lyon_tessellation::geometry_builder::simple_builder;


use schema_parser;

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
    pub fn new(x: f32, y: f32) -> Vertex { Vertex { position: [x, y] } }
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
            [mat.m11, mat.m12, 0.0 ],
            [mat.m21, mat.m22, 0.0 ],
            [mat.m31 ,mat.m32, 0.0 ]
        ])
    }
}

impl From<euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>> for Transform2D {
    fn from(t: euclid::TypedTransform2D<f32, KicadSpace, ScreenSpace>) -> Transform2D {
        Transform2D(t)
    }
}

pub fn ge_to_drawable(display: &glium::Display, shape: &schema_parser::component::geometry::GraphicElement) -> Option<Drawable> {
    match shape {
        &schema_parser::component::geometry::GraphicElement::Rectangle { ref start, ref end, .. } => {
            let r = euclid::Rect::<f32>::from_points(
                &[euclid::Point2D::<f32>::new(start.x as f32, start.y as f32),
                    euclid::Point2D::<f32>::new(end.x as f32, end.y as f32)]
            );
            Some(load_rectangle(display, &r))
        }
        _ => None
    }
}

pub fn load_rectangle(display: &glium::Display, rectangle: &euclid::Rect::<f32>) -> Drawable {
    let mut mesh = VertexBuffers::new();

    let r = BorderRadii::new_all_same(5.0);
    let w = StrokeOptions::default().with_line_width(3.0);

    let _ = stroke_rounded_rectangle(rectangle, &r, &w, &mut BuffersBuilder::new(&mut mesh, VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();

    let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    Drawable::new(vertex_buffer, indices, program)
}

pub struct Drawable {
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program
}

impl Drawable {
    pub fn new(vertices: glium::VertexBuffer<Vertex>, indices: glium::IndexBuffer<u16>, program: glium::Program) -> Self {
        Drawable {
            vertices: vertices,
            indices: indices,
            program: program
        }
    }

    pub fn draw<U: glium::uniforms::Uniforms>(&self, target: &mut glium::Frame, uniforms: &U){
        use glium::Surface;
        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            uniforms,
            &Default::default(),
        ).unwrap();
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
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
