use std::f32::consts::PI;
use euclid;

use geometry::{SchemaPoint2D,SchemaVector2D,TSchemaSchema};


pub type ThicknessType = f32;




#[derive(Debug, Clone)]
pub enum GraphicElement {
    Polygon {
        points: Vec<SchemaPoint2D>,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: SchemaPoint2D,
        end: SchemaPoint2D,
        unit: usize,
        convert: usize,
        filled: bool,
        // TODO: parts, convert, filled
    },
    Circle {
        center: SchemaPoint2D,
        radius: ThicknessType,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    CircleArc {
        center: SchemaPoint2D,
        radius: ThicknessType,
        start_coord: SchemaPoint2D,
        end_coord: SchemaPoint2D,
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
        position: SchemaPoint2D,
        unit: usize,
        convert: usize
        // TODO: parts, convert, filled
    },
    Pin {
        orientation: PinOrientation,
        name: Option<String>,
        number: usize,
        position: SchemaPoint2D,
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

    pub fn rot(&self) -> TSchemaSchema {
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
    pub fn unit_vec(&self) -> SchemaVector2D {
        match *self {
            PinOrientation::Up => euclid::TypedVector2D::new(0.0, 1.0),
            PinOrientation::Down => euclid::TypedVector2D::new(0.0, -1.0),
            PinOrientation::Right => euclid::TypedVector2D::new(1.0, 0.0),
            PinOrientation::Left => euclid::TypedVector2D::new(-1.0, 0.0),
        }
    }
}