mod drawable_component;
mod drawable_wire;


use std::fs;
use std::rc::Rc;
use std::cell::Cell;


use manipulation::library::Library;
pub use self::drawable_component::DrawableComponent;
pub use self::drawable_wire::DrawableWire;
use geometry::schema_elements::*;

use parsing::schema_file::ComponentInstance;

use geometry::{
    Point2D,
    Vector2D,
    AABB
};
use drawing;


pub struct DrawableComponentInstance {
    pub instance: ComponentInstance,
    drawable: Rc<DrawableComponent>,
}

// TODO: Implement
impl DrawableComponentInstance {
    // pub fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: &geometry::TSchemaScreen) {

    // }

    pub fn get_boundingbox(&self) -> AABB {
        let i = &self.instance;
        use utils::traits::Translatable;
        let bb = i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
        bb
    }
}

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    wires: Vec<DrawableWire>,
    drawables: Vec<DrawableComponentInstance>,
    bounding_box: Cell<Option<AABB>>
}

impl Schema {
    /// Creates a new blank schema
    pub fn new() -> Schema {
        Schema {
            wires: Vec::new(),
            drawables: Vec::new(),
            bounding_box: Cell::new(None)
        }
    }

    /// Populates a schema from a schema file pointed to by <path>.
    pub fn load(&mut self, library: &Library, path: String) {
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(mut schema_file) = ::parse_schema(&mut file){
                for mut instance in schema_file.components {
                    let component = library.get_component(&instance);
                    instance.set_component(component.clone());

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

            drawable.drawable.draw(buffers);
        }

        for wire in &self.wires {
            wire.draw(buffers);
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self) -> AABB {
        let mut aabb = AABB::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 0.0)
        );
        for component in &self.drawables {
            let i = &component.instance;
            use utils::traits::Translatable;
            let bb = &i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
            use ncollide2d::bounding_volume::BoundingVolume;
            aabb.merge(bb);
        }
        aabb
    }

    pub fn get_currently_selected_component(&self) -> Option<&DrawableComponentInstance> {
        for component in &self.drawables {
            let i = &component.instance;
            use utils::traits::Translatable;
            let bb = &i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
            if bb.half_extents().x > 0.0 {
                return Some(component);
            }
        }
        None
    }
}