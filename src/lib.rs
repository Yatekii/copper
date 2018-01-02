#![recursion_limit="128"]

#[macro_use]
extern crate nom;

extern crate euclid;

pub mod component;
pub mod schema_file;
mod common_parsing;

use component::Component;
use schema_file::ComponentInstance;
use std::io::Read;

use nom::{line_ending, space, digit};

pub fn parse_components<R: Read>(data: &mut R) -> Option<Vec<Component>> {
    let mut buff: Vec<u8> = Vec::new();

    if let Ok(_) = data.read_to_end(&mut buff) {
        let parse_raw = component_file(&buff);

        if let nom::IResult::Done(_, components) = parse_raw {
            Some(components)
        } else {
            println!("Error reading from file: {:#?}", parse_raw);
            None
        }
    } else {
        None
    }
}

named!(component_file< Vec<Component> >,
    do_parse!(
        tag_s!("EESchema-LIBRARY Version") >>
        space >>
        digit >>
        tag_s!(".") >>
        digit >>
        line_ending >>
        components: many1!(component::component) >>
        (components)
    )
);

pub fn parse_schema<R: Read>(data: &mut R) -> Option<Vec<ComponentInstance>> {
    let mut buff: Vec<u8> = Vec::new();

    if let Ok(_) = data.read_to_end(&mut buff) {
        let parse_raw = schema_file(&buff);

        match parse_raw {
            nom::IResult::Done(_, components) => Some(components),
            nom::IResult::Error(e) => { println!("{:?}", e); None },
            nom::IResult::Incomplete(needed) => { println!("{:?}", needed); None }
        }
        //  else {
        //     println!("Error reading from file: {:#?}", parse_raw);
        //     None
        // }
    } else {
        None
    }
}

named!(schema_file< Vec<ComponentInstance> >,
    do_parse!(
        tag_s!("EESchema Schematic File Version") >>
        space >>
        digit >>
        line_ending >>
        components: many1!(component_instance_only) >>
        (components)
    )
);

named!(pub component_instance_only<ComponentInstance>, 
    do_parse!(
        take_until!("$Comp") >>
        component: map!(schema_file::component_instance, |c| c) >>
        (component)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_1() {
        use std::io::Cursor;

        let file_data = include_str!("../test_data/Interface_CurrentLoop.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed_raw = parse_components(&mut file_cursor);
        
        println!("{:#?}", parsed_raw);

        assert_eq!(1, parsed_raw.unwrap().len());
    }

    #[test]
    fn parse_file_2() {
        use std::io::Cursor;

        let file_data = include_str!("../test_data/Driver_Display.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_components(&mut file_cursor).unwrap();

        assert_eq!(6, parsed.len());
    }

     #[test]
    fn parse_file_3() {
        use std::io::Cursor;

        let file_data = include_str!("../test_data/Driver_Motor.lib");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_components(&mut file_cursor).unwrap();

        assert_eq!(23, parsed.len());
    }

    #[test]
    fn parse_schema_1() {
        use std::io::Cursor;

        let file_data = include_str!("../test_data/kicad.sch");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_schema(&mut file_cursor).unwrap();

        assert_eq!(159, parsed.len());
    }
}