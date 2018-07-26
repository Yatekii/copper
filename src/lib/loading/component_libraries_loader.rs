use std::sync::{
    Arc,
    RwLock,
};
use std::fs;
use std::collections::HashMap;

use parsing::kicad::parse_components_library;
use state::component_libraries::*;
use state::component_libraries::component_library::ComponentLibrary;

pub struct ComponentLibrariesLoader {
    libraries: Arc<RwLock<ComponentLibraries>>,
}

impl ComponentLibrariesLoader {
    pub fn new(libraries: Arc<RwLock<ComponentLibraries>>) -> Self {
        Self {
            libraries: libraries,
        }
    }

    /// Populates a schema from a library file pointed to by <path>.
    pub fn load_from_file(&mut self, path: &str) {
        let mut libraries = self.libraries.write().unwrap();
        let mut map = HashMap::new();

        if let Ok(mut file) = fs::File::open(path) {
            if let Some(components) = parse_components_library(&mut file){
                for component in components.into_iter() {
                    map.insert(component.name.clone(), component);
                }
                libraries.add_component_library(&path, ComponentLibrary::new(map));
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }
}