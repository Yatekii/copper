use glium;
use glium::Surface;


use drawing::{Vertex,Color};


fn create_rectanguar_vertices(display: &glium::Display, position: Vertex, size: Vertex) -> (glium::VertexBuffer<Vertex>, glium::index::IndexBuffer<u8>) {
    let shape = vec![
        Vertex { position: [ position.x(), position.y() + size.y()] },
        position,
        position + size,
        Vertex { position: [ position.x() + size.x(),  position.y()] },
    ];
    let vertices = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u8,1,2,1,3,2]
    ).unwrap();
    (vertices, indices)
}

pub trait Shape {
    /// Issue a draw call to OGL
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters);
    fn grouped_draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, position: Vertex);
}

pub struct Rectangle {
    position: Vertex,
    size: Vertex,
    color: Color,
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::IndexBuffer<u8>,
    program: glium::Program
}

impl Rectangle {
    pub fn new(display: &glium::Display, position: Vertex, size: Vertex, color: Color) -> Rectangle {
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform bool grouped_draw;
            uniform vec2 group_position;
            out vec4 pos;
            void main() {
                if(grouped_draw){
                    gl_Position = vec4(group_position + position, 0.0, 1.0);
                } else {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
                pos = gl_Position;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec4 pos;
            uniform vec4 fill_color;
            out vec4 color;
            void main() {
                color = fill_color;
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let rectangle_buffers = create_rectanguar_vertices(display, position, size);

        Rectangle {
            position: position,
            size: size,
            color: color,
            vertices: rectangle_buffers.0,
            indices: rectangle_buffers.1,
            program: program
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters) {
        let uniforms = uniform!{
            fill_color: self.color
        };
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, params).unwrap();
    }

    fn grouped_draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, position: Vertex) {
        let uniforms = uniform!{
            fill_color: self.color,
            grouped_draw: true,
            group_position: position
        };
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, params).unwrap();
    }
}

pub struct Circle {
    position: Vertex,
    radius: f32,
    color: Color,
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::IndexBuffer<u8>,
    program: glium::Program
}

impl Circle {
    pub fn new(display: &glium::Display, position: Vertex, radius: f32, color: Color) -> Circle {
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform bool grouped_draw;
            uniform vec2 group_position;
            out vec4 pos;
            void main() {
                if(grouped_draw){
                    gl_Position = vec4(group_position + position, 0.0, 1.0);
                } else {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
                pos = gl_Position;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec4 pos;
            uniform vec2 center;
            uniform float radius;
            uniform bool grouped_draw;
            uniform vec2 group_position;
            uniform vec4 fill_color;
            out vec4 color;
            void main() {
                vec2 p;
                if(grouped_draw){
                    p = pos.xy - (group_position + center);
                } else {
                    p = pos.xy - center;
                }
                if(length(p) < radius) {
                    color = fill_color;
                } else {
                    color = vec4(0.0, 0.0, 0.0, 0.0);
                }
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let rectangle_buffers = create_rectanguar_vertices(display, position - Vertex::new(radius, radius), Vertex::new(radius * 2.0, radius * 2.0));

        Circle {
            position: position,
            radius: radius,
            color: color,
            vertices: rectangle_buffers.0,
            indices: rectangle_buffers.1,
            program: program
        }
    }
}

impl Shape for Circle {
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters) {
        let uniforms = uniform! {
            center: self.position.position,
            radius: self.radius,
            fill_color: self.color
        };
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, params).unwrap();
    }

    fn grouped_draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, position: Vertex) {
        let uniforms = uniform!{
            center: self.position.position,
            radius: self.radius,
            fill_color: self.color,
            grouped_draw: true,
            group_position: position
        };
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, params).unwrap();
    }
}