pub mod geometry;

use std::str;
use std::f32;
use std::cell::Cell;

use nom::{alphanumeric, alpha, anychar, space, line_ending, not_line_ending};
use nom::types::{CompleteStr, CompleteByteSlice};

use self::geometry::*;

use common_parsing::{utf8_str, point, bytes_to_utf8, number_str};

use geometry::{SchemaPoint2D};

use ncollide2d::math::{Point};

use helpers::SchemaAABB;

#[derive(Debug, PartialEq, Clone)]
enum OptionFlag {
    Normal,
    Power
}

use helpers::clone_cached_aabb;
#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Component {
    pub name: String,
    reference: String,
    text_offset: isize,
    draw_pin_number: bool,
    draw_pin_name: bool,
    unit_count: isize,
    units_locked: bool,
    option_flag: OptionFlag,
    fields: Vec<Field>,
    alias: Vec<String>,
    graphic_elements: Vec<GraphicElement>,
    pins: Vec<PinDescription>,
    #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
    bounding_box: Cell<Option<SchemaAABB>>
}

impl Component {
    pub fn parse(input: CompleteByteSlice) -> Option<Component> {
        let parse_res = component(input);

        match parse_res {
            Ok((_, o)) => Some(o),
            _ => None
        }
    }

    pub fn get_graphic_elements(&self) -> &Vec<GraphicElement> {
        &self.graphic_elements
    }

    pub fn set_graphic_elements(&mut self, elements: Vec<GraphicElement>) {
        self.graphic_elements = elements;
    }

    pub fn add_graphic_element(&mut self, element: GraphicElement) {
        self.graphic_elements.push(element);
    }

    pub fn add_graphic_elements(&mut self, elements: &mut Vec<GraphicElement>) {
        self.graphic_elements.append(elements);
    }

    pub fn update_boundingbox(&self) {
        let mut max_x = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for shape in &self.graphic_elements {
            match shape {
                &GraphicElement::Rectangle { ref start, ref end, .. } => {
                    max_x = max_x.max(start.x).max(end.x);
                    min_x = min_x.min(start.x).min(end.x);
                    max_y = max_y.max(start.y).max(end.y);
                    min_y = min_y.min(start.y).min(end.y);
                },
                &GraphicElement::Circle { ref center, radius, .. } => {
                    max_x = max_x.max(center.x + radius);
                    min_x = min_x.min(center.x - radius);
                    max_y = max_y.max(center.y + radius);
                    min_y = min_y.min(center.y - radius);
                },
                &GraphicElement::Pin { ref position, ref orientation, length, .. } => {
                    let end_position = position.clone() + (orientation.unit_vec() * (length as f32));
                    max_x = max_x.max(position.x).max(end_position.x );
                    min_x = min_x.min(position.x).min(end_position.x);
                    max_y = max_y.max(position.y).max(end_position.y);
                    min_y = min_y.min(position.y).min(end_position.y);
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

        for field in &self.fields {
            if field.visible {
                max_x = max_x.max(field.position.x + field.dimension as f32 / 2.0);
                min_x = min_x.min(field.position.x - field.dimension as f32 / 2.0);
                max_y = max_y.max(field.position.y + field.dimension as f32 / 2.0);
                min_y = min_y.min(field.position.y - field.dimension as f32 / 2.0);
            }
        }

        if max_x > f32::MIN
        && max_y > f32::MIN
        && min_x < f32::MAX
        && min_y < f32::MAX {
            self.bounding_box.set(Some(SchemaAABB::new(
                Point::new(min_x, min_y),
                Point::new(max_x, max_y)
            )))
        } else {
            self.bounding_box.set(Some(SchemaAABB::new(
                Point::new(0.0, 0.0),
                Point::new(0.0, 0.0)
            )))
        }
    }

    pub fn get_boundingbox(&self) -> SchemaAABB {
        use helpers::CellCopy;
        self.bounding_box.copy().take().unwrap_or_else(|| {
            self.update_boundingbox();
            // Unwrap is always safe as we just calculated a BB
            self.bounding_box.copy().take().unwrap()
        })
    }
}

/// Parses a Y/N value to true/false
named!(yesno(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['Y' as u8]})
);

/// Parses a L/F value to true/false
named!(locked(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['L' as u8]})
);

/// Parses a filled value to true/false
named!(filled(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['F' as u8]})
);


/// Parses a utf8 numberstring value to signed int
named!(int(CompleteByteSlice) -> isize,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a utf8 numberstring value to float
named!(float(CompleteByteSlice) -> f32,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a utf8 numberstring value to unsigned int
named!(uint(CompleteByteSlice) -> usize,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a N/P single character to OptionFlag
named!(option_flag(CompleteByteSlice) -> OptionFlag,
    map!(alpha, { |i: CompleteByteSlice| if i.0 == &['P' as u8] { OptionFlag::Power } else { OptionFlag::Normal } })
);

fn is_number_char(c: u8) -> bool {
    ((c >= '0' as u8) && (c <= '9' as u8)) || c == '-' as u8 || c == '.' as u8
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

/// Parses a 0/1 single character to TextOrientation
named!(text_orientation(CompleteByteSlice) -> TextOrientation,
    map!(alpha, { |i: CompleteByteSlice| if i.0 == &['0' as u8] { TextOrientation::Horizontal } else { TextOrientation::Vertical } })
);

named!(italic(CompleteByteSlice) -> bool, map!(anychar, |c| c == 'I'));

named!(bold(CompleteByteSlice) -> bool, map!(anychar, |c| c == 'B'));

// Parses a Component from start to end
named!(pub component(CompleteByteSlice) -> (Component),
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


#[derive(Debug, Clone)]
pub enum Justify {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl Justify {
    fn from_char(c: char) -> Option<Justify> {
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
pub struct Field {
    n: isize,
    pub text: String,
    pub position: SchemaPoint2D,
    pub dimension: usize,
    pub orientation: TextOrientation,
    pub visible: bool,
    pub hjustify: Justify,
    pub vjustify: Justify,
    italic: bool,
    bold: bool,
    name: Option<String>,
}

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

named!(delimited_text(CompleteByteSlice) -> &str,
    map_res!(delimited!(tag!("\""), take_until!("\""), tag!("\"")), bytes_to_utf8)
);

named!(orientation(CompleteByteSlice) -> TextOrientation, 
    map_opt!(anychar, TextOrientation::from_char)
);

named!(visibility(CompleteByteSlice) -> bool,
    map!(anychar, |c| c == 'V')
);

named!(justification(CompleteByteSlice) -> Justify,
    map_opt!(anychar, Justify::from_char)
);

// Parses an Arc
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

// Parses a Circle
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

// Parses a Pin
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

// Parses a Rectangle
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

// Parses a Text
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

// TODO:
// Parses a Polygon
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

// Eats a comment
named!(comment(CompleteByteSlice) -> (),
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
        let comp = Component::parse(CompleteByteSlice(SAMPLE_DOC.as_bytes())).unwrap();

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
        let comp = Component::parse(CompleteByteSlice(SAMPLE_CON.as_bytes())).unwrap();

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
        use component::{Component, OptionFlag};
        use component::geometry;
        use geometry::SchemaPoint2D;

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
                geometry::GraphicElement::Rectangle {
                    start: SchemaPoint2D::new(0.0, 0.0),
                    end: SchemaPoint2D::new(10.0, 10.0),
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
                geometry::GraphicElement::Circle {
                    center: SchemaPoint2D::new(0.0, 0.0),
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
                geometry::GraphicElement::Pin {
                    orientation: geometry::PinOrientation::Right,
                    position: SchemaPoint2D::new(0.0, 0.0),
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
                geometry::GraphicElement::Rectangle {
                    start: SchemaPoint2D::new(0.0, 0.0),
                    end: SchemaPoint2D::new(10.0, 10.0),
                    unit: 1,
                    convert: 0,
                    filled: false,
                }
            );

            comp.graphic_elements.push(
                geometry::GraphicElement::Rectangle {
                    start: SchemaPoint2D::new(3.0, 3.0),
                    end: SchemaPoint2D::new(15.0, 15.0),
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
