extern crate lyon;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate euclid;


extern crate schema_parser;


mod drawing;
mod resource_manager;
mod drawable_component;
mod visual_helpers;
mod library;
mod schema;


// use std::thread;
// use std::time;
use std::env;


use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::GlContext;


// use resource_manager::{ResourceManager};


const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];


fn main() {
    // Create a window with an event loop
    let (w, h) = (700, 700);
    let mut event_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
                                                //.with_vsync()
                                                .with_dimensions(w, h)
                                                .with_decorations(true)
                                                //.with_multisampling(16)
                                                .with_title("Schema Renderer".to_string());
    let api = glutin::Api::OpenGl;
    let version = (3, 2);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_vsync(true);

    let (window, mut device, mut factory, target, mut main_depth) = gfx_window_glutin::init::<drawing::ColorFormat, drawing::DepthFormat>(window_builder, context, &event_loop);
    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    // Create a resource manager, which will hold fonts and other assets
    let resource_manager = resource_manager::ResourceManager::new(&mut factory, &target);
    let rm_ref = &resource_manager;

    // Load library and schema file
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify a .lib and a .sch file.");
        ::std::process::exit(1);
    }

    let library = library::Library::new(args[1].clone()).unwrap();

    // let mut schema = schema::Schema::new(rm_ref);
    //schema.load(&library, args[2].clone());
    let mut view_state = drawing::ViewState::new(w, h);

    // let bb = schema.get_bounding_box();
    // view_state.update_from_box_pan(&bb);

    let mut data = drawing::pipe::Data {
        vbuf: (),
        out: target
    };

    let mut running = true;

    while running {
        // Start a new frame
        // Color it uniformly to start off
        // let mut target = display.draw();
        encoder.clear(&data.out, CLEAR_COLOR);

        // TODO: draw
        // schema.draw(&mut target, &view_state.current_perspective);

        // TODO: Draw cursor
        // let mut c = view_state.cursor.clone();
        // c.x =  (c.x / view_state.width  as f32) * 2.0 - 1.0;
        // c.y = -(c.y / view_state.height as f32) * 2.0 - 1.0;

        // let kc = view_state.current_perspective.inverse().unwrap().transform_point(&c);
        // visual_helpers::draw_coords_at_cursor(rm_ref, &mut target, 50.0, c.x, c.y, kc.x, kc.y);

        // Finish up the current frame
        encoder.flush(&mut device);
        use glutin::GlContext;
        window.swap_buffers().unwrap();
        device.cleanup();

        event_loop.poll_events(|ev| {
            // println!("{:?}", ev);
            match ev {
                // The window was closed
                // We break the loop and let it go out of scope, which will close it finally
                glutin::Event::WindowEvent { event,.. } => {
                    // println!("{:?}", event);
                    match event {
                        glutin::WindowEvent::Closed => { running = false; },
                        glutin::WindowEvent::KeyboardInput {
                            input: glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Q),
                                modifiers: glutin::ModifiersState {
                                    ctrl: true,
                                    ..
                                },
                                ..
                            },
                            ..
                        } => { running = false; },
                        glutin::WindowEvent::Resized(w, h) => {
                            view_state.update_from_resize(w, h);
                            // let bb = schema.get_bounding_box();
                            // view_state.update_from_box_pan(&bb);
                        },
                        glutin::WindowEvent::CursorMoved{position, ..} => {
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