use euclid;
use glium_text_rusttype;
use glium;



use resource_manager::{ResourceManager, FontKey};
use drawing::Transform2D;


pub fn draw_coords_at_cursor(resource_manager: &ResourceManager, target: &mut glium::Frame, dimension: f32, x: f32, y: f32, kx: f32, ky: f32) {
    let font = resource_manager.get_font(FontKey {
        size: dimension as u32,
        path: "test_data/Inconsolata-Regular.ttf".into()
    });
    let content = format!("{}, {} \n {}, {}", x, y, kx, ky);
    let text = glium_text_rusttype::TextDisplay::new(&resource_manager.text_system, font, &content);

    let transform = euclid::TypedTransform3D::<f32, f32, f32>::create_scale(0.05, 0.05, 1.0)
                                                            .post_translate(euclid::TypedVector3D::new(
                                                                x,
                                                                y,
                                                                0.0
                                                            ));

    let _ = glium_text_rusttype::draw(
        &text,
        &resource_manager.text_system,
        target,
        transform.to_row_arrays(),
        (1.0, 0.0, 0.0, 1.0)
    );
}