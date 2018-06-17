use helpers::SchemaAABB;
use nom::{space, line_ending, digit};

use ::common_parsing::{utf8_str, point};
use std::str;
use std::cell::Cell;

use geometry::SchemaPoint2D;
use ::component;

use ncollide2d::math::{Point, Vector};

#[derive(Debug)]
pub struct SchemaFile {
    pub components: Vec<ComponentInstance>,
    pub wires: Vec<WireSegment>,
    pub labels: Vec<Label>,
    junctions: Vec<Junction>,
}

impl SchemaFile {
    pub fn parse(input: &[u8]) -> Option<SchemaFile> {
        let parse_res = schema_file(input);

        // println!("Parse result: {:#?}", parse_res);

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

named!(schema_file< Vec<SchemaEntry> >,
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

use helpers::clone_cached_aabb;
#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ComponentInstance {
    pub name: String,
    pub reference: String,
    pub position: SchemaPoint2D,
    pub component: Option<component::Component>,
    #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
    bounding_box: Cell<Option<SchemaAABB>>
}


impl ComponentInstance {
    pub fn update_boundingbox(&self) {
        use helpers::Translatable;
        self.bounding_box.set(Some(self.component.as_ref().map_or(
            SchemaAABB::new(
                Point::new(0.0, 0.0),
                Point::new(0.0, 0.0)
            ),
            |c| c.get_boundingbox().translated(Vector::new(
                self.position.x.clone(),
                self.position.y.clone()
            ))
        )));
    }

    pub fn get_boundingbox(&mut self) -> SchemaAABB {
        use helpers::CellCopy;
        self.bounding_box.copy().take().unwrap_or_else(|| {
            self.update_boundingbox();
            // Unwrap is always safe as we just calculated a BB
            self.bounding_box.copy().take().unwrap()
        })
    }
}

named!(component_instance<SchemaEntry>, 
    do_parse!(
        tag_s!("$Comp") >> line_ending >>
        tag_s!("L") >> space >> name: utf8_str >> space >> reference: utf8_str >> line_ending >>
        tag_s!("U") >> take_until_either!("\r\n") >> line_ending >>
        tag_s!("P") >> space >> position: point >> line_ending >>
        take_until_and_consume_s!("$EndComp") >> line_ending >>
        (SchemaEntry::ComponentInstance(ComponentInstance {
            name: name.to_owned(),
            reference: reference.to_owned(),
            position: SchemaPoint2D::new(position.x, -position.y),
            bounding_box: Cell::new(None),
            component: None
        }))
    )
);

#[derive(Debug)]
pub struct WireSegment {
    pub kind: WireType,
    pub start: SchemaPoint2D,
    pub end: SchemaPoint2D,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WireType {
    Wire,
    Bus,
    Dotted
}

named!(wire_instance<SchemaEntry>,
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

named!(wire_segment<WireSegment>,
    do_parse!(
        tag_s!("Wire") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> line_ending >>
        (WireSegment {
            kind: WireType::Wire,
            start: start,
            end: end,
        })
    )
);

named!(bus_segment<WireSegment>,
    do_parse!(
        tag_s!("Bus") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> line_ending >>
        (WireSegment {
            kind: WireType::Bus,
            start: start,
            end: end,
        })
    )
);

named!(line_segment<WireSegment>,
    do_parse!(
        tag_s!("Notes") >> space >> tag_s!("Line") >> line_ending >>
        opt!(space) >> start: point >> space >> end: point >> opt!(space) >> line_ending >>
        (WireSegment {
            kind: WireType::Dotted,
            start: start,
            end: end,
        })
    )
);

named!(whole_line_str<&str>,
    map_res!(
        do_parse!(
            text: take_until_either!(" \r\n") >>
            line_ending >>
            (text)
        ),
        str::from_utf8
    )
);

#[derive(Debug)]
pub struct Label {
    text: String,
    position: SchemaPoint2D,
    //todo: fill
}

named!(label_entry<SchemaEntry>,
    do_parse!(
        tag_s!("Text") >> space >> tag_s!("Label") >> space >> position: point >> space >> _orientation: digit >> space >>
        _dimension: utf8_str >> space >> tag_s!("~") >> space >> utf8_str >> line_ending >>
        text: whole_line_str >>
        (SchemaEntry::Label(Label {
            text: text.to_owned(),
            position: position,
        }))
    )
);

#[derive(Debug)]
pub struct Note {
    text: String,
    //todo: fill
}

named!(note_entry<SchemaEntry>,
    do_parse!(
        tag_s!("Text") >> space >> tag_s!("Notes") >> take_until_either!("\r\n") >> line_ending >>
        take_until_either!("\r\n") >> line_ending >>
        (SchemaEntry::Note(Note {
            text: "".to_owned(),
        }))
    )
);

#[derive(Debug)]
struct Junction {
    position: SchemaPoint2D,
}

named!(junction_entry<SchemaEntry>,
    do_parse!(
        tag_s!("Connection") >> space >> tag_s!("~") >> space >> pos: point >> line_ending >>
        (SchemaEntry::Junction(Junction { position: pos }))
    )
);

#[derive(Debug)]
struct NoConnection {
    position: SchemaPoint2D,
}

named!(no_conn_entry<SchemaEntry>,
    do_parse!(
        tag_s!("NoConn") >> space >> tag_s!("~") >> space >> pos: point >> line_ending >>
        (SchemaEntry::NoConnection( NoConnection { position: pos } ))
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
        let (_, cmp) = component_instance(SAMPLE_COMPONENT.as_bytes()).unwrap();

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

        assert_eq!(cmp.position, SchemaPoint2D::new(4950.0, 2600.0));
    }

    #[test]
    fn parse_wire() {
        let (_, wire) = wire_instance(SAMPLE_WIRE.as_bytes()).unwrap();

        if let SchemaEntry::Wire(wire) = wire {
            assert_eq!(wire.kind, WireType::Wire);
            assert_eq!(wire.start, SchemaPoint2D::new(3300.0, 1800.0));
            assert_eq!(wire.end,   SchemaPoint2D::new(3900.0, 1800.0));
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
        let (_, label) = label_entry(SAMPLE_LABEL.as_bytes()).unwrap();

        if let SchemaEntry::Label(_label) = label {
            // do nothing... (tbd!)
        } else {
            panic!("Unexpected SchemaEntry type returned from parser!");
        }
    }
}