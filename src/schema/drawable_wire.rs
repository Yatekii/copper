use std::cell::RefCell;
use std::rc::Rc;


use drawables;
use drawing;
 use schema_parser::geometry;
use schema_parser::schema_file::{WireSegment,WireType};
use resource_manager;


pub struct DrawableWire {
    pub start: geometry::SchemaPoint2D,
    pub end: geometry::SchemaPoint2D,
    pub wire: Box<drawables::Drawable>,
}


impl DrawableWire {
    pub fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: &geometry::TSchemaScreen){
        self.wire.draw(resource_manager.clone(), perspective.clone());
    }

    // pub fn from_schema(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, wire: &WireSegment) -> DrawableWire {
    //     let start = geometry::SchemaPoint2D::new(wire.start.x, -wire.start.y);
    //     let end = geometry::SchemaPoint2D::new(wire.end.x, -wire.end.y);
    //     let color = match wire.kind {
    //         WireType::Wire => drawing::Color::new(0.0, 0.28, 0.0, 1.0),
    //         WireType::Dotted => drawing::Color::new(0.0, 0.0, 0.48, 1.0),
    //         _ => drawing::Color::new(0.0, 0.28, 0.0, 1.0)
    //     };
    //     DrawableWire {
    //         start: start.clone(),
    //         end: end.clone(),
    //         wire: Box::new(drawables::loaders::load_line(resource_manager.clone(), color, &start, &end))
    //     }
    // }
}