use std::sync::{
    Arc,
    RwLock,
};

use uuid::Uuid;

use super::gfx_machinery::GfxMachinery;

use state::schema::*;
use state::event::{Listener, EventMessage};

use drawing::drawables::Drawable;
use drawing::drawables::component::ComponentDrawable;

pub struct ComponentDrawer {
    view_state: Arc<RwLock<ViewState>>,
    gfx_machinery: GfxMachinery,
}

impl ComponentDrawer {
    pub fn new(view_state: Arc<RwLock<ViewState>>) -> Self {
        Self {
            view_state: view_state,
            gfx_machinery: GfxMachinery::new(),
        }
    }

    fn get_drawable_mut(&mut self, uuid: &Uuid) -> Option<&mut dyn Drawable> {
        self.gfx_machinery.get_drawable_mut(uuid)
    }
}

impl Listener for ComponentDrawer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::OpenComponent(component) => {
                let component_drawable = Box::new(ComponentDrawable::new(0, component));
                self.gfx_machinery.clear_drawables();
                self.gfx_machinery.add_drawable(&component.uuid, component_drawable);
            },
            EventMessage::AddGeometricElement(_graphic_element) => (),
            EventMessage::DrawComponent => self.gfx_machinery.draw(&self.view_state.read().unwrap()),
            EventMessage::ResizeDrawArea(w, h) => {
                self.gfx_machinery.resize_target(*w, *h);
            },
            _ => (),
        }
    }
}