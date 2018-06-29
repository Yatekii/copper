use drawing;
use drawing::drawables::Drawable;
use drawing::drawables::loaders::load_rectangle;
use drawing::color::Color;
use drawing::schema::DrawableComponentInstance;


// pub fn draw_coords_at_cursor(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, posx: f32, posy: f32, x: f32, y: f32, kx: f32, ky: f32) {
    
//     let font = {
//         let rm = resource_manager.borrow_mut();
//         rm.get_font(resource_manager::FontKey::new("test_data/Inconsolata-Regular.ttf"))
//     };

//     let content = format!("{:.2}, {:.2} \n {:.2}, {:.2}", x, y, kx, ky);
//     let section = gfx_glyph::Section {
//         text: &content,
//         screen_position: (posx, posy),
//         scale: gfx_glyph::Scale::uniform(24.0),
//         ..gfx_glyph::Section::default()
//     };

//     let mut f = font.borrow_mut();
//     f.queue(section);

//     let t = resource_manager.borrow().target.clone();
//     let r = resource_manager.borrow().depth_stencil.clone();
//     f.draw_queued(&mut resource_manager.borrow_mut().encoder, &t, &r).unwrap();
// }

pub fn draw_selection_indicator(
    buffers: &mut drawing::Buffers,
    currently_selected_component: &DrawableComponentInstance
) {
        let aabb = currently_selected_component.get_boundingbox().clone();
        //println!("BB: {:?}", aabb);
        let indicator_rect = load_rectangle(
            Color::new(1.0, 0.0, 0.0, 1.0),
            &aabb, true);
        indicator_rect.draw(buffers);
        // println!("Selected {:?}", aabb);
        // println!("Selected {:?}", currently_selected_component.instance.name);
}