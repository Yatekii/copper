use std::sync::{
    Arc,
    RwLock,
};

use ncollide2d::partitioning::{
    DBVT,
    DBVTLeaf,
};
use ncollide2d::query::PointInterferencesCollector;

use uuid::Uuid;

use state::schema::*;
use state::event::{Listener, EventMessage};
use geometry::*;
use manipulation::library::Library;

pub struct SchemaViewer {
    schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    library: Arc<RwLock<Library>>,

    collision_world: RwLock<DBVT<f32, Uuid, AABB>>,
}

impl SchemaViewer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, library: Arc<RwLock<Library>>) -> SchemaViewer {
        SchemaViewer {
            schema: schema,
            view_state: view_state,
            library: library,
            collision_world: RwLock::new(DBVT::new()),
        }
    }

    pub fn get_currently_hovered_component_uuid(&self, cursor: Point2D) -> Option<Uuid> {
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
            view_state.update_hovered_component(Some(component_uuid), Some(schema.get_component_instance(component_uuid).reference.clone()))
        }
    }
}

impl Listener for SchemaViewer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(instance) => {
                // TODO: This is an ugly fix, remove ASAP
                let library = self.library.write().unwrap();
                let component = library.get_component(instance);
                let instance = instance.clone();

                // TODO: reenable
                //instance.set_component(component.clone());

                let aabb = instance.get_boundingbox().clone();
                let _ = self.collision_world.write().unwrap().insert(DBVTLeaf::new(aabb, instance.uuid));
            },
            EventMessage::ViewStateChanged => {
                self.update_currently_hovered_component();
            }
            _ => (),
        }
    }
}