extern crate graphics;
extern crate glium_graphics;
extern crate piston;


extern crate schema_parser;


use std::fs;
use std::env;


use glium_graphics::{
    Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings
};
use piston::input::*;
use piston::event_loop::EventLoop;
use piston::window::WindowSettings;
use graphics::draw_state::Blend;


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
    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let ref mut window: GliumWindow =
        WindowSettings::new("Schema Renderer", [w, h])
                        .exit_on_esc(true)
                        .opengl(opengl)
                        .build()
                        .unwrap();

    // State of the window
    let mut mk = keyboard::ModifierKey::default();

    let mut bounding_box = (
        schema_parser::component::geometry::Point { x: 0, y: 0 },
        schema_parser::component::geometry::Point { x: 0, y: 0 }
    );

    use piston::window::Window;
    let w = window.size().width;
    let h = window.size().height;
    let mut vp = graphics::Viewport {
        rect: [0, 0, w as i32, h as i32],
        window_size: [w, h],
        draw_size: [w, h],
    };

    let mut g2d = Glium2d::new(opengl, window);
    window.set_lazy(true);
    while let Some(e) = window.next() {
        // Make sure we remember any modifier keys pressed
        println!("{:?}", e);
        mk.event(&e);
        if let Some(args) = e.render_args() {
            use graphics::*;
            let mut target = window.draw();
            g2d.draw(&mut target, vp, |c, g| {
                // println!("{:?}", c.transform);
                // c.trans(
                //     (args.viewport().rect[2]) as f64 / 2.0,
                //     (args.viewport().rect[3]) as f64 / 2.0
                // );
                // let transform = c.transform.trans(
                //     (args.viewport().rect[2]) as f64 / 2.0,
                //     (args.viewport().rect[3]) as f64 / 2.0
                // );

                // println!("Setting BB: ({:?},{:?})",
                //             (bounding_box.0.x + bounding_box.1.x) as f64 / 2.0,
                //             (bounding_box.0.y + bounding_box.1.y) as f64 / 2.0);

                clear([0.8, 0.8, 0.8, 1.0], g);
                g.clear_stencil(0);

                let draw_state = c.draw_state.blend(Blend::Alpha);

                Rectangle::new([0.0, 0.0, 0.0, 1.0]).draw([-50.0, -50.0, 100.0, 100.0], &draw_state, c.transform, g);

                let component = &components[0];
                for shape in &component.graphic_elements {
                    // println!("{:?}", shape);
                    match shape {
                        &schema_parser::component::geometry::GraphicElement::Rectangle { ref start, ref end, .. } => {
                            Rectangle::new([0.5, 1.0, 0.0, 0.3])
                                    .draw([
                                        start.x as f64,
                                        start.y as f64,
                                        (end.x - start.x) as f64,
                                        (end.y - start.y) as f64
                                    ], &draw_state, c.transform, g);
                        }
                        _ => ()
                    }
                }
            });

            target.finish().unwrap();
        }

        if let Some(Button::Keyboard(Key::Q)) = e.press_args() {
            if mk.contains(keyboard::ModifierKey::CTRL){
                use piston::window::Window;
                window.set_should_close(true);
            }
        }

        if let Some(Button::Keyboard(Key::Z)) = e.press_args() {
            let component = &components[0];
            bounding_box = component.get_boundingbox();
            println!("{:?}", bounding_box);
        }

        if let Some(Button::Keyboard(Key::Z)) = e.press_args() {
            let component = &components[0];
            bounding_box = component.get_boundingbox();
            println!("{:?}", bounding_box);
        }
    }
}