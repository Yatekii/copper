use std::collections::HashMap;
use std::fs;


use schema_parser;
use schema_parser::component;
use schema_parser::schema_file::ComponentInstance;


pub struct Library {
    pub components: HashMap<String, component::Component>
}

impl Library {
    pub fn new(path: &str) -> Option<Library> {
        let mut map = HashMap::new();
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(components) = schema_parser::parse_components(&mut file){
                for component in components.into_iter() {
                    map.insert(component.name.clone(), component);
                }
                Some(Library {
                    components: map
                })
            } else {
                println!("Could not parse the library file.");
                None
            }
        } else {
            println!("Lib file could not be opened.");
            None
        }
    }

    pub fn get_component(&self, component: &ComponentInstance) -> &component::Component {
        let item = self.components.get(&component.name);
        item.unwrap()
        // TODO: Return a placeholder component if no corresponding component was found in the lib.
    }
}