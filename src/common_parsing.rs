use std::str;

use nom::space;
use nom::types::CompleteByteSlice;

use geometry::Point2D;

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
    map_res!(take_while!(is_number_char), bytes_to_utf8)
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