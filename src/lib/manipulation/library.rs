use std::collections::HashMap;
use std::fs;
use std::rc::{
    Rc,
    Weak
};

use parsing::component;
use parsing::schema_file::ComponentInstance;

pub struct Library {
    pub components: HashMap<String, Rc<component::Component>>
}

impl Library {
    pub fn new(path: &str) -> Option<Library> {
        let mut map = HashMap::new();
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(components) = ::parse_components(&mut file){
                for component in components.into_iter() {
                    map.insert(component.name.clone(), Rc::new(component));
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

    pub fn get_component(&self, component: &ComponentInstance) -> Weak<component::Component> {
        let item = self.components.get(&component.name).map(|v| Rc::downgrade(v));
        // TODO: Return a placeholder component if no corresponding component was found in the lib.
        item.unwrap()
    }
}