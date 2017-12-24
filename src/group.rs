use std::vec;


use glium;


use shape;
use drawing;


pub struct Group {
    // position: drawing::Vertex,
    // rotation: drawing::Vertex,
    // scale: drawing::Vertex,
    shapes: vec::Vec<Box<shape::Shape>>
}

impl Group {
    /// Create a new group
    fn new(&self, shapes: vec::Vec<Box<shape::Shape>>) -> Group {
        Group {
            shapes: shapes
        }
    }

    /// Issue a draw call to OGL for all the children
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters) {
        for shape in &self.shapes {
            shape.draw(target, params);
        }
    }

    /// Adds a new shape to the group
    fn add_shape(&mut self, shape: Box<shape::Shape>) {
        self.shapes.push(shape);
    }
}