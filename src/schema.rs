use std::fs;
use std::f32;


use glium;
use euclid;


use resource_manager::ResourceManager;
use library::Library;
use schema_parser;
use drawable_component::DrawableComponent;
use drawing;


pub struct Schema<'a> {
    resource_manager: &'a ResourceManager<'a>,
    components: Vec<DrawableComponent<'a>>
}

impl<'a> Schema<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Schema<'a> {
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
                    let mut drawable = DrawableComponent::new(self.resource_manager, component.clone());
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

    pub fn draw(&self, target: &mut glium::Frame, perspective: &drawing::Transform2D) {
        for component in &self.components {
            // Unwrap should be ok as there has to be an instance for every component in the schema
            let i = component.instance.as_ref().unwrap();
            component.draw(target, &drawing::Transform2D(
                perspective.pre_translate(euclid::TypedVector2D::new(i.position.x, -i.position.y))
            ));
        }
    }

    pub fn get_bounding_box(&self) {
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
    }
}