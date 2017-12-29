use std::str;

use nom::space;

use component::geometry::Point;

/// Parses a general utf8 string
named!(pub utf8_str(&[u8]) -> &str,
    map_res!(
        take_until_either!(" \r\n"),
        str::from_utf8
    )
);

/// Parses a utf8 numberstring value to float
named!(pub coordinate(&[u8]) -> f32,
    map_res!(number_str, { |i: &str| i.parse() })
);

named!(number_str<&str>,
    map_res!(take_while!(is_number_char), str::from_utf8)
);

fn is_number_char(c: u8) -> bool {
    ((c >= '0' as u8) && (c <= '9' as u8)) || c == '-' as u8 || c == '.' as u8
}

named!(pub point<Point>,
    do_parse!(
        x: coordinate >>
        space >>
        y: coordinate >>
        (Point{ x: x, y: y})
    )
);