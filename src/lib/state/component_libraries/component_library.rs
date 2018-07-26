use std::collections::HashMap;

use state::schema::component::Component;

pub struct ComponentLibrary {
    pub components: HashMap<String, Component>
}

impl ComponentLibrary {
    pub fn new(map: HashMap<String, Component>) -> Self{
        Self {
            components: map
        }
    }

    pub fn get_component_by_name(&self, name: &str) -> Option<&Component> {
        self.components.get(name)
    }

    pub fn get_components(&self) -> Vec<&Component> {
        self.components.values().collect()
    }
}