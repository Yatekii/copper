use std::cell::Cell;
use std::io::Read;
use std::f32::consts::PI;
use std::str;

use uuid::Uuid;
use nom::{
    alphanumeric,
    alpha,
    space,
    line_ending,
    digit,
};
use nom::Err;
use nom::simple_errors::Context as NomErrorContext;
use nom::types::CompleteByteSlice;

use parsing::common::*;
use state::schema::component::{
    OptionFlag,
    Component,
    Field,
};
use geometry::*;


/// Parses an entire KiCad library file.
/// Returns the set of components it contains if the parse was successful.
/// Returns `None` otherwise.
pub fn parse_components_library<R: Read>(data: &mut R) -> Option<Vec<Component>> {
    let mut buff: Vec<u8> = Vec::new();

    if let Ok(_) = data.read_to_end(&mut buff) {
        let parse_raw = component_library(CompleteByteSlice(&buff));
        match parse_raw {
            Ok((_, components)) => Some(components),
            Err(e) => match e {
                Err::Incomplete(n) => {
                    println!("Required Number of Bytes {:?}", n);
                    None
                },
                Err::Error(c) => {
                    let NomErrorContext::Code(i, e) = c;
                    println!("We got an error that should be handled by Nom");
                    println!("ErrorKind is {:?}", e);
                    println!("Input was\n{:?}", str::from_utf8(i.0));
                    None
                },
                Err::Failure(c) => {
                    let NomErrorContext::Code(i, e) = c;
                    println!("Nom failed to parse the file.");
                    println!("ErrorKind is {:?}", e);
                    println!("Input was\n{:?}", str::from_utf8(i.0));
                    None
                }
            }
        }
    } else {
        None
    }
}

/* P A R S E   L I B R A R Y */

/// Nom parser which takes an entire EESchema library
named!(component_library(CompleteByteSlice) -> Vec<Component>,
    do_parse!(
        tag_s!("EESchema-LIBRARY Version") >>
        space >>
        digit >>
        tag_s!(".") >>
        digit >>
        line_ending >>
        components: many1!(parse_component) >>
        (components)
    )
);

/* P A R S E   C O M P O N E N T */

/// Parses a Component from start to end
named!(parse_component(CompleteByteSlice) -> (Component),
    do_parse!(
        many0!(comment) >>
        component_struct: delimited!(
            tag!("DEF"),
            component_def,
            tag!("ENDDEF")
        ) >>
        opt!(line_ending) >>
        many0!(comment) >>
        (component_struct)
    )
);

/// Parses the body of a Component
named!(component_def(CompleteByteSlice) -> (Component),
    do_parse!(
        space >>
        component_name: utf8_str >>
        space >>
        reference: utf8_str >>
        space >>
        _unused: alphanumeric >>
        space >>
        text_offset: int >>
        space >>
        draw_pin_number: yesno >>
        space >>
        draw_pin_name: yesno >>
        space >>
        unit_count: int >>
        space >>
        units_locked: locked >>
        space >>
        option_flag: option_flag >>
        line_ending >>
        fields: many0!(component_field) >>
        // TODO: parse fields
        take_until_s!("DRAW") >>
        tag!("DRAW") >>
        line_ending >>
        geometric_elements: many0!(alt!(
            arc_def |
            circle_def |
            pin_def |
            polygon_def |
            rectangle_def |
            text_def
        )) >>
        take_until_s!("ENDDEF") >>
        (Component {
            uuid: Uuid::new_v4(),
            name: component_name.to_owned(),
            reference: reference.to_owned(),
            text_offset: text_offset,
            draw_pin_number: draw_pin_number,
            draw_pin_name: draw_pin_name,
            unit_count: unit_count,
            units_locked: units_locked,
            option_flag: option_flag,
            fields: fields,
            alias: Vec::new(),
            graphic_elements: geometric_elements,
            pins: Vec::new(),
            bounding_box: Cell::new(None)
        })
    )
);

named!(field_tag(CompleteByteSlice) -> isize,
    do_parse!(
        tag_s!("F") >>
        n: int >>
        (n)
    )
);

named!(component_field(CompleteByteSlice) -> (Field),
    do_parse!(
        n: field_tag >>
        space >>
        text: delimited_text >>
        space >>
        position: point >>
        space >>
        dimension: uint >>
        space >>
        orientation: orientation >>
        space >>
        visible: visibility >>
        space >>
        hjustify: justification >>
        space >>
        vjustify: justification >>
        italic: italic >>
        bold: bold >>
        // name: opt!(ws!(utf8_str)) >>
        line_ending >>
        (Field {
            n: n,
            text: text.to_owned(),
            position: position,
            dimension: dimension,
            orientation: orientation,
            visible: visible,
            hjustify: hjustify,
            vjustify: vjustify,
            italic: italic,
            bold: bold,
            name: None // name.map(|s| s.to_owned()),
        })

    )
);

/// Parses a N/P single character to OptionFlag
named!(option_flag(CompleteByteSlice) -> OptionFlag,
    map!(alpha, { |i: CompleteByteSlice| if i.0 == &['P' as u8] { OptionFlag::Power } else { OptionFlag::Normal } })
);

/* P A R S E   G R A P H I C   E L E M E N T S */

/// Parses a arc in a KiCad component.
named!(arc_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("A") >>
        space >>
        pos: point >>
        space >>
        radius: float >>
        space >>
        anglex: int >>
        space >>
        angley: int >>
        space >>
        unit: uint >>
        space >>
        _convert: uint >>
        space >>
        thickness: uint >>
        space >>
        filled: filled >>
        space >>
        start: point >>
        space >>
        end: point >>
        line_ending >>
        (GraphicElement::CircleArc {
            center: pos,
            radius: radius,
            start_coord: start,
            end_coord: end,
            start_angle: anglex,
            end_angle: angley,
            convert: unit,
            unit: unit,
            filled: filled,
            thickness: thickness
        })
    )
);

/// Parses a circle in a KiCad component.
named!(circle_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("C") >>
        space >>
        pos: point >>
        space >>
        radius: float >>
        space >>
        unit: uint >>
        space >>
        _convert: uint >>
        space >>
        thickness: uint >>
        space >>
        filled: filled >>
        line_ending >>
        (GraphicElement::Circle {
            center: pos,
            radius: radius,
            convert: unit,
            unit: unit,
            filled: filled,
            thickness: thickness
        })
    )
);

/// Parses a pin in a KiCad component.
named!(pin_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("X") >>
        space >>
        name: pin_name >>
        space >>
        number: uint >>
        space >>
        pos: point >>
        space >>
        length: uint >>
        space >>
        orientation: pin_orientation >>
        space >>
        snum: uint >>
        space >>
        snom: uint >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        // TODO: etype & shape
        etype: utf8_str >>
        shape: opt!(do_parse!(space >> shape: utf8_str >> (shape))) >>
        line_ending >>
        (GraphicElement::Pin {
            uuid: Uuid::new_v4(),
            orientation: orientation,
            name: name,
            number: number,
            position: pos,
            length: length,
            number_size: snum,
            name_size: snom,
            unit: unit,
            etype: etype.to_owned(),
            shape: shape.map( |s| s.to_owned() ),
            convert: convert,
        })
    )
);

named!(pin_name(CompleteByteSlice) -> Option<String>,
    map!(alt!(
        map_res!(do_parse!(t: tag!("~") >> utf8_str >> (t)), bytes_to_utf8) |
        utf8_str
    ), |s| {
        if s == "~" {
            None
        } else {
            Some(s.to_owned())
        }
    })
);

/// Parses a rectangle in a KiCad component.
named!(rectangle_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("S") >>
        space >>
        start: point >>
        space >>
        end: point >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        _thickness: uint >>
        space >>
        filled: filled >>
        line_ending >>
        (GraphicElement::Rectangle {
            start: start,
            end: end,
            unit: unit,
            convert: convert,
            filled: filled,
        })
    )
);

/// Parses a text in a KiCad component.
named!(text_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("T") >>
        space >>
        orientation: text_orientation >>
        space >>
        pos: point >>
        space >>
        _dimension: uint >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        text: utf8_str >>
        line_ending >>
        (GraphicElement::TextField {
            content: text.to_owned(),
            orientation: orientation,
            position: pos,
            unit: unit,
            convert: convert,
        })
    )
);

/// TODO: Finish implementation
/// Parses a polygon in a KiCad component.
named!(polygon_def(CompleteByteSlice) -> (GraphicElement),
    do_parse!(
        tag!("P") >>
        space >>
        number_points: uint >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        thickness: uint >>
        space >>
        points: count!(
            do_parse!(
                p: point >>
                space >>
                (p)
            ),
            number_points
        ) >>
        filled: filled >>
        line_ending >>
        (GraphicElement::Polygon {
            points: points,
            convert: convert,
            unit: unit,
            thickness: thickness,
            filled: filled,
        })
    )
);

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
        points: Vec<Point2>,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point2,
        end: Point2,
        unit: usize,
        convert: usize,
        filled: bool,
        // TODO: parts, convert, filled
    },
    Circle {
        center: Point2,
        radius: f32,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    CircleArc {
        center: Point2,
        radius: f32,
        start_coord: Point2,
        end_coord: Point2,
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
        position: Point2,
        unit: usize,
        convert: usize
        // TODO: parts, convert, filled
    },
    Pin {
        uuid: Uuid,
        orientation: PinOrientation,
        name: Option<String>,
        number: usize,
        position: Point2,
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

/// Parses a 0/1 single character to TextOrientation
named!(text_orientation(CompleteByteSlice) -> TextOrientation,
    map!(alpha, { |i: CompleteByteSlice| if i.0 == &['0' as u8] { TextOrientation::Horizontal } else { TextOrientation::Vertical } })
);

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
    pub fn unit_vec(&self) -> Vector2 {
        match *self {
            PinOrientation::Up => Vector2::new(0.0, 1.0),
            PinOrientation::Down => Vector2::new(0.0, -1.0),
            PinOrientation::Right => Vector2::new(1.0, 0.0),
            PinOrientation::Left => Vector2::new(-1.0, 0.0),
        }
    }
}

/// Parses a U/D/R/L single character to PinOrientation
named!(pin_orientation(CompleteByteSlice) -> PinOrientation,
    map!(alpha, { |i: CompleteByteSlice| match i.0[0] as char {
        'U' => { PinOrientation::Up },
        'D' => { PinOrientation::Down },
        'R' => { PinOrientation::Right },
         _  => { PinOrientation::Left }
    }})
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_1() {
        use std::io::Cursor;

        let file_data = include_str!("../../test_data/Interface_CurrentLoop.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed_raw = parse_components(&mut file_cursor);

        assert_eq!(1, parsed_raw.unwrap().len());
    }

    #[test]
    fn parse_file_2() {
        use std::io::Cursor;

        let file_data = include_str!("../../test_data/Driver_Display.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_components(&mut file_cursor).unwrap();

        assert_eq!(6, parsed.len());
    }

    #[test]
    fn parse_file_3() {
        use std::io::Cursor;

        let file_data = include_str!("../../test_data/Driver_Motor.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_components(&mut file_cursor).unwrap();

        assert_eq!(23, parsed.len());
    }

    // TODO: allow special characters and detect ""
    // test might be broken for boolean values of the component
    const SAMPLE_DOC: &'static str = r##"DEF +3V3 #PWR 0 0 Y Y 1 F P
F0 "#PWR" 0 -150 50 H I C CNN
F1 "+3V3" 0 140 50 H V C CNN
F2 "K" 0 0 50 H I C CNN
F3 "T" 0 0 50 H I C CNN
ALIAS +3.3V
DRAW
X +3V3 1 0 0 0 U 50 50 1 1 W N
ENDDRAW
ENDDEF
"##;
    /*
    P 2 0 1 0 -30 50 0 100 N
    P 2 0 1 0 0 0 0 100 N
    P 2 0 1 0 0 100 30 50 N
    */
    const SAMPLE_CON: &'static str = r#"#
# 2PScrewConn
#
DEF 2PScrewConn X 0 40 Y Y 1 F N
F0 "X" -150 200 60 H V C CNN
F1 "2PScrewConn" -250 0 60 V V C CNN
F2 "" 0 0 60 H I C CNN
F3 "" 0 0 60 H I C CNN
DRAW
C -100 -75 50 0 1 0 N
C -100 75 50 0 1 0 N
S -200 150 0 -150 0 1 0 N
P 2 0 1 0 -125 -50 -75 -100 N
P 2 0 1 0 -125 100 -75 50 N
P 2 0 1 0 -75 -50 -125 -100 N
P 2 0 1 0 -75 100 -125 50 N
X ~ 1 200 100 200 L 50 50 1 1 I
X ~ 2 200 -100 200 L 50 50 1 1 I
ENDDRAW
ENDDEF
"#;


    #[test]
    fn parse_name() {
        let comp = parse_component(CompleteByteSlice(SAMPLE_DOC.as_bytes())).unwrap();

        assert_eq!("+3V3", comp.name);
        assert_eq!("#PWR", comp.reference);
        assert_eq!(true, comp.draw_pin_number);
        assert_eq!(true, comp.draw_pin_name);
        assert_eq!(1, comp.unit_count);
        assert_eq!(false, comp.units_locked);
        assert_eq!(OptionFlag::Power, comp.option_flag);

        assert_eq!(4, comp.fields.len());
        assert_eq!(comp.graphic_elements.len(), 1)
    }

    #[test]
    fn parse_name_with_comment() {

        let comp = parse_component(CompleteByteSlice(SAMPLE_CON.as_bytes())).unwrap();

        assert_eq!("2PScrewConn", comp.name);
        assert_eq!("X", comp.reference);
        assert_eq!(true, comp.draw_pin_number);
        assert_eq!(true, comp.draw_pin_name);
        assert_eq!(1, comp.unit_count);
        assert_eq!(false, comp.units_locked);
        assert_eq!(OptionFlag::Normal, comp.option_flag);

        assert_eq!(4, comp.fields.len());
        assert_eq!(comp.graphic_elements.len(), 9);
    }

    #[test]
    fn parse_pin_def() {
        let sample = "X ~NAME 1 200 100 200 L 50 50 1 1 I\r\n";

        let (_, pin) = pin_def(CompleteByteSlice(sample.as_bytes())).unwrap();

        match pin {
            GraphicElement::Pin { name, number, length, position, orientation, number_size, name_size, unit, convert, etype, shape } => {
                assert!(name.is_none());
                assert_eq!(number, 1);
                assert_eq!(position.x, 200.0);
                assert_eq!(position.y, 100.0);
                assert_eq!(length, 200);
                assert_eq!(orientation, PinOrientation::Left);
                assert_eq!(number_size, 50);
                assert_eq!(name_size, 50);
                assert_eq!(unit, 1);
                assert_eq!(convert, 1);
                assert_eq!(etype, "I");
                assert_eq!(shape, None);
            },
            _ => panic!("Unexpected parse result")
        }
    }

    #[test]
    fn parse_pin_name() {
        let inputs = [
            ("~ ", None),
            ("A ", Some("A")),
            ("LongName ", Some("LongName")),
            ("+3V3 ", Some("+3V3"))
        ];

        for &(input, expected) in inputs.iter() {
            let (_, parsed) = pin_name(CompleteByteSlice(input.as_bytes())).unwrap();
            assert_eq!(parsed, expected.map( |s| s.to_owned()));
        }
    }

    #[test]
    fn parse_field() {
        let sample = "F0 \"X\" -150 200 60 H V C CNN\r\n";

        let (_, parsed) = component_field(CompleteByteSlice(sample.as_bytes())).unwrap();

        assert_eq!(0, parsed.n);
    }

    #[test]
    fn parse_delimited_text() {
        let inputs = [
            ("\"test\"", "test"),
            ("\"\"", ""),
            ("\"P\"", "P"),
        ];

        for &(input, expected) in inputs.iter() {
            let (_, parsed) = delimited_text(CompleteByteSlice(input.as_bytes())).unwrap();
            assert_eq!(expected, parsed);
        }
    }

    #[test]
    fn parse_rectangle() {
        let sample = "S -400 400 400 -400 0 1 10 f\n";

        let (_, parsed) = rectangle_def(CompleteByteSlice(sample.as_bytes())).unwrap();

        match parsed {
            GraphicElement::Rectangle {start,  .. } => {
                assert_eq!(start.x, -400.0);
                assert_eq!(start.y, 400.0);
            },
            _ => panic!("Unexpected parse result")
        }
    }

    mod bounding_box {
        use std::cell::Cell;
        use ncollide2d::math::Point;
        use geometry::schema_elements::*;
        use geometry::Point2;

        fn build_component() -> Component {
            Component {
                name: "test_component".to_owned(),
                reference: "U".to_owned(),
                text_offset: 10,
                draw_pin_number: true,
                draw_pin_name: true,
                unit_count: 1,
                units_locked: false,
                option_flag: OptionFlag::Normal,
                fields: Vec::new(),
                alias: Vec::new(),
                graphic_elements: Vec::new(),
                pins: Vec::new(),
                bounding_box: Cell::new(None)
            }
        }

        #[test]
        fn rectangle() {
            let mut comp = build_component();

            comp.graphic_elements.push(
                GraphicElement::Rectangle {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(10.0, 10.0),
                    unit: 1,
                    convert: 0,
                    filled: false,
                }
            );

            let bb = comp.get_boundingbox();

            assert_eq!(bb.mins(), &Point::<f32>::new(0.0, 0.0));
            assert_eq!(bb.maxs(), &Point::<f32>::new(10.0, 10.0));
        }

        #[test]
        fn circle() {
            let mut comp = build_component();

            comp.graphic_elements.push(
                GraphicElement::Circle {
                    center: Point2::new(0.0, 0.0),
                    radius: 12.0,
                    unit: 0,
                    convert: 0,
                    thickness: 1,
                    filled: false,
                }
            );

            let bb = comp.get_boundingbox();

            assert_eq!(bb.mins(), &Point::new(-12.0, -12.0));
            assert_eq!(bb.maxs(), &Point::new(12.0, 12.0));
        }

        #[test]
        fn pin() {
            let mut comp = build_component();

            comp.graphic_elements.push(
                GraphicElement::Pin {
                    orientation: PinOrientation::Right,
                    position: Point2::new(0.0, 0.0),
                    name: None,
                    number: 1,
                    length: 15,
                    number_size: 3,
                    name_size: 2,
                    unit: 1,
                    convert: 0,
                    etype: "a_type".to_owned(),
                    shape: None,
                }
            );

            let bb = comp.get_boundingbox();

            assert_eq!(bb.mins(), &Point::new(0.0, 0.0));
            assert_eq!(bb.maxs(), &Point::new(15.0, 0.0));
        }

        #[test]
        fn two_overlapping_rectangles() {
            let mut comp = build_component();

            comp.graphic_elements.push(
                GraphicElement::Rectangle {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(10.0, 10.0),
                    unit: 1,
                    convert: 0,
                    filled: false,
                }
            );

            comp.graphic_elements.push(
                GraphicElement::Rectangle {
                    start: Point2::new(3.0, 3.0),
                    end: Point2::new(15.0, 15.0),
                    unit: 1,
                    convert: 0,
                    filled: false,
                }
            );

            let bb = comp.get_boundingbox();
            assert_eq!(bb.mins(), &Point::new(0.0, 0.0));
            assert_eq!(bb.maxs(), &Point::new(15.0, 15.0));
        }
    }
}