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

use parsing::schema_file::ComponentInstance;

use state::schema::*;
use geometry::*;

pub struct SchemaViewer {
    schema: Arc<RwLock<Schema>>,
    
    view_state: Arc<RwLock<ViewState>>,
    collision_world: RwLock<DBVT<f32, Uuid, AABB>>,
}

impl SchemaViewer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>) -> SchemaViewer {
        SchemaViewer {
            schema: schema,
            view_state: view_state,
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
        let mut schema = self.schema.write().unwrap();
        let mut view_state = self.view_state.write().unwrap();
        if let Some(component_uuid) = self.get_currently_hovered_component_uuid(view_state.get_cursor_in_schema_space()) {
            view_state.update_hovered_component(Some(component_uuid), Some(schema.get_component_instance(component_uuid).reference.clone()))
        }
    }
}

impl SchemaActor for SchemaViewer {
    fn component_added(&self, instance: &ComponentInstance) {
        let aabb = instance.get_boundingbox().clone();
        let _ = self.collision_world.write().unwrap().insert(DBVTLeaf::new(aabb, instance.uuid));
    }

    fn component_updated(&self, instance: &ComponentInstance) {

    }

    fn wire_added(&mut self, instance: schema_elements::WireSegment) {

    }
    fn wire_updated(&mut self, instance: schema_elements::WireSegment) {

    }
}