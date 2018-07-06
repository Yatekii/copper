use geometry;
use drawing;
use drawing::drawables;
use geometry::schema_elements;


const PIN_RADIUS: f32 = 10.0;

pub fn load_pin(
    position: &geometry::Point2D,
    length: f32,
    orientation: &schema_elements::PinOrientation,
    name: Option<String>,
    number: usize,
    number_size: usize,
    name_size: usize
) -> drawables::GroupDrawable {
    // Create a new group drawable
    let mut group = drawables::GroupDrawable::default();

    let circle = super::load_circle(drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, PIN_RADIUS, false);

    let orientation_vec = geometry::Vector2D::new(orientation.unit_vec().x, orientation.unit_vec().y);
    let end_position = position.clone() + (orientation_vec * length);

    let number_pos = end_position + (orientation_vec * -10.0);
    let number_pos = geometry::Point2D::new(number_pos.x, number_pos.y + 60.0);

    let number_orientation = match orientation {
        &schema_elements::PinOrientation::Up => schema_elements::TextOrientation::Vertical,
        &schema_elements::PinOrientation::Down => schema_elements::TextOrientation::Vertical,
        &schema_elements::PinOrientation::Right => schema_elements::TextOrientation::Horizontal,
        &schema_elements::PinOrientation::Left => schema_elements::TextOrientation::Horizontal
    };

    let number_hjustify = match orientation {
        &schema_elements::PinOrientation::Up => schema_elements::Justify::Right,
        &schema_elements::PinOrientation::Down => schema_elements::Justify::Left,
        &schema_elements::PinOrientation::Right => schema_elements::Justify::Right,
        &schema_elements::PinOrientation::Left => schema_elements::Justify::Left
    };

    let number_text = super::load_text(&number_pos, &format!("{}", number), number_size as f32, &number_orientation, number_hjustify, schema_elements::Justify::Center);

    if let Some(name) = name {
        let name_pos = end_position + orientation_vec * 20.0;
        let name_pos = geometry::Point2D::new(name_pos.x, name_pos.y + 25.0);
        let name_hjustify = match orientation {
            &schema_elements::PinOrientation::Up => schema_elements::Justify::Left,
            &schema_elements::PinOrientation::Down => schema_elements::Justify::Right,
            &schema_elements::PinOrientation::Right => schema_elements::Justify::Left,
            &schema_elements::PinOrientation::Left => schema_elements::Justify::Right
        };
        let name_text = super::load_text(&name_pos, &name, name_size as f32, &number_orientation, name_hjustify, schema_elements::Justify::Center);
        group.add(name_text);
    }

    let line = super::load_line(drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, &end_position);

    group.add(line);
    group.add(circle);
    group.add(number_text);

    group
}