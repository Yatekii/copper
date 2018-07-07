use drawing;
use drawing::drawables::Drawable;
use drawing::drawables::loaders::load_rectangle;
use drawing::color::Color;
use drawing::schema::DrawableComponentInstance;

pub fn draw_selection_indicator(
    buffers: &mut drawing::Buffers,
    currently_selected_component: &DrawableComponentInstance
) {
        let aabb = currently_selected_component.get_boundingbox().clone();
        //println!("BB: {:?}", aabb);
        let indicator_rect = load_rectangle(
            0,
            Color::new(1.0, 0.0, 0.0, 1.0),
            &aabb, true);
        indicator_rect.draw(buffers);
        // println!("Selected {:?}", aabb);
        // println!("Selected {:?}", currently_selected_component.instance.name);
}