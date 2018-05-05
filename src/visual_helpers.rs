use std::cell::RefCell;
use std::rc::Rc;


use gfx_glyph;


use resource_manager;


pub fn draw_coords_at_cursor(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, posx: f32, posy: f32, x: f32, y: f32, kx: f32, ky: f32) {
    
    // let font = {
    //     let rm = resource_manager.borrow_mut();
    //     rm.get_font(resource_manager::FontKey::new("test_data/Inconsolata-Regular.ttf"))
    // };

    // let content = format!("{:.2}, {:.2} \n {:.2}, {:.2}", x, y, kx, ky);
    // let section = gfx_glyph::Section {
    //     text: &content,
    //     screen_position: (posx, posy),
    //     scale: gfx_glyph::Scale::uniform(24.0),
    //     ..gfx_glyph::Section::default()
    // };

    // let mut f = font.borrow_mut();
    // f.queue(section);

    // let t = resource_manager.borrow().target.clone();
    // let r = resource_manager.borrow().depth_stencil.clone();
    // f.draw_queued(&mut resource_manager.borrow_mut().encoder, &t, &r).unwrap();
}