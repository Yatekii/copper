use std::sync::{
    Arc,
    RwLock,
};

use uuid::Uuid;

use state::schema::*;
use drawing;

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
    components: Vec<DrawableComponentInstance>,
}

impl SchemaDrawer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, library: Arc<RwLock<Library>>) -> SchemaDrawer {
        SchemaDrawer {
            schema: schema,
            view_state: view_state,
            library: library,
            wires: Vec::new(),
            components: Vec::new(),
        }
    }
    /// Issues draw calls to render the entire schema
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        for drawable in &self.components {
            // Unwrap should be ok as there has to be an instance for every component in the schema

            drawable.draw(buffers);
        }

        for wire in &self.wires {
            wire.draw(buffers);
        }
    }
}

impl SchemaActor for SchemaDrawer {
    fn component_added(&mut self, mut instance: ComponentInstance) {
        let library = self.library.write().unwrap();
        let component = library.get_component(&instance);
        instance.set_component(component.clone());
        let drawable_component = DrawableComponentInstance {
            instance: instance.clone(),
            drawable: DrawableComponent::new(self.components.len() as u32, component.clone()),
        };
        self.components.push(drawable_component);
    }

    fn component_updated(&mut self, instance: ComponentInstance) {

    }

    fn wire_added(&mut self, instance: WireSegment) {
        let dw = DrawableWire::from_schema((self.components.len() + self.wires.len()) as u32, &instance);
    }

    fn wire_updated(&mut self, instance: WireSegment) {
        
    }
}