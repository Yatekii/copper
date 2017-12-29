use std::ops;


use glium;
use glium_text_rusttype;
use euclid;
use lyon;

use lyon::tessellation;


use schema_parser::component;
use schema_parser::component::geometry::SchemaSpace;


pub struct ScreenSpace;

/* * * * * * * * * * * * * * * * * * * *
 *
 * Vertex Ops
 *
 * * * * * * * * * * * * * * * * * * * */

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

pub struct VertexCtor;
impl lyon::lyon_tessellation::VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());
        
        Vertex {
            position: vertex.position.to_array(),
        }
    }
}
impl lyon::lyon_tessellation::VertexConstructor<tessellation::StrokeVertex, Vertex> for VertexCtor {
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

/* * * * * * * * * * * * * * * * * * * *
 *
 * Color Ops
 *
 * * * * * * * * * * * * * * * * * * * */

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

/* * * * * * * * * * * * * * * * * * * *
 *
 * Transform Ops
 *
 * * * * * * * * * * * * * * * * * * * */

#[derive(Debug, Clone)]
pub struct Transform2D(pub euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace>);

impl ops::Deref for Transform2D {
    type Target = euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace>;
    fn deref(&self) -> &euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace> {
        let &Transform2D(ref mat) = self;
        mat
    }
}

impl ops::DerefMut for Transform2D {
    fn deref_mut(&mut self) -> &mut euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace> {
        let &mut Transform2D(ref mut mat) = self;
        mat
    }
}

impl glium::uniforms::AsUniformValue for Transform2D {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        let &Transform2D(ref mat) = self;
        glium::uniforms::UniformValue::Mat3([
            [mat.m11, mat.m21, 0.0 ],
            [mat.m21, mat.m22, 0.0 ],
            [mat.m31, mat.m32, 0.0 ]
        ])
    }
}

impl From<euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace>> for Transform2D {
    fn from(t: euclid::TypedTransform2D<f32, SchemaSpace, ScreenSpace>) -> Transform2D {
        Transform2D(t)
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
    pub fn default() -> Self {
        GroupDrawable {
            drawables: Vec::new()
        }
    }

    pub fn add<T: 'static + Drawable>(&mut self, drawable: T) {
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
    pub system: &'a glium_text_rusttype::TextSystem,
    pub text: glium_text_rusttype::TextDisplay<&'a glium_text_rusttype::FontTexture>,
    pub transform: euclid::TypedTransform3D<f32, SchemaSpace, SchemaSpace>
}

impl<'a> Drawable for TextDrawable<'a> {
    fn draw(&self, target: &mut glium::Frame, perspective: Transform2D) {
        let p = perspective.to_3d();
        let t = &self.transform;
        let transform = t.post_mul(&p);

        let p = euclid::TypedPoint3D::<f32, SchemaSpace>::new(-250.0, 0.0, 0.0);
        let t = transform.transform_point3d(&p);
        println!("T: {:?}", transform);
        println!("{} => {}", p, t);

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

pub struct ViewState {
    pub current_perspective: Transform2D,
    pub width: isize,
    pub height: isize,
    scale: f32,
    center: euclid::TypedPoint2D<f32, SchemaSpace>,
    pub cursor: euclid::TypedPoint2D<f32, ScreenSpace>
}

impl ViewState {
    pub fn new(w: u32, h: u32) -> ViewState {
        let mut vs = ViewState {
            current_perspective: euclid::TypedTransform2D::<f32, SchemaSpace, ScreenSpace>::identity().into(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 200.0,
            center: euclid::TypedPoint2D::origin(),
            cursor: euclid::TypedPoint2D::origin()
        };
        vs.update_perspective();
        vs
    }

    pub fn update_from_resize(&mut self, width: u32, height: u32) {
        self.width = width as isize;
        self.height = height as isize;
        self.update_perspective();
    }

    pub fn update_from_box_pan(&mut self, &(ref min, ref max): &(component::geometry::Point, component::geometry::Point)) {
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

        self.current_perspective = euclid::TypedTransform2D::<f32, SchemaSpace, ScreenSpace>::create_scale(self.scale * aspect_ratio, self.scale)
                                                            .pre_translate(self.center - euclid::TypedPoint2D::origin())
                                                            .into();
    }
}