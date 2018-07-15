use uuid::Uuid;
use std::sync::{Arc, Weak, RwLock};

use parsing::schema_file::ComponentInstance;

pub trait Listener {
    fn receive(&self, msg: &EventMessage);
}

pub enum EventMessage {
    AddComponent(ComponentInstance),
    ChangeComponent(ComponentInstance)
}

pub struct EventBus {
    bus: Arc<RwLock<EventBusInternal>>,
}

struct EventBusInternal {
    listeners: Vec<Weak<Listener>>,
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

    fn add_listener(&mut self, listener: Weak<dyn Listener>) {
        self.listeners.push(listener);
    }

    fn send(&self, msg: &EventMessage) {
        for listener_ref in &self.listeners {

            if let Some(listener) = listener_ref.upgrade() {
                listener.receive(msg);
            }
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

    pub fn add_listener(self, listener: Weak<dyn Listener>) {
        self.bus.write().unwrap().add_listener(listener);
    }
}


