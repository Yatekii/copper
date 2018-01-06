use std::ops;
use std::cell::RefCell;
use std::rc::Rc;


use gfx;
use gfx_device_gl;
use gfx_glyph;
use euclid;
use lyon;

use lyon::tessellation;


use geometry;
use schema_parser::component;
use schema_parser::component::geometry::SchemaSpace;
use resource_manager;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
type Resources = gfx_device_gl::Resources;


gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "position",
    }

    constant Locals {
        color: [f32; 4] = "color",
        perspective: [[f32; 4]; 4] = "perspective",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

/* * * * * * * * * * * * * * * * * * * *
 *
 * Vertex Ops
 *
 * * * * * * * * * * * * * * * * * * * */

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

// impl glium::uniforms::AsUniformValue for Vertex {
//     fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
//         glium::uniforms::UniformValue::Vec2(self.position)
//     }
// }

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

/* * * * * * * * * * * * * * * * * * * *
 *
 * Transform Ops
 *
 * * * * * * * * * * * * * * * * * * * */

pub struct ViewState {
    pub current_perspective: geometry::TSchemaScreen,
    pub width: isize,
    pub height: isize,
    pub scale: f32,
    center: geometry::SchemaPoint3D,
    pub cursor: geometry::ScreenPoint3D
}

impl ViewState {
    pub fn new(w: u32, h: u32) -> ViewState {
        let mut vs = ViewState {
            current_perspective: geometry::TSchemaScreen::identity().into(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 6000.0,
            center: geometry::SchemaPoint3D::origin(),
            cursor: geometry::ScreenPoint3D::origin()
        };
        vs.update_perspective();
        vs
    }

    pub fn update_from_resize(&mut self, width: u32, height: u32) {
        self.width = width as isize;
        self.height = height as isize;
        self.update_perspective();
    }

    pub fn update_from_zoom(&mut self, delta: f32) {
        self.scale += delta / 10000.0;
        if self.scale < 1.0 / 60000.0 {
            self.scale = 1.0 / 60000.0;
        }
        if self.scale > 0.3 {
            self.scale = 0.3;
        }
        self.update_perspective();
    }

    pub fn update_from_box_pan(&mut self, &(ref min, ref max): &(component::geometry::Point, component::geometry::Point)) {
        let m = (max.x - min.x).max(max.y - min.y);
        if m > 0.0 {
            self.scale = 2.45 / m;
            let w = max.x + min.x;
            let h = max.y + min.y;
            self.center = geometry::SchemaPoint2D::new(
                -w / 2.0,
                -h / 2.0
            ).to_3d();
            self.update_perspective();
        }
    }

    pub fn update_perspective(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);

        self.current_perspective = geometry::TSchemaScreen::create_scale(self.scale * aspect_ratio, self.scale, 1.0)
                                                            .pre_translate(self.center - geometry::SchemaPoint3D::origin());
    }

    pub fn screen_space_to_pixels(&self, distance: f32) -> usize {
        (self.scale * distance / self.height as f32) as usize
    }
}