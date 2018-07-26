use std::sync::{
    Arc,
    RwLock,
};

use ncollide2d::partitioning::{
    DBVT,
};
use ncollide2d::query::PointInterferencesCollector;

use uuid::Uuid;

use state::schema::*;
use state::event::{Listener, EventMessage};
use geometry::*;
use state::schema::component::Component;

pub struct ComponentViewer {
    collision_world: RwLock<DBVT<f32, Uuid, AABB>>,
}

impl ComponentViewer {
    pub fn new(_component: Arc<RwLock<Component>>, _view_state: Arc<RwLock<ViewState>>) -> ComponentViewer {
        ComponentViewer {
            collision_world: RwLock::new(DBVT::new()),
        }
    }

    pub fn get_currently_hovered_element_uuid(&self, cursor: Point2) -> Option<Uuid> {
        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(&cursor, &mut result);
            self.collision_world.read().unwrap().visit(&mut visitor);
        }
        result.first().map(|i| *i)
    }

    pub fn update_currently_hovered_element(&mut self) {
//        let schema = self.schema.write().unwrap();
//        let mut view_state = self.view_state.write().unwrap();
//        if let Some(component_uuid) = self.get_currently_hovered_element_uuid(view_state.get_cursor_in_schema_space()) {
//            view_state.update_hovered_component(Some(component_uuid), Some(component.get_element(component_uuid).reference.clone()))
//        }
    }
}

impl Listener for ComponentViewer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::OpenComponent(component) => {
                component.graphic_elements.iter().for_each(|_c| {
//                    let aabb = instance.get_boundingbox(c).clone();
//                    let _ = self.collision_world.write().unwrap().insert(DBVTLeaf::new(aabb, instance.uuid));
                });
            },
            EventMessage::ViewStateChanged => {
                self.update_currently_hovered_element();
            }
            _ => (),
        }
    }
}