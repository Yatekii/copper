use std::sync::{
    Arc,
    RwLock,
};

use uuid::Uuid;

use state::schema::*;
use drawing;

use state::event::{Listener, EventMessage};

pub use drawing::schema::{
    DrawableWire,
    DrawableComponent,
    DrawableComponentInstance,
};

use parsing::schema_file::ComponentInstance;
use geometry::schema_elements::WireSegment;

use manipulation::library::Library;

pub struct SchemaDrawer {
    schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    library: Arc<RwLock<Library>>,

    wires: Vec<DrawableWire>,
    components: RwLock<Vec<DrawableComponentInstance>>,
}

impl SchemaDrawer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, library: Arc<RwLock<Library>>) -> SchemaDrawer {
        SchemaDrawer {
            schema: schema,
            view_state: view_state,
            library: library,
            wires: Vec::new(),
            components: RwLock::from(Vec::new()),
        }
    }
    /// Issues draw calls to render the entire schema
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        for drawable in self.components.read().unwrap().iter() {
            // Unwrap should be ok as there has to be an instance for every component in the schema

            drawable.draw(buffers);
        }

        for wire in &self.wires {
            wire.draw(buffers);
        }
    }
}

impl SchemaActor for SchemaDrawer {
    fn component_added(&self, instance: &ComponentInstance) {
        let library = self.library.write().unwrap();
        let component = library.get_component(instance);
        let mut instance = instance.clone();

        instance.set_component(component.clone());
        let drawable_component = DrawableComponentInstance {
            instance: instance,
            drawable: DrawableComponent::new(self.components.read().unwrap().len() as u32, component.clone()),
        };
        self.components.write().unwrap().push(drawable_component);
    }

    fn component_updated(&self, instance: &ComponentInstance) {

    }

    fn wire_added(&mut self, instance: WireSegment) {
        let dw = DrawableWire::from_schema((self.components.read().unwrap().len() + self.wires.len()) as u32, &instance);
    }

    fn wire_updated(&mut self, instance: WireSegment) {
        
    }
}

impl Listener for SchemaDrawer {
    fn receive(&self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(component) => {
                self.component_added(component)
            },
            EventMessage::ChangeComponent(component) => self.component_updated(component),
        }
    }
}
