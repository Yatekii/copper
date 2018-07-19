#![recursion_limit="128"]
#![feature(extern_prelude)]
#![feature(nll)]

#[macro_use]
extern crate nom;
extern crate ncollide2d;
extern crate nalgebra;
extern crate lyon;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate epoxy;
extern crate gfx_core;
extern crate gfx_gl;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate bitflags;
extern crate uuid;

pub mod parsing;
pub mod drawing;
pub mod geometry;
pub mod manipulation;
pub mod utils;
pub mod state;
pub mod viewing;
pub mod loading;

use std::io::Read;

use parsing::component::Component;
use parsing::schema_file::SchemaFile;


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
        components: many1!(parsing::component::component) >>
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

    #[test]
    fn parse_schema_1() {
        use std::io::Cursor;

        let file_data = include_str!("../../test_data/kicad.sch");

        let mut file_cursor = Cursor::new(file_data.as_bytes());

        let parsed = parse_schema(&mut file_cursor).unwrap();

        assert_eq!(160, parsed.components.len());

        assert_eq!(79, parsed.labels.len());
    }
}