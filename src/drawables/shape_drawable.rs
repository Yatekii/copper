use std::cell::RefCell;
use std::rc::Rc;


use gfx;
use gfx_device_gl;


use drawing;
use drawing::pipe;
use resource_manager;
use schema_parser::geometry;


type Resources = gfx_device_gl::Resources;


pub struct ShapeDrawable {
    buffers: drawing::Buffers,
    color: drawing::Color
}

impl ShapeDrawable {
    pub fn new(buffers: drawing::Buffers, color: drawing::Color) -> Self {
        ShapeDrawable {
            buffers,
            color: color
        }
    }
}

impl super::Drawable for ShapeDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers){
        self.buffers.apply_to(buffers);
    }
}