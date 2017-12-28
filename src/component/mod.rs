pub mod geometry;

use std::str;

use nom::{alphanumeric, alpha, anychar, space, line_ending, not_line_ending};
use nom::IResult::Done;

use self::geometry::*;

#[derive(Debug, PartialEq)]
enum OptionFlag {
    Normal,
    Power
}

#[derive(Debug)]
pub struct Component {
    name: String,
    reference: String,
    text_offset: isize,
    draw_pin_number: bool,
    draw_pin_name: bool,
    unit_count: isize,
    units_locked: bool,
    option_flag: OptionFlag,
    fields: Vec<Field>,
    alias: Vec<String>,
    pub graphic_elements: Vec<GraphicElement>,
    pins: Vec<PinDescription>
}

impl Component {
    pub fn parse(input: &[u8]) -> Option<Component> {
        let parse_res = component(input);

        println!("Parse result: {:#?}", parse_res);

        match parse_res {
            Done(_, o) => Some(o),
            _ => None
        }
    }

    pub fn get_boundingbox(&self) -> (Point, Point) {
        let mut max_x = geometry::CoordType::min_value();
        let mut min_x = geometry::CoordType::max_value();
        let mut max_y = geometry::CoordType::min_value();
        let mut min_y = geometry::CoordType::max_value();
        for shape in &self.graphic_elements {
            match shape {
                &GraphicElement::Rectangle { ref start, ref end, .. } => {
                    max_x = max_x.max(start.x).max(end.x);
                    min_x = min_x.min(start.x).min(end.x);
                    max_y = max_y.max(start.y).max(end.y);
                    min_y = min_y.min(start.y).min(end.y);
                },
                &GraphicElement::Circle { ref center, radius, .. } => {
                    max_x = max_x.max(center.x + radius as isize);
                    min_x = min_x.min(center.x - radius as isize);
                    max_y = max_y.max(center.y + radius as isize);
                    min_y = min_y.min(center.y - radius as isize);
                },
                &GraphicElement::Pin { ref position, ref orientation, length, .. } => {
                    max_x = max_x.max(position.x).max(position.x + orientation.unit_vec()[0] as isize * length as isize);
                    min_x = min_x.min(position.x).min(position.x + orientation.unit_vec()[0] as isize * length as isize);
                    max_y = max_y.max(position.y).max(position.y + orientation.unit_vec()[1] as isize * length as isize);
                    min_y = min_y.min(position.y).min(position.y + orientation.unit_vec()[1] as isize * length as isize);
                },
                &GraphicElement::Polygon { ref points, .. } => {
                    for p in points {
                        max_x = max_x.max(p.x);
                        min_x = min_x.min(p.x);
                        max_y = max_y.max(p.y);
                        min_y = min_y.min(p.y);
                    }
                }
                _ => ()
            }
        }
        if max_x > geometry::CoordType::min_value()
        && max_y > geometry::CoordType::min_value()
        && min_x < geometry::CoordType::max_value()
        && min_y < geometry::CoordType::max_value() {
            (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
        } else {
            (Point { x: 0, y: 0 }, Point { x: 0, y: 0 })
        }
    }
}

/// Parses a Y/N value to true/false
named!(yesno(&[u8]) -> bool,
    map!(alpha, {|c| c == &['Y' as u8]})
);

/// Parses a L/F value to true/false
named!(locked(&[u8]) -> bool,
    map!(alpha, {|c| c == &['L' as u8]})
);

/// Parses a filled value to true/false
named!(filled(&[u8]) -> bool,
    map!(alpha, {|c| c == &['F' as u8]})
);

/// Parses a utf8 string value
named!(utf8_str_a(&[u8]) -> &str,
    map_res!(alphanumeric, str::from_utf8)
);

/// Parses a general utf8 string
named!(utf8_str(&[u8]) -> &str,
    map_res!(
        take_until_either!(" \r\n"),
        str::from_utf8
    )
);

/// Parses a utf8 numberstring value to signed int
named!(int(&[u8]) -> isize,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a utf8 numberstring value to unsigned int
named!(uint(&[u8]) -> usize,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a N/P single character to OptionFlag
named!(option_flag(&[u8]) -> OptionFlag,
    map!(alpha, { |i| if i == &['P' as u8] { OptionFlag::Power } else { OptionFlag::Normal } })
);

named!(number_str<&str>,
    map_res!(take_while!(is_number_char), str::from_utf8)
);

fn is_number_char(c: u8) -> bool {
    ((c >= '0' as u8) && (c <= '9' as u8)) || c == '-' as u8 || c == '.' as u8
}

/// Parses a U/D/R/L single character to PinOrientation
named!(pin_orientation(&[u8]) -> PinOrientation,
    map!(alpha, { |i: &[u8]| match i[0] as char {
        'U' => { PinOrientation::Up },
        'D' => { PinOrientation::Down },
        'R' => { PinOrientation::Right },
         _  => { PinOrientation::Left }
    }})
);

/// Parses a 0/1 single character to TextOrientation
named!(text_orientation(&[u8]) -> TextOrientation,
    map!(alpha, { |i| if i == &['0' as u8] { TextOrientation::Horizontal } else { TextOrientation::Vertical } })
);

named!(italic<bool>, map!(anychar, |c| c == 'I'));

named!(bold<bool>, map!(anychar, |c| c == 'B'));

// Parses a Component from start to end
named!(pub component(&[u8]) -> (Component),
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

// Parses the body of a Component
named!(component_def(&[u8]) -> (Component),
    do_parse!(
        space >>
        component_name: utf8_str >>
        space >>
        reference: utf8_str >>
        space >>
        unused: alphanumeric >>
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
            pins: Vec::new()
        })
    )
);


#[derive(Debug)]
enum Justify {
    Left,
    Right,
    Top,
    Bottom,
    Centre,
}

impl Justify {
    fn from_char(c: char) -> Option<Justify> {
        match c {
            'L' => Some(Justify::Left),
            'R' => Some(Justify::Right),
            'T' => Some(Justify::Top),
            'B' => Some(Justify::Bottom),
            'C' => Some(Justify::Centre),
            _   => None,
        }
    }
}

#[derive(Debug)]
struct Field {
    n: isize,
    text: String,
    position: Point,
    dimension: f32,
    orientation: TextOrientation,
    visible: bool,
    hjustify: Justify,
    vjustify: Justify,
    italic: bool,
    bold: bool,
    name: Option<String>,
}

named!(field_tag<isize>,
    do_parse!(
        tag_s!("F") >>
        n: int >>
        (n)
    )
);

named!(signed_number<f32>,
    map_res!(
        map_res!(
            take_while!(is_number_char),
            str::from_utf8
        ),
        |s: &str| { s.parse() } 
    )
);

named!(component_field(&[u8]) -> (Field),
    do_parse!(
        n: field_tag >>
        space >>
        text: delimited_text >>
        space >>
        position: point >>
        space >>
        dimension: signed_number >>
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

named!(delimited_text<&str>,
    map_res!(delimited!(tag!("\""), take_until!("\""), tag!("\"")), str::from_utf8)
);

named!(orientation(&[u8]) -> TextOrientation, 
    map_opt!(anychar, TextOrientation::from_char)
);

named!(visibility<bool>,
    map!(anychar, |c| c == 'V')
);

named!(justification<Justify>,
    map_opt!(anychar, Justify::from_char)
);

// Parses an Arc
named!(arc_def(&[u8]) -> (GraphicElement),
    do_parse!(
        tag!("A") >>
        space >>
        posx: int >>
        space >>
        posy: int >>
        space >>
        radius: uint >>
        space >>
        anglex: int >>
        space >>
        angley: int >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        thickness: uint >>
        space >>
        filled: filled >>
        space >>
        startx: int >>
        space >>
        starty: int >>
        space >>
        endx: int >>
        space >>
        endy: int >>
        line_ending >>
        (GraphicElement::CircleArc {
            center: Point { x: posx, y:posy },
            radius: radius,
            start_coord: Point { x: startx, y: starty },
            end_coord: Point { x: startx, y: starty },
            start_angle: anglex,
            end_angle: angley,
            convert: unit,
            unit: unit,
            filled: filled,
            thickness: thickness
        })
    )
);

// Parses a Circle
named!(circle_def(&[u8]) -> (GraphicElement),
    do_parse!(
        tag!("C") >>
        space >>
        posx: int >>
        space >>
        posy: int >>
        space >>
        radius: uint >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        thickness: uint >>
        space >>
        filled: filled >>
        line_ending >>
        (GraphicElement::Circle {
            center: Point { x: posx, y:posy },
            radius: radius,
            convert: unit,
            unit: unit,
            filled: filled,
            thickness: thickness
        })
    )
);


named!(pin_name<Option<String>>,
    map!(alt!(
        map_res!(do_parse!(t: tag!("~") >> utf8_str >> (t)), str::from_utf8) |
        utf8_str
    ), |s| {
        if s == "~" {
            None
        } else {
            Some(s.to_owned())
        }
    })
);

// Parses a Pin
named!(pin_def(&[u8]) -> (GraphicElement),
    do_parse!(
        tag!("X") >>
        space >>
        name: pin_name >>
        space >>
        number: uint >>
        space >>
        posx: int >>
        space >>
        posy: int >>
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
            orientation: orientation,
            name: name,
            number: number,
            position: Point { x: posx, y:posy },
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

// Parses a Rectangle
named!(rectangle_def(&[u8]) -> (GraphicElement),
    do_parse!(
        tag!("S") >>
        space >>
        startx: int >>
        space >>
        starty: int >>
        space >>
        endx: int >>
        space >>
        endy: int >>
        space >>
        unit: uint >>
        space >>
        convert: uint >>
        space >>
        thickness: uint >>
        space >>
        filled: filled >>
        line_ending >>
        (GraphicElement::Rectangle {
            start: Point { x: startx, y:starty },
            end: Point { x: endx, y:endy },
            unit: unit,
            convert: convert,
        })
    )
);

// Parses a Text
named!(text_def(&[u8]) -> (GraphicElement),
    do_parse!(
        tag!("T") >>
        space >>
        orientation: text_orientation >>
        space >>
        posx: int >>
        space >>
        posy: int >>
        space >>
        dimension: uint >>
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
            position: Point { x: posx, y:posy },
            unit: unit,
            convert: convert,
        })
    )
);

named!(point<Point>,
    do_parse!(
        x: int >>
        space >>
        y: int >>
        (Point{ x: x, y: y})
    )
);

// TODO:
// Parses a Polygon
named!(polygon_def(&[u8]) -> (GraphicElement),
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
            thickness: thickness
        })
    )
);

// Eats a comment
named!(comment(&[u8]) -> (),
    do_parse!(
        tag!("#") >>
        not_line_ending >>
        line_ending >>
        ()
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: allow special characters and detect ""
    //       test might be broken for boolean values of the component
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
        let comp = Component::parse(SAMPLE_DOC.as_bytes()).unwrap();

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
        let comp = Component::parse(SAMPLE_CON.as_bytes()).unwrap();

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

        let (_, pin) = pin_def(sample.as_bytes()).unwrap();

        match pin {
            GraphicElement::Pin { name, number, length, position, orientation, number_size, name_size, unit, convert, etype, shape } => {
                assert!(name.is_none());
                assert_eq!(number, 1);
                assert_eq!(position.x, 200);
                assert_eq!(position.y, 100);
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
            let (_, parsed) = pin_name(input.as_bytes()).unwrap();
            assert_eq!(parsed, expected.map( |s| s.to_owned()));
        }
    }

     #[test]
    fn parse_field() {
        let sample = "F0 \"X\" -150 200 60 H V C CNN\r\n";

        let (_, parsed) = component_field(sample.as_bytes()).unwrap();

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
            let (_, parsed) = delimited_text(input.as_bytes()).unwrap();
            assert_eq!(expected, parsed);
        }
    }

    #[test]
    fn parse_rectangle() {
        let sample = "S -400 400 400 -400 0 1 10 f\n";

        let (_, parsed) = rectangle_def(sample.as_bytes()).unwrap();

        match parsed {
            GraphicElement::Rectangle {start,  .. } => {
                assert_eq!(start.x, -400);
                assert_eq!(start.y, 400);
            },
            _ => panic!("Unexpected parse result")
        }
    }

    mod bounding_box {

        use component::{Component, OptionFlag};
        use component::geometry;

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
            }
        }

        #[test]
        fn rectangle() {
            let mut comp = build_component();

            let lower_left = geometry::Point { x: 0, y: 0 };
            let upper_right = geometry::Point { x: 10, y: 10 };

            comp.graphic_elements.push(
                geometry::GraphicElement::Rectangle {
                    start: geometry::Point { x: 0, y: 0 },
                    end: geometry::Point { x: 10, y: 10 },
                    unit: 1,
                    convert: 0,
                }
            );

            let bb = comp.get_boundingbox();

            assert_eq!(bb.0, lower_left);
            assert_eq!(bb.1, upper_right);
        }

        #[test]
        fn circle() {
            let mut comp = build_component();

            let lower_left = geometry::Point { x: 0, y: 0 };
            let upper_right = geometry::Point { x: 10, y: 10 };

            comp.graphic_elements.push(
                geometry::GraphicElement::Circle {
                    center: geometry::Point { x: 0, y: 0 },
                    radius: 12,
                    unit: 0,
                    convert: 0,
                    thickness: 1,
                    filled: false,
                }
            );

            let bb = comp.get_boundingbox();

            assert_eq!(bb.0, geometry::Point { x: -12, y: -12 });
            assert_eq!(bb.1, geometry::Point { x: 12, y: 12 });
        }

        #[test]
        fn pin() {
            let mut comp = build_component();

            let lower_left = geometry::Point { x: 0, y: 0 };
            let upper_right = geometry::Point { x: 10, y: 10 };

            comp.graphic_elements.push(
                geometry::GraphicElement::Pin {
                    orientation: geometry::PinOrientation::Right,
                    position: geometry::Point { x: 0, y: 0},
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

            assert_eq!(bb.0, geometry::Point { x: 0, y: 0 });
            assert_eq!(bb.1, geometry::Point { x: 15, y: 0 });
        }

        #[test]
        fn two_overlapping_rectangles() {
            let mut comp = build_component();

            comp.graphic_elements.push(
                geometry::GraphicElement::Rectangle {
                    start: geometry::Point { x: 0, y: 0 },
                    end: geometry::Point { x: 10, y: 10 },
                    unit: 1,
                    convert: 0,
                }
            );

            comp.graphic_elements.push(
                geometry::GraphicElement::Rectangle {
                    start: geometry::Point { x: 3, y: 3 },
                    end: geometry::Point { x: 15, y: 15 },
                    unit: 1,
                    convert: 0,
                }
            );

            let bb = comp.get_boundingbox();
            assert_eq!(bb.0, geometry::Point { x: 0, y: 0});
            assert_eq!(bb.1, geometry::Point { x: 15, y: 15});
        }
    }
}
