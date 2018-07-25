use nom::{
    space,
    line_ending,
    digit
};
use nom::types::{
    CompleteByteSlice
};

use uuid::Uuid;

use std::str;
use std::cell::Cell;

use geometry::{
    Point2D,
    Vector2D,
    Vector3,
    Matrix4,
    AABB
};
use parsing::common::*;
use parsing::component::*;
use geometry::schema_elements::*;

#[derive(Debug)]
pub struct SchemaFile {
    pub components: Vec<ComponentInstance>,
    pub wires: Vec<WireSegment>,
    pub labels: Vec<Label>,
    junctions: Vec<Junction>,
}

impl SchemaFile {
    pub fn parse(input: &[u8]) -> Option<SchemaFile> {
        let parse_res = schema_file(CompleteByteSlice(input));

        match parse_res {
            Ok((_, entries)) => {
                let mut components = Vec::new();
                let mut wires = Vec::new();
                let mut labels = Vec::new();
                let mut junctions = Vec::new();
                let mut notes = Vec::new();
                let mut no_conns = Vec::new();

                for e in entries.into_iter() {
                    match e {
                        SchemaEntry::ComponentInstance(comp) => components.push(comp),
                        SchemaEntry::Wire(wire) => wires.push(wire),
                        SchemaEntry::Label(label) => labels.push(label),
                        SchemaEntry::Junction(junction) => junctions.push(junction),
                        SchemaEntry::Note(note) => notes.push(note),
                        SchemaEntry::NoConnection(noconn) => no_conns.push(noconn),
                    }
                }

                Some( SchemaFile {
                    components: components,
                    wires: wires,
                    labels: labels,
                    junctions: junctions,
                })
            },
            _ => None
        }
    }
}

named!(schema_file(CompleteByteSlice) -> Vec<SchemaEntry>,
    do_parse!(
        tag_s!("EESchema Schematic File Version") >>
        space >>
        digit >>
        line_ending >>
        take_until_and_consume_s!("$EndDescr") >> line_ending >>
        components: many1!(alt!(
            component_instance | 
            wire_instance | 
            label_entry |
            junction_entry |
            note_entry |
            no_conn_entry
            )) >>
        tag_s!("$EndSCHEMATC") >> line_ending >>
        (components)
    )
);

#[derive(Debug)]
enum SchemaEntry {
    ComponentInstance(ComponentInstance),
    Wire(WireSegment),
    Label(Label),
    Junction(Junction),
    Note(Note),
    NoConnection(NoConnection),
}

use utils::traits::clone_cached_aabb;
#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ComponentInstance {
    pub uuid: Uuid,
    pub name: String,
    pub reference: String,
    pub position: Point2D,
    pub rotation: Matrix4,
    #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
    bounding_box: Cell<Option<AABB>>
}

impl ComponentInstance {
    pub fn new(name: String) -> ComponentInstance {
        ComponentInstance {
            uuid: Uuid::nil(),
            name: name,
            reference: "?".into(),
            position: Point2D::origin(),
            rotation: Matrix4::identity(),
            bounding_box: Cell::new(None)
        }
    }

    pub fn get_boundingbox(&self, component: &Component) -> AABB {
        use utils::traits::Translatable;
        component.get_boundingbox().translated(Vector2D::new(
            self.position.x,
            self.position.y
        ))
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.rotation.append_translation(
            &Vector3::new(
                self.position.x,
                self.position.y,
                0.0
            )
        )
    }
}

named!(field_tag(CompleteByteSlice) -> isize,
    do_parse!(
        tag_s!("F") >>
        space >>
        n: int >>
        (n)
    )
);

named!(field_entry(CompleteByteSlice) -> (Field),
    do_parse!(
        n: field_tag >>
        space >>
        text: delimited_text >>
        space >>
        orientation: orientation >>
        space >>
        position: point >>
        space >>
        dimension: uint >>
        many1!(space) >>
        // Flags for visibility of fields
        many0!(number_str) >>
        space >>
        hjustify: justification >>
        space >>
        vjustify: justification >>
        italic: italic >>
        bold: bold >>
        // name: opt!(ws!(utf8_str)) >>
        take_until_either!("\r\n") >> line_ending >>
        (Field { 
            n: n,
            text: text.to_owned(),
            position: position,
            dimension: dimension,
            orientation: orientation,
            visible: false,
            hjustify: hjustify,
            vjustify: vjustify,
            italic: italic,
            bold: bold,
            name: None // name.map(|s| s.to_owned()),
        })

    )
);

named!(component_rotation(CompleteByteSlice) -> Matrix4, 
    do_parse!(
        char!('\t') >>
        m: ws!(tuple!(
            coordinate, coordinate, coordinate, coordinate
        )) >>
        //take_until_either!("\r\n") >> line_ending >>
        (Matrix4::from_row_slice(&[
            m.0, -m.1, 0.0, 0.0,
            m.2, -m.3, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]).transpose())
    )
);

named!(component_instance(CompleteByteSlice) -> SchemaEntry, 
    do_parse!(
        tag_s!("$Comp") >> line_ending >>
        tag_s!("L") >> space >> name: utf8_str >> space >> reference: utf8_str >> line_ending >>
        tag_s!("U") >> take_until_either!("\r\n") >> line_ending >>
        tag_s!("P") >> space >> position: point >> line_ending >>
        _fields: many0!(field_entry) >>
        take_until_either!("\r\n") >> line_ending >>
        rotation: component_rotation >>
        take_until_and_consume_s!("$EndComp") >> line_ending >>
        (SchemaEntry::ComponentInstance(ComponentInstance {
            uuid: Uuid::nil(),
            name: name.to_owned(),
            reference: reference.to_owned(),
            position: Point2D::new(position.x, -position.y),
            bounding_box: Cell::new(None),
            rotation: rotation
        }))
    )
);

named!(wire_instance(CompleteByteSlice) -> SchemaEntry,
    do_parse!(
        tag_s!("Wire") >> space >>
        wire: alt!(
            wire_segment |
            bus_segment |
            line_segment
        ) >>
        (SchemaEntry::Wire(wire))
    )
);

named!(wire_segment(CompleteByteSlice) -> WireSegment,
    do_parse!(
        tag_s!("Wire") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> line_ending >>
        (WireSegment {
            uuid: Uuid::nil(),
            kind: WireType::Wire,
            start: Point2D::new(start.x, -start.y),
            end: Point2D::new(end.x, -end.y),
        })
    )
);

named!(bus_segment(CompleteByteSlice) -> WireSegment,
    do_parse!(
        tag_s!("Bus") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> line_ending >>
        (WireSegment {
            uuid: Uuid::nil(),
            kind: WireType::Bus,
            start: Point2D::new(start.x, -start.y),
            end: Point2D::new(end.x, -end.y),
        })
    )
);

named!(line_segment(CompleteByteSlice) -> WireSegment,
    do_parse!(
        tag_s!("Notes") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> opt!(space) >> line_ending >>
        (WireSegment {
            uuid: Uuid::nil(),
            kind: WireType::Dotted,
            start: Point2D::new(start.x, -start.y),
            end: Point2D::new(end.x, -end.y),
        })
    )
);

named!(whole_line_str(CompleteByteSlice) -> &str,
    map_res!(
        do_parse!(
            text: take_until_either!(" \r\n") >>
            line_ending >>
            (text)
        ),
        bytes_to_utf8
    )
);

named!(label_entry(CompleteByteSlice) -> SchemaEntry,
    do_parse!(
        tag_s!("Text") >> space >> tag_s!("Label") >> space >> position: point >> space >> _orientation: digit >> space >>
        _dimension: utf8_str >> space >> tag_s!("~") >> space >> utf8_str >> line_ending >>
        text: whole_line_str >>
        (SchemaEntry::Label(Label {
            text: text.to_owned(),
            position: Point2D::new(position.x, -position.y),
        }))
    )
);

named!(note_entry(CompleteByteSlice) -> SchemaEntry,
    do_parse!(
        tag_s!("Text") >> space >> tag_s!("Notes") >> take_until_either!("\r\n") >> line_ending >>
        take_until_either!("\r\n") >> line_ending >>
        (SchemaEntry::Note(Note {
            text: "".to_owned(),
        }))
    )
);

named!(junction_entry(CompleteByteSlice) -> SchemaEntry,
    do_parse!(
        tag_s!("Connection") >> space >> tag_s!("~") >> space >> position: point >> line_ending >>
        (SchemaEntry::Junction(Junction { position: Point2D::new(position.x, -position.y) }))
    )
);

named!(no_conn_entry(CompleteByteSlice) -> SchemaEntry,
    do_parse!(
        tag_s!("NoConn") >> space >> tag_s!("~") >> space >> position: point >> line_ending >>
        (SchemaEntry::NoConnection( NoConnection { position: Point2D::new(position.x, -position.y) } ))
    )
);

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_COMPONENT: &'static str = r##"$Comp
L GND #PWR?
U 1 1 558C20D6
P 4950 2600
F 0 "#PWR?" H 4950 2350 50  0001 C CNN
F 1 "GND" H 4950 2450 50  0000 C CNN
F 2 "" H 4950 2600 60  0000 C CNN
F 3 "" H 4950 2600 60  0000 C CNN
	1    4950 2600
	1    0    0    -1  
$EndComp
"##;

    const SAMPLE_WIRE: &'static str = r#"Wire Wire Line
3300 1800 3900 1800
"#;

const SAMPLE_SCHEMA_FILE: &'static str = r#"EESchema Schematic File Version 3
LIBS:PSU-rescue
LIBS:bourns
LIBS:buydisplay
LIBS:cirrus
LIBS:cui
LIBS:fairchild
LIBS:linear_tech
LIBS:micrel
LIBS:onsemi
LIBS:wurth
LIBS:antennas
LIBS:PSU-cache
EELAYER 26 0
EELAYER END
$Descr A3 16535 11693
encoding utf-8
Sheet 1 1
Title "PSU"
Date "2017-10-05"
Rev "V2"
Comp "Noah Huesser / yatekii@yatekii.ch"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Notes 8050 10900 0    276  Italic 55
Mesh Node\nr3 autumn 2017\nby yatekii
Wire Wire Line
	7150 3950 7300 3950
Connection ~ 7150 4150
Connection ~ 7750 4350
Wire Wire Line
	10450 4650 10450 4700
Wire Wire Line
	3500 9450 1700 9450
$EndSCHEMATC
"#;


    const SAMPLE_LABEL: &'static str = r#"Text Label 15250 1100 2    60   ~ 0
LED1
"#;

    fn parse_cmp() -> ComponentInstance {
        let (_, cmp) = component_instance(CompleteByteSlice(SAMPLE_COMPONENT.as_bytes())).unwrap();
        if let SchemaEntry::ComponentInstance(cmp) = cmp {
            cmp
        } else {
            panic!("Unexpected return value returned from parser!")
        }
    }

    #[test]
    fn parse_component_name() {
        let cmp = parse_cmp();

        assert_eq!(cmp.name, "GND");
    }

    #[test]
    fn parse_reference() {
        let cmp = parse_cmp();

        assert_eq!(cmp.reference, "#PWR?");
    }

    #[test]
    fn parse_position() {
        let cmp = parse_cmp();

        assert_eq!(cmp.position, Point2D::new(4950.0, -2600.0));
    }

    #[test]
    fn parse_wire() {
        let (_, wire) = wire_instance(CompleteByteSlice(SAMPLE_WIRE.as_bytes())).unwrap();

        if let SchemaEntry::Wire(wire) = wire {
            assert_eq!(wire.kind, WireType::Wire);
            assert_eq!(wire.start, Point2D::new(3300.0, -1800.0));
            assert_eq!(wire.end,   Point2D::new(3900.0, -1800.0));
        } else {
            panic!("Unexpected SchemaEntry type returned from parser!");
        }
    }

    #[test]
    fn parse_file() {
        let file = SchemaFile::parse(SAMPLE_SCHEMA_FILE.as_bytes()).unwrap();

        assert_eq!(file.components.len(), 0);
    }

    #[test]
    fn parse_label() {
        let (_, label) = label_entry(CompleteByteSlice(SAMPLE_LABEL.as_bytes())).unwrap();

        if let SchemaEntry::Label(_label) = label {
            // do nothing... (tbd!)
        } else {
            panic!("Unexpected SchemaEntry type returned from parser!");
        }
    }
}