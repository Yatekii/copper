use drawables;
use schema_parser::geometry;
use schema_parser::component;
use schema_parser::component::geometry as component_geometry;


pub fn load_text(
    position: &geometry::SchemaPoint2D,
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