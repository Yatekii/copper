pub mod view_state;

pub use self::view_state::ViewState;

use std::f32::consts::PI;

use uuid::Uuid;
use ::state::event::{Listener, EventMessage, EventBusHandle};

use parsing::schema_file::ComponentInstance;

use geometry::*;

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    components: Vec<ComponentInstance>,
    wires: Vec<schema_elements::WireSegment>,
    bounding_box: Option<AABB>,
    event_bus: EventBusHandle,
}

pub trait SchemaActor {
    fn component_added(&self, instance: &ComponentInstance);
    fn component_updated(&self, instance: &ComponentInstance);
    fn wire_added(&mut self, instance: schema_elements::WireSegment);
    fn wire_updated(&mut self, instance: schema_elements::WireSegment);
}

impl Schema {
    /// Creates a new blank schema
    pub fn new(event_bus: EventBusHandle) -> Schema {
        Schema {
            wires: Vec::new(),
            components: Vec::new(),
            bounding_box: None,
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
        println!("Added comp!");
    }

    pub fn add_wire(&mut self, instance: schema_elements::WireSegment) {
        self.wires.push(instance)
        // TODO: emit add event
    }
}

// impl<T: SchemaActor> Listener for T {
//     fn receive(&self, msg: &EventMessage) {
//         match msg {
//             EventMessage::AddComponent(component) => {
//                 self.component_added(component)
//             },
//             EventMessage::ChangeComponent(component) => self.component_updated(component),
//         }
//     }
// }