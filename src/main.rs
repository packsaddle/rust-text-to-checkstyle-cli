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
        let mut buffer = String::new();
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect(
            "can't read.",
        );
        let piece = checkstyle::ErrorPiece {
            column: matches.value_of("column").unwrap().parse().unwrap(),
            line: matches.value_of("line").unwrap().parse().unwrap(),
            message: buffer,
            severity: matches.value_of("severity").unwrap().to_string(),
            source: matches.value_of("source").unwrap().to_string(),
        };
        let file = checkstyle::ErrorFile {
            name: matches.value_of("name").unwrap().to_string(),
            error_pieces: vec![piece],
        };
        let container = checkstyle::Container { error_files: vec![file] };
        println!("{}", container.to_xml().unwrap());
    }
}
