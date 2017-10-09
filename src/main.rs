extern crate atty;

use atty::Stream;
use std::io::{self, Read};
mod checkstyle;

fn main() {
    if atty::is(Stream::Stdin) {
        println!("I'm a terminal");
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect(
            "can't read.",
        );
        let piece = checkstyle::ErrorPiece {
            column: 0,
            line: 0,
            message: buffer,
            severity: String::from("info"),
            source: String::from("text_to_checkstyle"),
        };
        let file = checkstyle::ErrorFile {
            name: String::from("path/to/file"),
            error_pieces: vec![piece],
        };
        let container = checkstyle::Container { error_files: vec![file] };
        println!("{}", container.to_xml().unwrap());
    }
}

//<?xml version='1.0'?>
//<checkstyle>
//    <file name='path/to/file'>
//        <error column='0' line='0' message='valid text'
// severity='info' source='TextToCheckstyle'/>
//    </file>
//</checkstyle>
