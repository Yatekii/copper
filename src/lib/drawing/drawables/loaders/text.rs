use drawing::drawables;
use geometry;
use parsing::kicad::component_library::*;


pub fn load_text(
    position: &geometry::Point2,
    content: &String,
    dimension: f32,
    orientation: &TextOrientation,
    hjustify: Justify,
    vjustify: Justify
) -> drawables::TextDrawable {
    drawables::TextDrawable {
        position: position.clone(),
        content: content.clone(),
        dimension: dimension,
        orientation: orientation.clone(),
        hjustify: hjustify,
        vjustify: vjustify
    }
}