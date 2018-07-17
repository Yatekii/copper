use std::f32::consts::PI;
use nalgebra;
use uuid::Uuid;

use geometry::{ Point2D, Vector2D, Matrix3 };

#[derive(Debug, Clone)]
pub enum Justify {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl Justify {
    pub fn from_char(c: char) -> Option<Justify> {
        match c {
            'L' => Some(Justify::Left),
            'R' => Some(Justify::Right),
            'T' => Some(Justify::Top),
            'B' => Some(Justify::Bottom),
            'C' => Some(Justify::Center),
            _   => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GraphicElement {
    Polygon {
        points: Vec<Point2D>,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point2D,
        end: Point2D,
        unit: usize,
        convert: usize,
        filled: bool,
        // TODO: parts, convert, filled
    },
    Circle {
        center: Point2D,
        radius: f32,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    CircleArc {
        center: Point2D,
        radius: f32,
        start_coord: Point2D,
        end_coord: Point2D,
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
        position: Point2D,
        unit: usize,
        convert: usize
        // TODO: parts, convert, filled
    },
    Pin {
        orientation: PinOrientation,
        name: Option<String>,
        number: usize,
        position: Point2D,
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

    pub fn rot(&self) -> Matrix3 {
        match *self {
            TextOrientation::Vertical => nalgebra::geometry::Rotation3::from_axis_angle(&nalgebra::base::Vector3::z_axis(), -PI / 2.0).unwrap(),
            TextOrientation::Horizontal => Matrix3::identity()
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
    pub fn unit_vec(&self) -> Vector2D {
        match *self {
            PinOrientation::Up => Vector2D::new(0.0, 1.0),
            PinOrientation::Down => Vector2D::new(0.0, -1.0),
            PinOrientation::Right => Vector2D::new(1.0, 0.0),
            PinOrientation::Left => Vector2D::new(-1.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WireSegment {
    pub uuid: Uuid,
    pub kind: WireType,
    pub start: Point2D,
    pub end: Point2D,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WireType {
    Wire,
    Bus,
    Dotted
}

#[derive(Debug)]
pub struct Label {
    pub text: String,
    pub position: Point2D,
    //todo: fill
}

#[derive(Debug)]
pub struct Note {
    pub text: String,
    //todo: fill
}

#[derive(Debug)]
pub struct Junction {
    pub position: Point2D,
}

#[derive(Debug)]
pub struct NoConnection {
    pub position: Point2D,
}