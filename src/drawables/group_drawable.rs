use std::cell::RefCell;
use std::rc::Rc;


use resource_manager;
use schema_parser::geometry;


pub struct GroupDrawable {
    drawables: Vec<Box<super::Drawable>>
}

impl GroupDrawable {
    pub fn default() -> Self {
        GroupDrawable {
            drawables: Vec::new()
        }
    }

    pub fn add<T: 'static + super::Drawable>(&mut self, drawable: T) {
        self.drawables.push(Box::new(drawable));
    }
}

impl super::Drawable for GroupDrawable {
    fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: geometry::TSchemaScreen) {
        for drawable in &self.drawables {
            drawable.draw(resource_manager.clone(), perspective.clone());
        }
    }
}