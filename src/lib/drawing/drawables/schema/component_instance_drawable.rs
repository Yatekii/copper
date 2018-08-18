use drawing;
use drawing::drawables::*;
use geometry::Matrix4;
use state::schema::component::Component;
use super::super::Drawable;


pub struct ComponentInstanceDrawable {
    drawables: Vec<Box<Drawable>>,
    transform: Matrix4,
}

impl ComponentInstanceDrawable {
    pub fn new(
        component_id: u32,
        component: &Component
    ) -> ComponentInstanceDrawable {
        // Generate all shapes for the component
        let drawables = component.get_graphic_elements()
                                 .iter()
                                 .filter_map(|shape| loaders::load_drawable_from_graphic_element(component_id, &shape))
                                 .collect::<Vec<_>>();

        ComponentInstanceDrawable {
            drawables: drawables,
            transform: Matrix4::identity(),
        }
    }
    
}

impl Drawable for ComponentInstanceDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers, _transform: bool) {
        buffers.abo.push(drawing::Attributes {
            transform: self.transform.into()
        });
        for drawable in &self.drawables {
            drawable.draw(buffers, false);
        }
    }
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }
    fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = transform.clone();
    }
    fn set_id(&mut self, id: u32) {
        for drawable in &mut self.drawables {
            drawable.set_id(id);
        }
    }
}