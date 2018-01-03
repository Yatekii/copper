use std::fs;
use std::f32;
use std::cell::RefCell;
use std::rc::Rc;


use euclid;
use gfx;
use gfx_device_gl;


use resource_manager;
use library::Library;
use schema_parser;
use drawable_component::DrawableComponent;
use drawing;
use schema_parser::component;
use schema_parser::component::geometry;


type Resources = gfx_device_gl::Resources;


pub struct Schema {
    resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    components: Vec<DrawableComponent>
}

impl Schema {
    pub fn new(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>) -> Schema {
        Schema {
            resource_manager: resource_manager,
            components: Vec::new()
        }
    }

    pub fn load(&mut self, library: &Library, path: String) {
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(components) = schema_parser::parse_schema(&mut file){
                for instance in components {
                    let component = library.get_component(&instance);
                    let mut drawable = DrawableComponent::new(self.resource_manager.clone(), component.clone());
                    drawable.instance = Some(instance);
                    self.components.push(drawable);
                }
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }

    pub fn draw(&self, encoder: &mut gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>, perspective: &drawing::Transform2D) {
        for component in &self.components {
            // Unwrap should be ok as there has to be an instance for every component in the schema
            let i = component.instance.as_ref().unwrap();
            component.draw(encoder, &drawing::Transform2D(
                perspective.pre_translate(euclid::TypedVector2D::new(i.position.x, -i.position.y))
            ));
        }
    }

    pub fn get_bounding_box(&self) -> (component::geometry::Point, component::geometry::Point){
        let mut max_x = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;

        for component in &self.components {
            let i = component.instance.as_ref().unwrap();
            let bb = &component.bounding_box;
            let startx = bb.0.x + i.position.x;
            let starty = bb.0.y - i.position.y;
            let endx = bb.1.x + i.position.x;
            let endy = bb.1.y - i.position.y;

            max_x = max_x.max(startx).max(endx);
            min_x = min_x.min(startx).min(endx);
            max_y = max_y.max(starty).max(endy);
            min_y = min_y.min(starty).min(endy);
        }
        (component::geometry::Point { x: min_x, y: min_y }, component::geometry::Point { x: max_x, y: max_y })
    }
}