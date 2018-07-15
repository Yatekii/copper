use drawing;

use geometry::{
    AABB,
    Vector2D,
};

use parsing::schema_file::ComponentInstance;
pub use super::drawable_component::DrawableComponent;

pub struct DrawableComponentInstance {
    pub instance: ComponentInstance,
    pub drawable: DrawableComponent,
}

impl DrawableComponentInstance {
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        self.drawable.draw(buffers, &self.instance);
    }

    pub fn get_boundingbox(&self) -> AABB {
        let i = &self.instance;
        use utils::traits::Translatable;
        let bb = i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
        bb
    }
}