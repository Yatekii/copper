use nom::{alphanumeric, alpha, digit, space, line_ending, not_line_ending};

use std::str;

struct Component {
    name: String,
    reference: String,
    draw_pin_number: bool,
    draw_pin_name: bool, 
    unit_count: isize,
    units_locked: bool,
    option_flag: OptionFlag,
    alias: Vec<String>,
    graphic_elements: Vec<GraphicElement>,
    pins: Vec<PinDescription>
}

impl Component {
    fn parse(input: &[u8]) -> Option<Component> {
        let parse_res = component(input);

        println!("Parse result: {:#?}", parse_res);

        let name = parse_res.unwrap();


        Some(Component {
            name: ((name.1).0).to_owned(),
            reference: ((name.1).1).to_owned(),
            draw_pin_number: (name.1).3,
            draw_pin_name: (name.1).4,
            unit_count: (name.1).5,
            units_locked: (name.1).6,
            option_flag: OptionFlag::Normal,
            alias: Vec::new(),
            graphic_elements: Vec::new(),
            pins: Vec::new(),
        })
    }
}

named!(yesno(&[u8]) -> bool, 
    map!(alpha, {|c| c == &['Y' as u8]})
);

named!(locked(&[u8]) -> bool, 
    map!(alpha, {|c| c == &['L' as u8]})
);

named!(utf8_str(&[u8]) -> &str,
    map_res!(alphanumeric, str::from_utf8)
);

named!(int(&[u8]) -> isize,
    map_res!(utf8_str, { |i: &str| i.parse() })
);


named!(option_flag(&[u8]) -> OptionFlag,
    map!(alpha, { |i| if i == &['P' as u8] { OptionFlag::Power } else { OptionFlag::Normal } })
);

named!(component_def(&[u8]) -> (&str, &str, isize, bool, bool, isize, bool, OptionFlag), 
    do_parse!(
        space   >>
        component_name: utf8_str >>
        space >>
        reference: utf8_str >>
        space >>
        unused: alphanumeric >> 
        space >>
        text_offset: int >>
        space >>
        draw_pinnumber: yesno >> 
        space >>
        draw_pinname: yesno >>
        space >>
        unit_count: int >>
        space >>
        units_locked: locked >>
        space >>
        option_flag: option_flag >>
        line_ending >>
        take_until_s!("ENDDEF") >>
        (component_name, reference, text_offset, draw_pinnumber, draw_pinname, unit_count, units_locked, option_flag)
    )
);


named!(component(&[u8]) -> (&str, &str, isize, bool, bool, isize, bool, OptionFlag),
    do_parse!(
        many0!(comment) >>
        component_struct: delimited!(
            tag!("DEF"),
            component_def,
            tag!("ENDDEF")
        ) >>
        (component_struct)
    )
);

named!(comment(&[u8]) -> (),
    do_parse!(
        tag!("#") >>
        not_line_ending >>
        line_ending >>
        ()
    )
);

struct PinDescription {
    orientation: PinOrientation,
    name: Option<String>,
    number: u16,
    // Todo: Complete...
}

enum PinOrientation {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
enum OptionFlag {
    Normal,
    Power
}

type CoordType = isize;

struct Point {
    x: CoordType,
    y: CoordType,
}

enum GraphicElement {
    Polygon {
        points: Vec<Point>,
        thickness: usize,
        // Todo: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point,
        end: Point,
        // Todo: parts, convert, filled
    },
    Circle {
        center: Point,
        radius: CoordType,
        // Todo: parts, convert, filled
    },
    CircleArc {
        center: Point,
        start_coord: Point,
        end_coord: Point,
        start_angle: isize,
        end_angle: isize,
        // Todo: parts, convert, filled
    },
    TextField {
        content: String,
        orientation: TextOrientation,
        position: Point,
        // Todo: parts, convert, filled
    }
}

enum TextOrientation {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DOC: &'static str = r#"DEF BNC P 0 40 Y NR 1 L NR
F0 “P” 10.120 60 H V L C
F1 “BNC” 110 - 60 40 V V L C
DRAW
C 0 0 70 0 1 0
C 0 0 20 0 1 0
X Ext. 2 0 - 200 130 U 40 40 1 1 P
X In 1 - 150 0.130 R 40 40 1 1 P
ENDDRAW
ENDDEF
"#;

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

        assert_eq!("BNC", comp.name);
        assert_eq!("P", comp.reference);
        assert_eq!(true, comp.draw_pin_number);
        assert_eq!(false, comp.draw_pin_name);
        assert_eq!(1, comp.unit_count);
        assert_eq!(true, comp.units_locked);
        assert_eq!(OptionFlag::Normal, comp.option_flag);
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
    }
}