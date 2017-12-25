use std::vec;


use glium;


use shape;
use drawing;


pub struct Group {
    position: drawing::Vertex,
    // rotation: drawing::Vertex,
    // scale: drawing::Vertex,
    shapes: vec::Vec<Box<shape::Shape>>
}

impl Group {
    /// Create a new group
    pub fn new(position: drawing::Vertex, shapes: vec::Vec<Box<shape::Shape>>) -> Group {
        Group {
            position: position,
            shapes: shapes
        }
    }

    /// Issue a draw call to OGL for all the children
    pub fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, perspective: &drawing::Mat4) {
        for shape in &self.shapes {
            shape.grouped_draw(target, params, perspective, self.position);
        }
    }

    /// Adds a new shape to the group
    pub fn add_shape(&mut self, shape: Box<shape::Shape>) {
        self.shapes.push(shape);
    }
}