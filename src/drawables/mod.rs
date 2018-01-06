mod shape_drawable;
mod group_drawable;
mod text_drawable;
pub mod loaders;


use std::cell::RefCell;
use std::rc::Rc;


use resource_manager;
use geometry;


pub use self::shape_drawable::ShapeDrawable;
pub use self::group_drawable::GroupDrawable;
pub use self::text_drawable::TextDrawable;


pub trait Drawable {
    fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: geometry::TSchemaScreen);
}