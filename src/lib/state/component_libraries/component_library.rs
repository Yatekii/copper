use std::collections::HashMap;

use parsing::component;

pub struct ComponentLibrary {
    pub components: HashMap<String, component::Component>
}

impl ComponentLibrary {
    pub fn new(map: HashMap<String, component::Component>) -> Self{
        Self {
            components: map
        }
    }

    pub fn get_component_by_name(&self, name: &str) -> Option<&component::Component> {
        self.components.get(name)
    }
}