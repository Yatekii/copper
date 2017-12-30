use std::f32::consts::PI;


pub type CoordType = f32;
pub type ThicknessType = f32;

use euclid;


pub struct SchemaSpace;

pub type SchemaPoint = euclid::TypedPoint2D<f32, SchemaSpace>;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: CoordType,
    pub y: CoordType,
}

impl Point {
    pub fn to_euclid(&self) -> SchemaPoint {
        euclid::TypedPoint2D::new(self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub enum GraphicElement {
    Polygon {
        points: Vec<Point>,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point,
        end: Point,
        unit: usize,
        convert: usize,
        filled: bool,
        // TODO: parts, convert, filled
    },
    Circle {
        center: Point,
        radius: ThicknessType,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    CircleArc {
        center: Point,
        radius: ThicknessType,
        start_coord: Point,
        end_coord: Point,
        start_angle: isize,
        end_angle: isize,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    TextField {
        content: String,
        orientation: TextOrientation,
        position: Point,
        unit: usize,
        convert: usize
        // TODO: parts, convert, filled
    },
    Pin {
        orientation: PinOrientation,
        name: Option<String>,
        number: usize,
        position: Point,
        length: usize,
        number_size: usize,
        name_size: usize,
        unit: usize,
        convert: usize,
        etype: String,
        shape: Option<String>
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextOrientation {
    Horizontal,
    Vertical,
}

impl TextOrientation {
    pub fn from_char(c: char) -> Option<TextOrientation> {
        match c {
            'H' => Some(TextOrientation::Horizontal),
            'V' => Some(TextOrientation::Vertical),
            _ => None
        }
    }

    pub fn rot(&self) -> euclid::TypedTransform3D<f32, SchemaSpace, SchemaSpace> {
        match *self {
            TextOrientation::Vertical => euclid::TypedTransform3D::create_rotation(0.0, 0.0, 1.0, euclid::Length::new(-PI / 2.0)),
            TextOrientation::Horizontal => euclid::TypedTransform3D::identity()
        }
    }
}

#[derive(Debug, Clone)]
pub enum PinDescription {
}

#[derive(Debug, PartialEq, Clone)]
pub enum PinOrientation {
    Up,
    Down,
    Right,
    Left,
}

impl PinOrientation {
    pub fn unit_vec(&self) -> euclid::TypedVector2D<f32, SchemaSpace> {
        match *self {
            PinOrientation::Up => euclid::TypedVector2D::new(0.0, 1.0),
            PinOrientation::Down => euclid::TypedVector2D::new(0.0, -1.0),
            PinOrientation::Right => euclid::TypedVector2D::new(1.0, 0.0),
            PinOrientation::Left => euclid::TypedVector2D::new(-1.0, 0.0),
        }
    }
}