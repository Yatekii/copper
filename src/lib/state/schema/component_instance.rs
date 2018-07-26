use uuid::Uuid;
use std::cell::Cell;

use geometry::{
    Point2,
    Vector2,
    Vector3,
    Matrix4,
    AABB
};
use state::schema::component::Component;

use utils::traits::clone_cached_aabb;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ComponentInstance {
    pub uuid: Uuid,
    pub name: String,
    pub reference: String,
    pub position: Point2,
    pub rotation: Matrix4,
    #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
    pub bounding_box: Cell<Option<AABB>>
}

impl ComponentInstance {
    pub fn new(name: String) -> ComponentInstance {
        ComponentInstance {
            uuid: Uuid::nil(),
            name: name,
            reference: "?".into(),
            position: Point2::origin(),
            rotation: Matrix4::identity(),
            bounding_box: Cell::new(None)
        }
    }

    pub fn get_boundingbox(&self, component: &Component) -> AABB {
        use utils::traits::Translatable;
        component.get_boundingbox().translated(Vector2::new(
            self.position.x,
            self.position.y
        ))
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.rotation.append_translation(
            &Vector3::new(
                self.position.x,
                self.position.y,
                0.0
            )
        )
    }
}