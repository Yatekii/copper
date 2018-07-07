use std::f32;

use drawing;
use drawing::drawables;
use geometry;
use geometry::schema_elements::*;


pub struct DrawableWire {
    pub start: geometry::Point2D,
    pub end: geometry::Point2D,
    pub wire: Box<drawables::Drawable>,
}


impl DrawableWire {
    pub fn draw(&self, buffers: &mut drawing::Buffers){
        buffers.abo.push(drawing::Attributes {
            transform: geometry::Matrix4::identity().into()
        });
        self.wire.draw(buffers);
    }

    pub fn get_boundingbox(&self) -> geometry::AABB {
        let max_x = self.start.x.max(self.end.x );
        let min_x = self.start.x.min(self.end.x);
        let max_y = self.start.y.max(self.end.y);
        let min_y = self.start.y.min(self.end.y);
        if max_x > f32::MIN
        && max_y > f32::MIN
        && min_x < f32::MAX
        && min_y < f32::MAX {
            geometry::AABB::new(
                geometry::Point2D::new(min_x, min_y),
                geometry::Point2D::new(max_x, max_y)
            )
        } else {
            geometry::AABB::new(
                geometry::Point2D::new(0.0, 0.0),
                geometry::Point2D::new(0.0, 0.0)
            )
        }
    }

    pub fn from_schema(component_id: u32, wire: &WireSegment) -> DrawableWire {
        let start = geometry::Point2D::new(wire.start.x, wire.start.y);
        let end = geometry::Point2D::new(wire.end.x, wire.end.y);
        let color = match wire.kind {
            WireType::Wire => drawing::Color::new(0.0, 0.28, 0.0, 1.0),
            WireType::Dotted => drawing::Color::new(0.0, 0.0, 0.48, 1.0),
            _ => drawing::Color::new(0.0, 0.28, 0.0, 1.0)
        };
        DrawableWire {
            start: start.clone(),
            end: end.clone(),
            wire: Box::new(drawables::loaders::load_line(component_id, color, &start, &end))
        }
    }
}