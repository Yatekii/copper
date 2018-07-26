use geometry;
use drawing;
use drawing::drawables;
use parsing::kicad::component_library::*;


const PIN_RADIUS: f32 = 10.0;

pub fn load_pin(
    component_id: u32,
    position: &geometry::Point2,
    length: f32,
    orientation: &PinOrientation,
    name: Option<String>,
    number: usize,
    number_size: usize,
    name_size: usize
) -> drawables::GroupDrawable {
    // Create a new group drawable
    let mut group = drawables::GroupDrawable::default();

    let circle = super::load_circle(component_id, drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, PIN_RADIUS, false);

    let orientation_vec = geometry::Vector2::new(orientation.unit_vec().x, orientation.unit_vec().y);
    let end_position = position.clone() + (orientation_vec * length);

    let number_pos = end_position + (orientation_vec * -10.0);
    let number_pos = geometry::Point2::new(number_pos.x, number_pos.y + 60.0);

    let number_orientation = match orientation {
        &PinOrientation::Up => TextOrientation::Vertical,
        &PinOrientation::Down => TextOrientation::Vertical,
        &PinOrientation::Right => TextOrientation::Horizontal,
        &PinOrientation::Left => TextOrientation::Horizontal
    };

    let number_hjustify = match orientation {
        &PinOrientation::Up => Justify::Right,
        &PinOrientation::Down => Justify::Left,
        &PinOrientation::Right => Justify::Right,
        &PinOrientation::Left => Justify::Left
    };

    let number_text = super::load_text(&number_pos, &format!("{}", number), number_size as f32, &number_orientation, number_hjustify, Justify::Center);

    if let Some(name) = name {
        let name_pos = end_position + orientation_vec * 20.0;
        let name_pos = geometry::Point2::new(name_pos.x, name_pos.y + 25.0);
        let name_hjustify = match orientation {
            &PinOrientation::Up => Justify::Left,
            &PinOrientation::Down => Justify::Right,
            &PinOrientation::Right => Justify::Left,
            &PinOrientation::Left => Justify::Right
        };
        let name_text = super::load_text(&name_pos, &name, name_size as f32, &number_orientation, name_hjustify, Justify::Center);
        group.add(name_text);
    }

    let line = super::load_line(component_id, drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, &end_position);

    group.add(line);
    group.add(circle);
    group.add(number_text);

    group
}