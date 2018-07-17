use std::sync::{
    Arc,
    RwLock,
};

use parsing::schema_file::ComponentInstance;
use geometry::schema_elements::WireSegment;

pub trait Listener {
    fn receive(&mut self, msg: &EventMessage);
}

#[derive(Debug)]
pub enum EventMessage {
    DrawSchema,
    ResizeDrawArea(u16, u16),
    AddComponent(ComponentInstance),
    ChangeComponent(ComponentInstance),
    AddWire(WireSegment),
    ViewStateChanged,
}

pub struct EventBus {
    bus: Arc<RwLock<EventBusInternal>>,
}

struct EventBusInternal {
    listeners: Vec<Arc<RwLock<Listener>>>,
}

impl EventBus {
    pub fn new() -> EventBus {
        let internal = Arc::new(RwLock::new(EventBusInternal::new()));

        EventBus {
            bus: internal
        }
    }

    pub fn get_handle(&self) -> EventBusHandle {

        return EventBusHandle::new(self.bus.clone());
    }
}

impl EventBusInternal {

    fn new() -> EventBusInternal {
        return EventBusInternal { listeners: Vec::new() }
    }

    fn add_listener(&mut self, listener: Arc<RwLock<dyn Listener>>) {
        self.listeners.push(listener);
    }

    fn send(&self, msg: &EventMessage) {
        for listener_ref in &self.listeners {
            listener_ref.write().unwrap().receive(msg);
        }
    }
}

pub struct EventBusHandle {
    bus: Arc<RwLock<EventBusInternal>>
}

impl EventBusHandle {
    fn new(bus: Arc<RwLock<EventBusInternal>>) -> EventBusHandle {
        return EventBusHandle {
            bus: bus,
        }
    }

    pub fn send(&self, msg: &EventMessage) {
        self.bus.read().unwrap().send(msg);
    }

    pub fn add_listener(self, listener: Arc<RwLock<dyn Listener>>) {
        self.bus.write().unwrap().add_listener(listener);
    }
}


