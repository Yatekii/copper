pub mod component_library;

use std::collections::HashMap;

use ::state::event::{EventBusHandle};

use parsing::component;

use state::component_libraries::component_library::ComponentLibrary;

/// Represents a schema containing all its components and necessary resource references
pub struct ComponentLibraries {
    libraries: HashMap<String, ComponentLibrary>
}

impl ComponentLibraries {
    /// Creates a new blank schema
    pub fn new(_event_bus: EventBusHandle) -> Self {
        Self {
            libraries: HashMap::new()
        }
    }

    pub fn get_component_library(&mut self, name: &str) -> Option<&mut ComponentLibrary> {
        self.libraries.get_mut(name)
    }

    pub fn add_component_library(&mut self, name: &str, library: ComponentLibrary) {
        self.libraries.insert(name.to_owned(), library);
    }

    pub fn get_component_by_name(&self, name: &str) -> Option<&component::Component> {
        for lib in self.libraries.values() {
            let component = lib.get_component_by_name(name);
            if component.is_some() {
                return component;
            }
        }
        None
    }

    pub fn get_component_by_name_and_lib(&self, component_name: &str, library_name: &str) -> Option<&component::Component> {
        if let Some(lib) = self.libraries.get(library_name) {
            let component = lib.get_component_by_name(component_name);
            if component.is_some() {
                return component;
            }
        }
        None
    }

    pub fn get_components_from_lib(&self, library_name: &str) -> Vec<&component::Component> {
        if let Some(lib) = self.libraries.get(library_name) {
            return lib.get_components()
        }
        Vec::new()
    }

    pub fn get_components_from_lib_yolo(&self) -> Vec<&component::Component> {
        self.libraries.values().collect::<Vec<&ComponentLibrary>>()[0].get_components()
    }

    pub fn get_libraries(&self) -> Vec<String> {
        self.libraries.keys().map(|k| k.to_owned()).collect::<Vec<String>>()
    }
}