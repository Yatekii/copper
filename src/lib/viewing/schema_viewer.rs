use std::sync::{
    Arc,
    RwLock,
};
use std::collections::HashMap;

use ncollide2d::partitioning::{
    DBVT,
    DBVTLeaf,
    DBVTLeafId,
};
use ncollide2d::query::PointInterferencesCollector;
use ncollide2d::bounding_volume::BoundingVolumeInterferencesCollector;

use uuid::Uuid;

use state::schema::*;
use state::component_libraries::*;
use state::event::{Listener, EventMessage};
use geometry::*;
use utils::geometry::point_to_vector_2d;
use parsing::kicad::component_library::GraphicElement;
use drawing::drawables::loaders::pin::PIN_RADIUS;

#[derive(Clone, Debug)]
pub enum ElectricalConductor {
    Wire(Uuid),
    Pin(Uuid, usize),
}

pub struct SchemaViewer {
    schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    libraries: Arc<RwLock<ComponentLibraries>>,

    collision_world: RwLock<DBVT<f32, Uuid, AABB>>,
    wire_net: RwLock<DBVT<f32, ElectricalConductor, AABB>>,
    leaf_map: HashMap<Uuid, DBVTLeafId>,
    wire_leaf_map: HashMap<Uuid, DBVTLeafId>,
    pin_leaf_map: HashMap<Uuid, HashMap<usize, DBVTLeafId>>,
    selected_component: Option<Uuid>,
}

impl SchemaViewer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, libraries: Arc<RwLock<ComponentLibraries>>) -> SchemaViewer {
        SchemaViewer {
            schema: schema,
            view_state: view_state,
            libraries: libraries,
            collision_world: RwLock::new(DBVT::new()),
            wire_net: RwLock::new(DBVT::new()),
            leaf_map: HashMap::new(),
            wire_leaf_map: HashMap::new(),
            pin_leaf_map: HashMap::new(),
            selected_component: None,
        }
    }

    pub fn get_currently_hovered_component_uuid(&self, cursor: Point2) -> Option<Uuid> {
        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(&cursor, &mut result);
            self.collision_world.read().unwrap().visit(&mut visitor);
        }
        result.first().map(|i| *i)
    }

    pub fn get_component_uuids_in_rect(&self, aabb: &AABB) -> Vec<Uuid> {
        let mut result = Vec::new();
        {
            let mut visitor = BoundingVolumeInterferencesCollector::new(aabb, &mut result);
            self.collision_world.read().unwrap().visit(&mut visitor);
        }
        result
    }

    pub fn update_currently_hovered_component(&mut self) {
        let _schema = self.schema.write().unwrap();
        let mut view_state = self.view_state.write().unwrap();

        // In any case, forget the current item.
        view_state.hovered_items.clear();
        // If we actually hover an item, remember it
        if view_state.grabbed_items.is_empty() {
            if let Some(component_uuid) = self.get_currently_hovered_component_uuid(view_state.get_cursor_in_schema_space()) {
                view_state.hovered_items.insert(component_uuid);
                //schema.get_component_instance(&component_uuid).reference.clone()
            }
        }
    }

    /// Returns all electrical conductor `Uuid`s which correspond to an element in the schema.
    /// Returns an empty `Vec<T>` if no electrical conductor is found underneath the cursor.
    /// Can be used for snapping in wire drawing for example.
    pub fn get_currently_hovered_conductor_uuid(&self, cursor: &Point2) -> Vec<ElectricalConductor> {
        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(cursor, &mut result);
            self.wire_net.read().unwrap().visit(&mut visitor);
        }
        result
    }
}

impl Listener for SchemaViewer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(instance) => {
                // TODO: This is an ugly fix, remove ASAP
                // Retrieve component data from the lib.
                let libraries = self.libraries.write().unwrap();
                let component = libraries.get_component_by_name(&instance.name);

                if let Some(c) = component {
                    // Add component to the necessary BVT.
                    let aabb = instance.get_boundingbox(c).clone();
                    self.leaf_map.insert(instance.uuid, self.collision_world.write().unwrap().insert(DBVTLeaf::new(aabb, instance.uuid)));

                    // Add the pins of the component to the necessary BVT.
                    // Check for intersections with other pins or wires and remember those connections.
                    let mut pins = HashMap::new();
                    for potential_pin in c.graphic_elements.iter().enumerate() {
                        if let (i, GraphicElement::Pin{ uuid, position, .. }) = potential_pin {
                            let pos = instance.position + point_to_vector_2d(position);
                            let half_width = Vector2::new(PIN_RADIUS / 2.0, PIN_RADIUS / 2.0);
                            let aabb = AABB::new(pos - half_width, pos + half_width);
                            pins.insert(i,self.wire_net.write().unwrap().insert(DBVTLeaf::new(aabb, ElectricalConductor::Pin(uuid.clone(), i))));
                        }
                    }
                    self.pin_leaf_map.insert(
                        instance.uuid,
                        pins,
                    );
                }
            },
            EventMessage::UpdateComponent(instance) => {
                let libraries = self.libraries.write().unwrap();
                let component = libraries.get_component_by_name(&instance.name);

                let mut collision_world = self.collision_world.write().unwrap();

                // Update the pins of the component in the necessary BVT.
                // Check for intersections with other pins or wires and remember those connections.
                if let Some(c) = component {
                    // Remove the old collision data for the component in the BVT.
                    let leaf_id = self.leaf_map.get(&instance.uuid);
                    collision_world.remove(*leaf_id.unwrap());

                    // Update component in the necessary BVT.
                    let aabb = instance.get_boundingbox(c).clone();
                    self.leaf_map.insert(instance.uuid, collision_world.insert(DBVTLeaf::new(aabb, instance.uuid)));

                    // Remove the old collision data for the components pins in the BVT.
                    let pin_leaf_ids = self.pin_leaf_map.get(&instance.uuid);
                    for pin_leaf_id in pin_leaf_ids.unwrap().values() {
                        self.wire_net.write().unwrap().remove(*pin_leaf_id);
                    }

                    let mut pins = HashMap::new();
                    for potential_pin in c.graphic_elements.iter().enumerate() {
                        if let (i, GraphicElement::Pin{ uuid, position, .. }) = potential_pin {
                            let pos = instance.position + point_to_vector_2d(position);
                            let half_width = Vector2::new(PIN_RADIUS / 2.0, PIN_RADIUS / 2.0);
                            let aabb = AABB::new(pos - half_width, pos + half_width);
                            pins.insert(i,self.wire_net.write().unwrap().insert(DBVTLeaf::new(aabb, ElectricalConductor::Pin(uuid.clone(), i))));
                        }
                    }
                    self.pin_leaf_map.insert(
                        instance.uuid,
                        pins,
                    );
                }
            },
            EventMessage::AddWire(instance) => {
                // Add the pins of the component to the necessary BVT.
                // Check for intersections with other pins or wires and remember those connections.
                let half_width = Vector2::new(PIN_RADIUS / 2.0, PIN_RADIUS / 2.0);
                let aabb = AABB::new(instance.start - half_width, instance.end + half_width);
                self.wire_leaf_map.insert(
                    instance.uuid,
                    self.wire_net.write().unwrap().insert(DBVTLeaf::new(aabb, ElectricalConductor::Wire(instance.uuid.clone()))),
                );
            },
            EventMessage::UpdateWire(instance) => {
                let mut wire_net = self.wire_net.write().unwrap();

                // Remove the old collision data for the wire in the BVT.
                let leaf_id = self.wire_leaf_map.get(&instance.uuid);
                wire_net.remove(*leaf_id.unwrap());

                // Add the pins of the component to the necessary BVT.
                // Check for intersections with other pins or wires and remember those connections.
                let half_width = Vector2::new(PIN_RADIUS / 2.0, PIN_RADIUS / 2.0);
                let aabb = AABB::new(instance.start - half_width, instance.end + half_width);
                self.wire_leaf_map.insert(
                    instance.uuid,
                    wire_net.insert(DBVTLeaf::new(aabb, ElectricalConductor::Wire(instance.uuid.clone()))),
                );
            },
            EventMessage::SelectComponent(uuid) => self.selected_component = Some(uuid.clone()),
            EventMessage::ViewStateChanged => {
                self.update_currently_hovered_component();
            }
            _ => (),
        }
    }
}