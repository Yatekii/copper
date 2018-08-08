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

use uuid::Uuid;

use state::schema::*;
use state::component_libraries::*;
use state::event::{Listener, EventMessage};
use geometry::*;

pub struct SchemaViewer {
    schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    libraries: Arc<RwLock<ComponentLibraries>>,

    collision_world: RwLock<DBVT<f32, Uuid, AABB>>,
    leaf_map: HashMap<Uuid, DBVTLeafId>,
    selected_component: Option<Uuid>,
}

impl SchemaViewer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, libraries: Arc<RwLock<ComponentLibraries>>) -> SchemaViewer {
        SchemaViewer {
            schema: schema,
            view_state: view_state,
            libraries: libraries,
            collision_world: RwLock::new(DBVT::new()),
            leaf_map: HashMap::new(),
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

    pub fn update_currently_hovered_component(&mut self) {
        let schema = self.schema.write().unwrap();
        let mut view_state = self.view_state.write().unwrap();
        if let Some(component_uuid) = self.get_currently_hovered_component_uuid(view_state.get_cursor_in_schema_space()) {
            view_state.update_hovered_component(Some(component_uuid), Some(schema.get_component_instance(&component_uuid).reference.clone()))
        } else {
            view_state.update_hovered_component(None, None)
        }
    }

    pub fn get_selected_component(&mut self) {

    }
}

impl Listener for SchemaViewer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(instance) => {
                // TODO: This is an ugly fix, remove ASAP
                let libraries = self.libraries.write().unwrap();
                let component = libraries.get_component_by_name(&instance.name);

                if let Some(c) = component {
                    let aabb = instance.get_boundingbox(c).clone();
                    self.leaf_map.insert(instance.uuid, self.collision_world.write().unwrap().insert(DBVTLeaf::new(aabb, instance.uuid)));
                }
            },
            EventMessage::UpdateComponent(instance) => {
                let libraries = self.libraries.write().unwrap();
                let component = libraries.get_component_by_name(&instance.name);

                let mut collision_world = self.collision_world.write().unwrap();

                let leaf_id = self.leaf_map.get(&instance.uuid);
                collision_world.remove(*leaf_id.unwrap());

                if let Some(c) = component {
                    let aabb = instance.get_boundingbox(c).clone();
                    self.leaf_map.insert(instance.uuid, collision_world.insert(DBVTLeaf::new(aabb, instance.uuid)));
                }
            },
            EventMessage::SelectComponent(uuid) => self.selected_component = Some(uuid.clone()),
            EventMessage::ViewStateChanged => {
                self.update_currently_hovered_component();
            }
            _ => (),
        }
    }
}