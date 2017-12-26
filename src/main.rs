extern crate graphics;
extern crate glium_graphics;
extern crate piston;

use glium_graphics::{
    Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings
};
use piston::input::*;
use piston::event_loop::EventLoop;
use piston::window::WindowSettings;
use graphics::draw_state::Blend;


fn main() {
    // Create the event loop
    // let mut event_loop = glium::glutin::EventsLoop::new();

    // Create a window
    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let ref mut window: GliumWindow =
        WindowSettings::new("Schema Renderer", [w, h])
                        .exit_on_esc(true)
                        .opengl(opengl)
                        .build()
                        .unwrap();

    let mut blend = Blend::Alpha;
    let mut clip_inside = true;
    // let rust_logo = Texture::from_path(window, "assets/rust.png",
    //     Flip::None, &TextureSettings::new()).unwrap();

    // let shapes = vec![
    //     Box::new(Rectangle::new(&display, Vertex::new(-0.75, 0.25), Vertex::new(0.5, 0.5), Color::new(0.0, 1.0, 0.0, 1.0))) as Box<shape::Shape>,
    //     Box::new(Rectangle::new(&display, Vertex::new(0.25, -0.75), Vertex::new(0.5, 0.5), Color::new(0.0, 1.0, 0.0, 0.5))) as Box<shape::Shape>
    // ];
    // let mut g = Group::new(Vertex::new(0.25, 0.25), shapes);
    // g.add_shape(Box::new(Circle::new(&display, Vertex::new(-0.5, -0.5), 0.25, Color::new(1.0, 0.0, 0.0, 1.0))));
    // g.add_shape(Box::new(Circle::new(&display, Vertex::new(0.5, 0.5), 0.25, Color::new(0.0, 1.0, 1.0, 1.0))));

    // State of the window
    let mut closed = false;

    let mut g2d = Glium2d::new(opengl, window);
    window.set_lazy(true);
    while let Some(e) = window.next() {
        println!("{:?}", e);
        if let Some(args) = e.render_args() {
            use graphics::*;

            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                g.clear_stencil(0);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);

                let draw_state = c.draw_state.blend(blend);
                Rectangle::new([0.5, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

                let transform = c.transform.trans(100.0, 100.0);
                // Compute clip rectangle from upper left corner.
                let (clip_x, clip_y, clip_w, clip_h) = (100, 100, 100, 100);
                let (clip_x, clip_y, clip_w, clip_h) =
                    (clip_x, c.viewport.unwrap().draw_size[1] - clip_y - clip_h, clip_w, clip_h);
                let clipped = c.draw_state.scissor([clip_x, clip_y, clip_w, clip_h]);
                // Image::new().draw(&rust_logo, &clipped, transform, g);

                let transform = c.transform.trans(200.0, 200.0);
                Ellipse::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 50.0, 50.0], &DrawState::new_clip(), transform, g);
                // Image::new().draw(&rust_logo,
                //     &(if clip_inside { DrawState::new_inside() }
                //         else { DrawState::new_outside() }),
                //     transform, g);
            });

            target.finish().unwrap();
        }

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            blend = match blend {
                Blend::Alpha => Blend::Add,
                Blend::Add => Blend::Multiply,
                Blend::Multiply => Blend::Invert,
                Blend::Invert => Blend::Alpha,
            };
            println!("Changed blending to {:?}", blend);
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            clip_inside = !clip_inside;
            if clip_inside {
                println!("Changed to clip inside");
            } else {
                println!("Changed to clip outside");
            }
        }
    }

    // Main event loop
    // while !closed {
    //     // Draw new frame
    //     let mut target = display.draw();
    //     use glium::Surface;
    //     target.clear_color(0.0, 0.0, 1.0, 1.0);
    //     let (width, height) = target.get_dimensions();
    //     let pixel_size = 1 / width;
    //     let perspective = {
    //         let aspect_ratio = height as f32 / width as f32;

    //         let fov: f32 = 3.141592 / 3.0;
    //         let zfar = 1024.0;
    //         let znear = 0.1;

    //         let f = 1.0 / (fov / 2.0).tan() / 2.0;

    //         Mat4 {
    //             mat: [
    //                 [f *   aspect_ratio    ,    0.0, 0.0, 0.0],
    //                 [         0.0         ,     f , 0.0, 0.0],
    //                 [         0.0         ,    0.0, 1.0, 0.0],
    //                 [         0.0         ,    0.0, 0.0, 1.0],
    //             ]
    //         }
    //     };
    //     g.draw(&mut target, &params, &perspective);
    //     target.finish().unwrap();
    //     event_loop.poll_events(|event| {
    //         // println!("New event: {:#?}", event);
    //         match event {
    //             // The window was closed
    //             // We break the loop and let it go out of scope, which will close it finally
    //             glium::glutin::Event::WindowEvent { event,.. } => {
    //                 match event {
    //                     glium::glutin::WindowEvent::Closed => { closed = true; },
    //                     glium::glutin::WindowEvent::KeyboardInput {
    //                         input: glium::glutin::KeyboardInput {
    //                             virtual_keycode: Some(glium::glutin::VirtualKeyCode::Q),
    //                             modifiers: glium::glutin::ModifiersState {
    //                                 ctrl: true,
    //                                 ..
    //                             },
    //                             ..
    //                         },
    //                         ..
    //                     } => { closed = true; }
    //                     _ => ()
    //                 }
    //             },
    //             _ => ()
    //         };
    //         let m = time::Duration::from_millis(10);
    //         thread::sleep(m);
    //     });
    // }
}