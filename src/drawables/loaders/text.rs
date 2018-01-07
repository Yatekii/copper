use std::cell::RefCell;
use std::rc::Rc;


use drawables;
use schema_parser::geometry;
use schema_parser::component;
use schema_parser::component::geometry as component_geometry;
use resource_manager;


pub fn load_text(
    _resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
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