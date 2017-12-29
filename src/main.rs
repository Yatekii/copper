extern crate lyon;
#[macro_use]
extern crate glium;
extern crate glium_text_rusttype;
extern crate euclid;


extern crate schema_parser;


mod drawing;
mod resource_manager;
mod drawable_component;


use std::thread;
use std::time;
use std::fs;
use std::env;


use glium::Surface;
use glium::glutin::EventsLoop;

use resource_manager::{ResourceManager, FontKey};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
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

    let mut resource_manager = ResourceManager::new(&display);
    resource_manager.load_font(FontKey {
        size: 100,
        path: "/Users/yatekii/repos/schema_renderer/test_data/Inconsolata-Regular.ttf".into()
    });

    let rm_ref = &resource_manager;

    let mut view_state = drawing::ViewState::new(w, h);

    let mut current_component_index = 0;
    let mut current_component = drawable_component::DrawableComponent::new(rm_ref, components[current_component_index].clone());
                                                    
    view_state.update_from_box_pan(current_component.get_bounding_box());

    let mut running = true;

    while running {
        let mut target = display.draw();
        target.clear_color(0.8, 0.8, 0.8, 1.0);

        current_component.draw(&mut target, &view_state.current_perspective);

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
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Left),
                                state: glium::glutin::ElementState::Released,
                                ..
                            },
                            ..
                        } => {
                            if current_component_index > 0 {
                                current_component_index -= 1;
                                current_component = drawable_component::DrawableComponent::new(rm_ref, components[current_component_index].clone());

                                view_state.update_from_box_pan(current_component.get_bounding_box());
                            }
                        },
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Right),
                                state: glium::glutin::ElementState::Released,
                                ..
                            },
                            ..
                        } => {
                            if current_component_index < components.len() - 1 {
                                current_component_index += 1;
                                current_component = drawable_component::DrawableComponent::new(rm_ref, components[current_component_index].clone());

                                view_state.update_from_box_pan(current_component.get_bounding_box());
                            }
                        },
                        glium::glutin::WindowEvent::Resized(w, h) => {
                            view_state.update_from_resize(w, h);
                        },
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