mod drawable_component;
mod drawable_wire;


use std::fs;
use std::f32;
use std::cell::RefCell;
use std::rc::Rc;


use euclid;


use resource_manager;
use library::Library;
use schema_parser;
pub use self::drawable_component::DrawableComponent;
pub use self::drawable_wire::DrawableWire;
use schema_parser::schema_file::WireSegment;

use schema_parser::schema_file::ComponentInstance;

use std::collections::HashMap;
use schema_parser::geometry;


struct DrawableComponentInstance {
    instance: ComponentInstance,
    drawable: Rc<DrawableComponent>,
}

// TODO: Implement
// impl DrawableComponentInstance {
//     pub fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: &geometry::TSchemaScreen) {

//     }
// }

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    components: HashMap<String, Rc<DrawableComponent>>,
    wires: Vec<DrawableWire>,
    drawables: Vec<DrawableComponentInstance>,
}

impl Schema {
    /// Creates a new blank schema
    pub fn new(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>) -> Schema {
        Schema {
            resource_manager: resource_manager,
            components: HashMap::new(),
            wires: Vec::new(),
            drawables: Vec::new(),
        }
    }

    /// Populates a schema from a schema file pointed to by <path>.
    pub fn load(&mut self, library: &Library, path: String) {
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(schema_file) = schema_parser::parse_schema(&mut file){
                for instance in schema_file.components {
                    let component = library.get_component(&instance);

                    if !self.components.contains_key(&component.name) {
                        let drawable = DrawableComponent::new(self.resource_manager.clone(), component.clone());

                        self.components.insert(component.name.clone(), Rc::new(drawable));
                    }


                    let drawable_component = DrawableComponentInstance {
                        instance: instance.clone(),
                        drawable: self.components.get(&component.name).unwrap().clone(),
                    };

                    self.drawables.push(drawable_component);
                }

                let rm = self.resource_manager.clone();

                self.wires.extend(schema_file.wires.iter().map( |w: &WireSegment| DrawableWire::from_schema(rm.clone(), w) ));
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }

    /// Issues draw calls to render the entire schema
    pub fn draw(&self, perspective: &geometry::TSchemaScreen) {
        for drawable in &self.drawables {
            // Unwrap should be ok as there has to be an instance for every component in the schema
            let i = &drawable.instance;

            debug!("Drawing component {}", i.name);

            drawable.drawable.draw(self.resource_manager.clone(), &perspective.pre_translate(euclid::TypedVector3D::new(i.position.x, -i.position.y, 0.0)));

         
            // component.draw(self.resource_manager.clone(), &perspective.pre_translate(euclid::TypedVector3D::new(i.position.x, -i.position.y, 0.0)));
        }

        for wire in &self.wires {
            debug!("Drawing wire from {:?} to {:?}", wire.start, wire.end);
            wire.draw(self.resource_manager.clone(), perspective);
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self) -> (geometry::SchemaPoint2D, geometry::SchemaPoint2D){
        let mut max_x = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;

        for component in &self.drawables {
            let i = &component.instance;
            let bb = &component.drawable.bounding_box;
            let startx = bb.0.x + i.position.x;
            let starty = bb.0.y - i.position.y;
            let endx = bb.1.x + i.position.x;
            let endy = bb.1.y - i.position.y;

            max_x = max_x.max(startx).max(endx);
            min_x = min_x.min(startx).min(endx);
            max_y = max_y.max(starty).max(endy);
            min_y = min_y.min(starty).min(endy);
        }
        (geometry::SchemaPoint2D::new(min_x, min_y), geometry::SchemaPoint2D::new(max_x, max_y))
    }
}