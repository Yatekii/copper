use drawing;
use drawing::drawables;
use geometry;
use geometry::*;
use geometry::schema_elements::*;
use super::super::Drawable;

pub struct WireDrawable {
    pub wire: Box<Drawable>,
    transform: Matrix4,
}


impl WireDrawable {
    pub fn from_schema(component_id: u32, wire: &WireSegment) -> WireDrawable {
        let start = geometry::Point2D::new(wire.start.x, wire.start.y);
        let end = geometry::Point2D::new(wire.end.x, wire.end.y);
        let color = match wire.kind {
            WireType::Wire => drawing::Color::new(0.0, 0.28, 0.0, 1.0),
            WireType::Dotted => drawing::Color::new(0.0, 0.0, 0.48, 1.0),
            _ => drawing::Color::new(0.0, 0.28, 0.0, 1.0)
        };
        WireDrawable {
            wire: Box::new(drawables::loaders::load_line(component_id, color, &start, &end)),
            transform: Matrix4::identity(),
        }
    }
}

impl Drawable for WireDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers){
        buffers.abo.push(drawing::Attributes {
            transform: geometry::Matrix4::identity().into()
        });
        self.wire.draw(buffers);
    }
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }
    fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = transform.clone();
    }
}