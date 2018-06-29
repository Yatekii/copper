use drawing::drawables;
use geometry;
use parsing::component;
use parsing::component::geometry as component_geometry;


pub fn load_text(
    position: &geometry::Point2D,
    content: &String,
    dimension: f32,
    orientation: &component_geometry::TextOrientation,
    hjustify: component::Justify,
    vjustify: component::Justify
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