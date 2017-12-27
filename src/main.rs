extern crate lyon;

#[macro_use]
extern crate glium;

extern crate euclid;

mod drawing;

extern crate schema_parser;

use std::thread;
use std::time;


use std::fs;
use std::env;

use glium::Surface;
use glium::glutin::EventsLoop;


use lyon::extra::rust_logo::build_logo_path;
use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{FillTessellator, FillOptions};
use lyon::tessellation;

use lyon::tessellation::geometry_builder::SimpleBuffersBuilder;
use lyon::lyon_tessellation::basic_shapes::fill_rectangle;
use lyon::lyon_tessellation::geometry_builder::simple_builder;

// A very simple vertex constructor that only outputs the vertex position
struct VertexCtor;
impl VertexConstructor<tessellation::FillVertex, drawing::Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> drawing::Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());
        drawing::Vertex {
            position: vertex.position.to_array(),
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if(args.len() != 2){
        println!("Please specify a .lib file.");
    } else {
        let path = &args[1];
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(components) = schema_parser::parse_components(&mut file){
                run(components);
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("File could not be opened.");
        }
    }
}

fn run(components: Vec<schema_parser::component::Component>) {
    // Create a window
    let (w, h) = (640, 480);

    let mut eloop = EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(700, 700)
        .with_decorations(true)
        .with_title("Schema Renderer".to_string());

    let context = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &eloop).unwrap();


    let mut bounding_box = (
        schema_parser::component::geometry::Point { x: 0, y: 0 },
        schema_parser::component::geometry::Point { x: 0, y: 0 }
    );


    let component = &components[0];

    let mut rectangles = Vec::new();

     for shape in &component.graphic_elements {
        // println!("{:?}", shape);
        match shape {
            &schema_parser::component::geometry::GraphicElement::Rectangle { ref start, ref end, .. } => {
                let r = euclid::Rect::<f32>::from_points(
                    &[euclid::Point2D::<f32>::new(start.x as f32, start.y as f32),
                     euclid::Point2D::<f32>::new(end.x as f32, end.y as f32)]
                );

                rectangles.push(r);
            }
            _ => ()
        }
    }

    let mut mesh = VertexBuffers::new();

    let count = fill_rectangle(&rectangles[0], &mut BuffersBuilder::new(&mut mesh, VertexCtor));

    let vertex_buffer = glium::VertexBuffer::new(&display, &mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &mesh.indices,
    ).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();


    let mut running = true;

    while running {
        let mut target = display.draw();
        target.clear_color(0.8, 0.8, 0.8, 1.0);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan() / 2.0 / 10000.0;

            drawing::Mat4 {
                mat: [
                    [f *   aspect_ratio    ,    0.0, 0.0, 0.0],
                    [         0.0         ,     f , 0.0, 0.0],
                    [         0.0         ,    0.0, 1.0, 0.0],
                    [         0.0         ,    0.0, 0.0, 1.0],
                ]
            }
        };

        let uniforms  = uniform!{
            perspective: perspective
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        eloop.poll_events(|ev| {
            match ev {
                // The window was closed
                // We break the loop and let it go out of scope, which will close it finally
                glium::glutin::Event::WindowEvent { event,.. } => {
                    match event {
                        glium::glutin::WindowEvent::Closed => { running = false; },
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Q),
                                modifiers: glium::glutin::ModifiersState {
                                    ctrl: true,
                                    ..
                                },
                                ..
                            },
                            ..
                        } => { running = false; }
                        _ => ()
                    }
                },
                _ => ()
            }
            let m = time::Duration::from_millis(10);
            thread::sleep(m);
        });
    }
}

pub static VERTEX_SHADER: &'static str = r#"
    #version 140
    in vec2 position;
    uniform mat4 perspective;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        gl_Position = perspective*gl_Position;
    }
"#;

pub static FRAGMENT_SHADER: &'static str = r#"
    #version 140
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
