#![feature(proc_macro)]

extern crate lyon;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate glutin;

extern crate gtk;
extern crate gdk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

extern crate epoxy;
extern crate shared_library;

extern crate gfx_core;
extern crate gfx_gl;


extern crate ncollide2d;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate copper;

mod main_window;


// use std::thread;
// use std::time;
use std::env;
use std::ptr;
use shared_library::dynamic_library::DynamicLibrary;

use main_window::Win;

fn main() {
    // Load libepoxy
    epoxy::load_with(|s| {
        unsafe {
            match DynamicLibrary::open(None).unwrap().symbol(s) {
                Ok(v) => v,
                Err(e) => { /* println!("{}: {}", s, e); */ ptr::null() },
            }
        }
    });

    // Run the GTK application
    use relm::Widget;
    Win::run(()).unwrap();

    let _ = env_logger::init();

    // while running {
    //     event_loop.poll_events(|ev| {
    //         // println!("{:?}", ev);
    //         match ev {
    //             // The window was closed
    //             // We break the loop and let it go out of scope, which will close it finally
    //             glutin::Event::WindowEvent { event,.. } => {
    //                 // println!("{:?}", event);
    //                 match event {
    //                     glutin::WindowEvent::Closed => { running = false; },
    //                     glutin::WindowEvent::KeyboardInput {
    //                         input: glutin::KeyboardInput {
    //                             virtual_keycode: Some(glutin::VirtualKeyCode::Q),
    //                             modifiers: glutin::ModifiersState {
    //                                 ctrl: true,
    //                                 ..
    //                             },
    //                             ..
    //                         },
    //                         ..
    //                     } => { running = false; },
    //                     glutin::WindowEvent::Resized(w, h) => {
    //                         println!("Window resized to width={}, height={}", w, h);

    //                         // We must manually update the inner size of the window
    //                         window.set_inner_size(w, h);

    //                         // We also must manually resize the GL context, this 
    //                         window.resize(w, h);

    //                         view_state.update_from_resize(w, h);
    //                         let bb = schema.get_bounding_box();
    //                         view_state.update_from_box_pan(bb);
    //                         let target = &mut resource_manager.borrow_mut().target.clone();
    //                         let depth_stencil = &mut resource_manager.borrow_mut().depth_stencil.clone();
    //                         gfx_window_glutin::update_views(&window, target, depth_stencil);

    //                         resource_manager.borrow_mut().target = target.clone();
    //                         resource_manager.borrow_mut().depth_stencil = depth_stencil.clone();
    //                     },
    //                     glutin::WindowEvent::CursorMoved{position, ..} => {
    //                         let dx = position.0 as f32 - view_state.cursor.x;
    //                         let dy = position.1 as f32 - view_state.cursor.y;
                            
    //                         view_state.cursor.x = position.0 as f32;
    //                         view_state.cursor.y = position.1 as f32;

    //                         if view_state.mouse_state.middle {
    //                             view_state.center += schema_parser::geometry::Vector2D::new(dx as f32 * 10.0, -dy as f32 * 10.0);
    //                             view_state.update_perspective();
    //                         }
    //                     },
    //                     glutin::WindowEvent::MouseInput { button, state, .. } => {
    //                         if state == glutin::ElementState::Pressed {
    //                             match button {
    //                                 glutin::MouseButton::Left => view_state.mouse_state.left = true,
    //                                 glutin::MouseButton::Middle => view_state.mouse_state.middle = true,
    //                                 glutin::MouseButton::Right => view_state.mouse_state.right = true,
    //                                 _ => {}
    //                             }
    //                         } else {
    //                             match button {
    //                                 glutin::MouseButton::Left => view_state.mouse_state.left = false,
    //                                 glutin::MouseButton::Middle => view_state.mouse_state.middle = false,
    //                                 glutin::MouseButton::Right => view_state.mouse_state.right = false,
    //                                 _ => {}
    //                             }
    //                         }
    //                     }
    //                     glutin::WindowEvent::MouseWheel{delta, ..} => {
    //                         if let glutin::MouseScrollDelta::PixelDelta(_x, y) = delta {
    //                             view_state.update_from_zoom(y);
    //                         }
    //                         if let glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
    //                             view_state.update_from_zoom(y);
    //                         }
    //                     },
    //                     _ => ()
    //                 }
    //             },
    //             _ => ()
    //         }
    //         // let m = time::Duration::from_millis(1);
    //         // thread::sleep(m);
    //     });
    // }
}
