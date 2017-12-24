use glium;
use glium::Surface;


use drawing::Vertex;


fn createRectanguarVertices(display: &glium::Display, position: Vertex, size: Vertex) -> (glium::VertexBuffer<Vertex>, glium::index::IndexBuffer<u8>) {
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
}

pub struct Rectangle {
    position: Vertex,
    size: Vertex,
    scale: Vertex,
    translation: Vertex,
    rotation: Vertex,
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::IndexBuffer<u8>,
    program: glium::Program
}

impl Rectangle {
    pub fn new(display: &glium::Display, position: Vertex, size: Vertex) -> Rectangle {
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform mat4 matrix;
            out vec4 pos;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                pos = gl_Position;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec4 pos;
            out vec4 color;
            void main() {
                
                if(pos.x * pos.x + pos.y * pos.y < 0.25) {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                } else {
                    color = vec4(0.0, 0.0, 0.0, 0.0);
                }
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let rectangle_buffers = createRectanguarVertices(display, position, size);

        Rectangle {
            position: position,
            size: size,
            scale: size,
            translation: size,
            rotation: size,
            vertices: rectangle_buffers.0,
            indices: rectangle_buffers.1,
            program: program
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters) {
        target.draw(&self.vertices, &self.indices, &self.program, &glium::uniforms::EmptyUniforms, params).unwrap();
    }
}