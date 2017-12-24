use std::thread;
use std::time;


#[macro_use]
extern crate glium;


mod drawing;
use drawing::*;
mod shape;
use shape::*;


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

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let rectangle_a = Rectangle::new(&display, Vertex::new(-0.75, 0.25), Vertex::new(0.5, 0.5), Color::new(0.0, 1.0, 0.0, 1.0));
    let rectangle_b = Rectangle::new(&display, Vertex::new(0.25, -0.75), Vertex::new(0.5, 0.5), Color::new(0.0, 1.0, 0.0, 0.5));
    let circle_a = Circle::new(&display, Vertex::new(-0.5, -0.5), 0.25, Color::new(1.0, 0.0, 0.0, 1.0));
    let circle_b = Circle::new(&display, Vertex::new(0.5, 0.5), 0.25, Color::new(0.0, 1.0, 1.0, 1.0));

    // State of the window
    let mut closed = false;

    // Main event loop
    while !closed {
        // Draw new frame
        let mut target = display.draw();
        use glium::Surface;
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        rectangle_a.draw(&mut target, &params);
        rectangle_b.draw(&mut target, &params);
        circle_a.draw(&mut target, &params);
        circle_b.draw(&mut target, &params);
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