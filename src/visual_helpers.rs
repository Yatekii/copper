use std::cell::RefCell;
use std::rc::Rc;


use euclid;
use gfx_device_gl;
use gfx_glyph;


use resource_manager;


type Resources = gfx_device_gl::Resources;


pub fn draw_coords_at_cursor(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, posx: f32, posy: f32, x: f32, y: f32, kx: f32, ky: f32) {
    
    let font = {
        let rm = resource_manager.borrow_mut();
        rm.get_font(resource_manager::FontKey::new("test_data/Inconsolata-Regular.ttf"))
    };

    let content = format!("{:.2}, {:.2} \n {:.2}, {:.2}", x, y, kx, ky);
    let section = gfx_glyph::Section {
        text: &content,
        screen_position: (posx, posy),
        scale: gfx_glyph::Scale::uniform(24.0),
        ..gfx_glyph::Section::default()
    };

    let mut f = font.borrow_mut();
    f.queue(section);

    let t = resource_manager.borrow().target.clone();
    let r = resource_manager.borrow().depth_stencil.clone();
    f.draw_queued(&mut resource_manager.borrow_mut().encoder, &t, &r).unwrap();

    let transform = euclid::TypedTransform3D::<f32, f32, f32>::create_scale(0.05, 0.05, 1.0)
        .post_translate(euclid::TypedVector3D::new(x, y, 0.0));
}