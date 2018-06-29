mod shape_drawable;
mod group_drawable;
mod text_drawable;
pub mod loaders;


use drawing;


pub use self::shape_drawable::ShapeDrawable;
pub use self::group_drawable::GroupDrawable;
pub use self::text_drawable::TextDrawable;


pub trait Drawable {
    fn draw(&self, buffers: &mut drawing::Buffers);
}