extern crate lyon;
#[macro_use]
extern crate glium;
extern crate glium_text_rusttype;
extern crate euclid;


extern crate schema_parser;


mod drawing;
mod resource_manager;
mod drawable_component;
mod visual_helpers;
mod library;
mod schema;


use std::thread;
use std::time;
use std::env;


use glium::Surface;
use glium::glutin::EventsLoop;


use resource_manager::{ResourceManager};


fn main() {
    // Create a window with an event loop
    let (w, h) = (700, 700);
    let mut eloop = EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
                                                //.with_vsync()
                                                .with_dimensions(w, h)
                                                .with_decorations(true)
                                                //.with_multisampling(16)
                                                .with_title("Schema Renderer".to_string());
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &eloop).unwrap();

    // Create a resource manager, which will hold fonts and other assets
    let resource_manager = ResourceManager::new(&display);
    let rm_ref = &resource_manager;

    // Load library and schema file
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify a .lib and a .sch file.");
        ::std::process::exit(1);
    }

    let library = library::Library::new(args[1].clone()).unwrap();

    let mut schema = schema::Schema::new(rm_ref);
    schema.load(&library, args[2].clone());

    let mut view_state = drawing::ViewState::new(w, h);

    let mut running = true;

    while running {
        let mut target = display.draw();
        target.clear_color(0.8, 0.8, 0.8, 1.0);

        schema.draw(&mut target, &view_state.current_perspective);

        let mut c = view_state.cursor.clone();
        c.x = (c.x / view_state.width as f32) * 2.0 - 1.0;
        
        c.y = -((c.y / view_state.height as f32) * 2.0 - 1.0);

        let kc = view_state.current_perspective.inverse().unwrap().transform_point(&c);
        visual_helpers::draw_coords_at_cursor(rm_ref, &mut target, 50.0, c.x, c.y, kc.x, kc.y);

        target.finish().unwrap();

        eloop.poll_events(|ev| {
            // println!("{:?}", ev);
            match ev {
                // The window was closed
                // We break the loop and let it go out of scope, which will close it finally
                glium::glutin::Event::WindowEvent { event,.. } => {
                    // println!("{:?}", event);
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
                        } => { running = false; },
                        glium::glutin::WindowEvent::Resized(w, h) => {
                            view_state.update_from_resize(w, h);
                        },
                        glium::glutin::WindowEvent::CursorMoved{position, ..} => {
                            view_state.cursor.x = position.0 as f32;
                            view_state.cursor.y = position.1 as f32;
                        },
                        // glium::glutin::WindowEvent::MouseInput{
                        //     state: glium::glutin::ElementState::Pressed,
                        //     button: glium::glutin::MouseButton::Left,
                        //     ..
                        // } => {
                        //     let mut c = view_state.cursor.clone();
                        //     c.x /= view_state.width as f32;
                        //     c.x *= 2.0;
                        //     c.x -= 1.0;
                            
                        //     c.y /= view_state.height as f32;
                        //     c.y *= 2.0;
                        //     c.y -= 1.0;

                        //     c.y *= -1.0;

                        //     println!("{:?} => {:?}", c, view_state.current_perspective.inverse().unwrap().transform_point(&c));
                        // },
                        _ => ()
                    }
                },
                _ => ()
            }
            // let m = time::Duration::from_millis(1);
            // thread::sleep(m);
        });
    }
}