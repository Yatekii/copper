pub type CoordType = isize;
pub type ThicknessType = usize;

#[derive(Debug)]
pub struct Point {
    pub x: CoordType,
    pub y: CoordType,
}

#[derive(Debug)]
pub enum GraphicElement {
    Polygon {
        points: Vec<Point>,
        unit: usize,
        convert: usize,
        thickness: usize,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point,
        end: Point,
        unit: usize,
        convert: usize,
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
        convert: usize,
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

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum PinDescription {
}

#[derive(Debug, PartialEq)]
pub enum PinOrientation {
    Up,
    Down,
    Right,
    Left,
}

impl PinOrientation {
    pub fn unit_vec(&self) -> [i8;2] {
        match *self {
            PinOrientation::Up => [0, 1],
            PinOrientation::Down => [0, -1],
            PinOrientation::Right => [1, 0],
            PinOrientation::Left => [-1, 0],
        }
    }
}