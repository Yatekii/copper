use std::thread;
use std::time;


#[macro_use]
extern crate glium;


mod drawing;
use drawing::Vertex;


fn main() {
    // Create the event loop
    let mut event_loop = glium::glutin::EventsLoop::new();

    // Create a window
    let window = glium::glutin::WindowBuilder::new()
                                                .with_dimensions(1024, 768)
                                                .with_title("Hello world");

    // OGL Context
    let context = glium::glutin::ContextBuilder::new();

    // Create the display with a window and register it with the event loop
    // Unwrap is ok since the application is pointless without a window
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    // Draw some stuff

    let vertex1 = Vertex { position: [ -0.5,  0.5] };
    let vertex2 = Vertex { position: [ -0.5, -0.5] };
    let vertex3 = Vertex { position: [  0.5,  0.5] };
    let vertex4 = Vertex { position: [  0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
        &[0u8,1,2,1,3,2]).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
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

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    // End draw

    // State of the window
    let mut closed = false;

    // Main event loop
    while !closed {
        // Draw new frame
        let mut target = display.draw();
        use glium::Surface;
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &params).unwrap();
        target.finish().unwrap();

        event_loop.poll_events(|event| {
            // println!("New event: {:#?}", event);
            match event {
                // The window was closed
                // We break the loop and let it go out of scope, which will close it finally
                glium::glutin::Event::WindowEvent { event,.. } => {
                    match event {
                        glium::glutin::WindowEvent::Closed => { closed = true; },
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
                        } => { closed = true; }
                        _ => ()
                    }
                },
                _ => ()
            };
            let m = time::Duration::from_millis(10);
            thread::sleep(m);
        });
    }
}