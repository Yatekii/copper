pub mod view_state;

pub use self::view_state::ViewState;

use std::f32::consts::PI;

use uuid::Uuid;
use ::state::event::{EventMessage, EventBusHandle};

use parsing::schema_file::ComponentInstance;

use geometry::*;

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    components: Vec<ComponentInstance>,
    wires: Vec<schema_elements::WireSegment>,
    event_bus: EventBusHandle,
}

impl Schema {
    /// Creates a new blank schema
    pub fn new(event_bus: EventBusHandle) -> Schema {
        Schema {
            wires: Vec::new(),
            components: Vec::new(),
            event_bus: event_bus,
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self) -> AABB {
        let mut aabb = AABB::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 0.0)
        );
        self.components.iter().for_each(|c| {
            use utils::traits::Translatable;
            let bb = &c.get_boundingbox().translated(Vector2D::new(c.position.x, c.position.y));
            use ncollide2d::bounding_volume::BoundingVolume;
            aabb.merge(bb);
        });
        aabb
    }

    pub fn get_component_instance(&self, component_uuid: Uuid) -> &ComponentInstance {
        self.components.iter().find(|c| c.uuid == component_uuid).unwrap()
    }

    pub fn get_component_instance_mut(&mut self, component_uuid: Uuid) -> &mut ComponentInstance {
        self.components.iter_mut().find(|c| c.uuid == component_uuid).unwrap()
    }

    pub fn rotate_component(&mut self, component_uuid: Uuid) {
        self.get_component_instance_mut(component_uuid).rotation *= Matrix4::from_axis_angle(
            &Vector3::z_axis(),
            PI / 2.0
        );
        // TODO: emit changed event
    }

    pub fn add_component(&mut self, mut instance: ComponentInstance) {
        instance.uuid = Uuid::new_v4();
        self.components.push(instance.clone());
        self.event_bus.send(&EventMessage::AddComponent(instance));
    }

    pub fn add_wire(&mut self, mut instance: schema_elements::WireSegment) {
        instance.uuid = Uuid::new_v4();
        self.wires.push(instance.clone());
        self.event_bus.send(&EventMessage::AddWire(instance));
    }
}