use std::str;

use nom::{
    space,
    anychar,
    alpha,
    line_ending,
    not_line_ending,
};
use nom::types::CompleteByteSlice;

use geometry::Point2D;
use parsing::kicad::component_library::*;

pub fn bytes_to_utf8(c: CompleteByteSlice) -> Result<&str, str::Utf8Error> {
    str::from_utf8(c.0)
}

/// Parses a general utf8 string
named!(pub utf8_str(CompleteByteSlice) -> &str,
    map_res!(
        take_until_either!(" \r\n"),
        bytes_to_utf8
    )
);

/// Parses a utf8 numberstring value to float
named!(pub coordinate(CompleteByteSlice) -> f32,
    map_res!(number_str, { |r: &str| r.parse() })
);

named!(pub number_str(CompleteByteSlice) -> &str,
    map_res!(take_while1!(is_number_char), bytes_to_utf8)
);

pub fn is_number_char(c: u8) -> bool {
    ((c >= '0' as u8) && (c <= '9' as u8)) || c == '-' as u8 || c == '.' as u8
}

named!(pub point(CompleteByteSlice) -> Point2D,
    do_parse!(
        x: coordinate >>
        space >>
        y: coordinate >>
        (Point2D::new(x,y))
    )
);

named!(pub delimited_text(CompleteByteSlice) -> &str,
    map_res!(delimited!(tag!("\""), take_until!("\""), tag!("\"")), bytes_to_utf8)
);

named!(pub orientation(CompleteByteSlice) -> TextOrientation, 
    map_opt!(anychar, TextOrientation::from_char)
);

named!(pub visibility(CompleteByteSlice) -> bool,
    map!(anychar, |c| c == 'V')
);

named!(pub justification(CompleteByteSlice) -> Justify,
    map_opt!(anychar, Justify::from_char)
);

// Eats a comment
named!(pub comment(CompleteByteSlice) -> (),
    do_parse!(
        tag!("#") >>
        not_line_ending >>
        line_ending >>
        ()
    )
);

/// Parses a Y/N value to true/false
named!(pub yesno(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['Y' as u8]})
);

/// Parses a L/F value to true/false
named!(pub locked(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['L' as u8]})
);

/// Parses a filled value to true/false
named!(pub filled(CompleteByteSlice) -> bool,
    map!(alpha, {|c: CompleteByteSlice| c.0 == &['F' as u8]})
);


/// Parses a utf8 numberstring value to signed int
named!(pub int(CompleteByteSlice) -> isize,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a utf8 numberstring value to float
named!(pub float(CompleteByteSlice) -> f32,
    map_res!(number_str, { |i: &str| i.parse() })
);

/// Parses a utf8 numberstring value to unsigned int
named!(pub uint(CompleteByteSlice) -> usize,
    map_res!(number_str, { |i: &str| i.parse() })
);

named!(pub italic(CompleteByteSlice) -> bool, map!(anychar, |c| c == 'I'));

named!(pub bold(CompleteByteSlice) -> bool, map!(anychar, |c| c == 'B'));