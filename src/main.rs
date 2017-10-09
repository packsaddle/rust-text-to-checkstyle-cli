extern crate atty;
#[macro_use]
extern crate clap;

use atty::Stream;
use clap::{Arg, App};
use std::io::{self, Read};
mod checkstyle;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("file").value_name("FILE").index(1))
        .arg(
            Arg::with_name("name")
                .long("name")
                .value_name("NAME")
                .default_value("path/to/file"),
        )
        .arg(
            Arg::with_name("line")
                .long("line")
                .value_name("LINE")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("column")
                .long("column")
                .value_name("COLUMN")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("severity")
                .long("severity")
                .value_name("SEVERITY")
                .default_value("info"),
        )
        .arg(
            Arg::with_name("source")
                .long("source")
                .value_name("SOURCE")
                .default_value("TextToCheckstyle"),
        )
        .get_matches();

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
