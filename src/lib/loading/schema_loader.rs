use std::sync::{
    Arc,
    RwLock,
};
use std::fs;

use state::schema::*;
use parsing::kicad::schema::*;

pub struct SchemaLoader {
    schema: Arc<RwLock<Schema>>,
}

impl SchemaLoader {
    pub fn new(schema: Arc<RwLock<Schema>>) -> SchemaLoader {
        SchemaLoader {
            schema: schema,
        }
    }

    /// Populates a schema from a schema file pointed to by <path>.
    pub fn load_from_file(&mut self, path: String) {
        let mut schema = self.schema.write().unwrap();
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(schema_file) = parse_schema(&mut file) {
                for instance in schema_file.components {
                    schema.add_component(instance);
                }

                schema_file.wires.iter().for_each( |w: &WireSegment| {
                    schema.add_wire(w.clone());
                });
            } else {
                println!("Could not parse the schema file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }
}