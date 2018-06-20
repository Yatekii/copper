#![recursion_limit="128"]

#[macro_use]
extern crate nom;

extern crate euclid;

extern crate ncollide2d;

#[macro_use]
extern crate derivative;

pub mod component;
pub mod schema_file;
mod common_parsing;

pub mod geometry;
pub use std::cell::Cell;
pub mod helpers;

use component::Component;
use schema_file::SchemaFile;
use std::io::Read;

use nom::{line_ending, space, digit};
use nom::types::CompleteByteSlice;

pub fn parse_components<R: Read>(data: &mut R) -> Option<Vec<Component>> {
    let mut buff: Vec<u8> = Vec::new();

    if let Ok(_) = data.read_to_end(&mut buff) {
        let parse_raw = component_file(CompleteByteSlice(&buff));

        if let Ok((_, components)) = parse_raw {
            Some(components)
        } else {
            println!("Error reading from file: {:#?}", parse_raw);
            None
        }
    } else {
        None
    }
}

named!(component_file(CompleteByteSlice) -> Vec<Component>,
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

pub fn parse_schema<R: Read>(data: &mut R) -> Option<SchemaFile> {
    let mut buff: Vec<u8> = Vec::new();

    if let Ok(_) = data.read_to_end(&mut buff) {
        SchemaFile::parse(&buff)
    } else {
        None
    }
}

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

        assert_eq!(159, parsed.components.len());

        assert_eq!(79, parsed.labels.len());
    }
}