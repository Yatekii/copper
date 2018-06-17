mod drawable_component;
mod drawable_wire;


use std::fs;
use std::rc::Rc;
use std::cell::Cell;


use library::Library;
use schema_parser;
pub use self::drawable_component::DrawableComponent;
pub use self::drawable_wire::DrawableWire;
use schema_parser::schema_file::WireSegment;

use schema_parser::schema_file::ComponentInstance;

use std::collections::HashMap;
use drawing;

use schema_parser::helpers::SchemaAABB;

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
    components: HashMap<String, Rc<DrawableComponent>>,
    wires: Vec<DrawableWire>,
    drawables: Vec<DrawableComponentInstance>,
    bounding_box: Cell<Option<SchemaAABB>>
}

impl Schema {
    /// Creates a new blank schema
    pub fn new() -> Schema {
        Schema {
            components: HashMap::new(),
            wires: Vec::new(),
            drawables: Vec::new(),
            bounding_box: Cell::new(None)
        }
    }

    /// Populates a schema from a schema file pointed to by <path>.
    pub fn load(&mut self, library: &Library, path: String) {
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(schema_file) = schema_parser::parse_schema(&mut file){
                for instance in schema_file.components {
                    let component = library.get_component(&instance);

                    if !self.components.contains_key(&component.name) {
                        let drawable = DrawableComponent::new(component.clone(), instance.clone());

                        self.components.insert(component.name.clone(), Rc::new(drawable));
                    }

                    let drawable_component = DrawableComponentInstance {
                        instance: instance.clone(),
                        drawable: Rc::new(DrawableComponent::new(component.clone(), instance.clone())),
                    };

                    self.drawables.push(drawable_component);
                }

                self.wires.extend(schema_file.wires.iter().map( |w: &WireSegment| DrawableWire::from_schema(w)));
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }

    /// Issues draw calls to render the entire schema
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        for drawable in &self.drawables {
            // Unwrap should be ok as there has to be an instance for every component in the schema
            let i = &drawable.instance;

            debug!("Drawing component {}", i.name);

            drawable.drawable.draw(buffers);

         
            // component.draw(self.resource_manager.clone(), &perspective.pre_translate(euclid::TypedVector3D::new(i.position.x, -i.position.y, 0.0)));
        }

        for wire in &self.wires {
            debug!("Drawing wire from {:?} to {:?}", wire.start, wire.end);
            wire.draw(buffers);
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self) -> SchemaAABB {
        use ncollide2d::math::{ Point, Vector };
        let mut aabb = SchemaAABB::new(
            Point::new(0.0, 0.0),
            Point::new(0.0, 0.0)
        );
        for component in &self.drawables {
            let i = &component.instance;
            use schema_parser::helpers::Translatable;
            let bb = &i.get_boundingbox().translated(Vector::new(i.position.x, i.position.y));
            use ncollide2d::bounding_volume::BoundingVolume;
            aabb.merge(bb)
        }
        aabb
    }
}