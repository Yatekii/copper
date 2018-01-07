use std::cell::RefCell;
use std::rc::Rc;


use gfx;
use gfx_device_gl;


use drawing;
use drawing::pipe;
use resource_manager;
use geometry;


type Resources = gfx_device_gl::Resources;


pub struct ShapeDrawable<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    color: drawing::Color
}

impl ShapeDrawable<Resources> {
    pub fn new(bundle: gfx::pso::bundle::Bundle<Resources, pipe::Data<Resources>>, color: drawing::Color) -> Self {
        ShapeDrawable {
            bundle: bundle,
            color: color
        }
    }
}

impl super::Drawable for ShapeDrawable<Resources> {
    fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: geometry::TSchemaScreen){
        let locals = drawing::Locals {
            perspective: perspective.to_row_arrays(),
            color: self.color.color,
        };
        resource_manager.borrow_mut().encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

        self.bundle.encode(&mut resource_manager.borrow_mut().encoder);
    }
}