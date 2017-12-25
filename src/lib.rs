#![recursion_limit="128"]

#[macro_use]
extern crate nom;

pub mod component;

use component::Component;
use std::io::Read;

use nom::line_ending;

pub fn parse_components<R: Read>(data: &mut R) -> Option<Vec<Component>> {
    let mut buff: Vec<u8> = Vec::new();

    data.read_to_end(&mut buff);

    let parse_raw = component_file(&buff);

    if let nom::IResult::Done(_, components) = parse_raw {
        Some(components)
    } else {
        println!("Error reading from file: {:#?}", parse_raw);
        None
    }
}

named!(component_file< Vec<Component> >,
    do_parse!(
        tag_s!("EESchema-LIBRARY Version 2.3") >>
        line_ending >>
        components: many1!(component::component) >>
        (components)
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

        // for comp in &parsed {
        //     println!("{:#?}", comp);
        // }

        assert_eq!(6, parsed.len());
    }
}