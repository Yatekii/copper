/// Test the parser by parsing *all* Kicad Symbols
/// Symbols are located in test/kicad-symbols-master

const SYMBOL_PATH: &'static str = "test_data/kicad-symbols-master/";

extern crate schema_parser;

use std::fs;
use std::path::PathBuf;


fn try_parse(p: &PathBuf) {
    let mut file = std::fs::File::open(p).unwrap();

    let parse_res = schema_parser::parse_components(&mut file).expect(&format!("Failed to parse file {:?}", p));

    // There should be at least one component in each file
    assert!(parse_res.len() > 0);
}

#[test]
fn parse_all_symbols() {
    let lib_files = fs::read_dir(SYMBOL_PATH).unwrap();

    for file in lib_files.map(|e| e.unwrap()).filter( |e| e.file_name().to_str().unwrap().ends_with(".lib") ).map( |e| e.path() ) {
        try_parse(&file);
    }
}
