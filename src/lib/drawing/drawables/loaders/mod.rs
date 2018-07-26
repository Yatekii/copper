mod circle;
mod line;
mod pin;
mod polygon;
mod rectangle;
mod text;


pub use self::circle::load_circle;
pub use self::line::load_line;
pub use self::pin::load_pin;
pub use self::polygon::load_polygon;
pub use self::rectangle::load_rectangle;
pub use self::text::load_text;

use drawing;
use drawing::drawables;
use geometry::*;
use parsing::kicad::component_library::*;

pub const VS_CODE: &[u8] = include_bytes!("../../shaders/shape.glslv");
pub const FS_CODE: &[u8] = include_bytes!("../../shaders/shape.glslf");

pub const VS_RENDER_CODE: &[u8] = include_bytes!("../../shaders/render.glslv");
pub const FS_RENDER_CODE: &[u8] = include_bytes!("../../shaders/render.glslf");

pub fn load_drawable_from_graphic_element(
    component_id: u32,
    shape: &GraphicElement,
) -> Option<Box<drawables::Drawable>> {
    match shape {
        &GraphicElement::Rectangle { start, end, filled, .. } => {
            let mins = Point2D::new(
                if start.x > end.x { end.x } else { start.x },
                if start.y > end.y { end.y } else { start.y }
            );
            let maxs = Point2D::new(
                if start.x > end.x { start.x } else { end.x },
                if start.y > end.y { start.y } else { end.y }
            );
            let r = AABB::new(
                mins,
                maxs
            );
            Some(Box::new(drawables::loaders::load_rectangle(
                component_id,
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &r,
                filled
            )))
        }
        &GraphicElement::Circle { ref center, radius, filled, .. } => {
            Some(Box::new(drawables::loaders::load_circle(
                component_id,
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &center.clone(),
                radius, filled
            )))
        },
        &GraphicElement::Pin { ref orientation, ref position, length, ref name, number, number_size, name_size, .. } => {
            Some(Box::new(drawables::loaders::load_pin(
                component_id,
                &(position.clone()),
                length as f32, orientation, name.clone(), number, number_size, name_size
            )))
        },
        &GraphicElement::Polygon { ref points, filled, .. } => {
            Some(Box::new(drawables::loaders::load_polygon(
                component_id,
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &points.iter().map(|point| Point2D::new(point.x, point.y)).collect(),
                filled
            )))
        },
        _ => None
    }
}