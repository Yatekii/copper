use drawing;
use geometry::Matrix4;
use drawing::drawables::loaders;
use state::schema::component::Component;


pub struct ComponentDrawable {
    drawables: Vec<Box<super::super::Drawable>>,
}

impl ComponentDrawable {
    pub fn new(
        component_id: u32,
        component: &Component
    ) -> Self {
        // Generate all shapes for the component
        let drawables = component.get_graphic_elements()
                                 .iter()
                                 .filter_map(|shape| loaders::load_drawable_from_graphic_element(component_id, &shape))
                                 .collect::<Vec<_>>();

        Self {
            drawables: drawables,
        }
    }
    
}

impl super::super::Drawable for ComponentDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers, transform: bool){
        buffers.abo.push(drawing::Attributes {
            transform: Matrix4::identity().into()
        });
        for drawable in &self.drawables {
            drawable.draw(buffers, false);
        }
    }
    fn get_transform(&self) -> Matrix4 { Matrix4::identity() }
    fn set_transform(&mut self, _transform: &Matrix4) {}
    fn set_id(&mut self, id: u32) {
        for drawable in &mut self.drawables {
            drawable.set_id(id);
        }
    }
}