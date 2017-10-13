extern crate atty;
#[macro_use]
extern crate clap;

use atty::Stream;
use clap::{Arg, App};
use std::io::{self, Read};
use std::fs::File;
mod text2checkstyle;
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

    let mut buffer = String::new();
    if atty::is(Stream::Stdin) {
        match matches.value_of("file") {
            Some(file_name) => {
                let mut f = File::open(file_name).expect("file not found");
                f.read_to_string(&mut buffer).expect(
                    "something went wrong reading the file",
                );
            }
            _ => return,
        }
    } else {
        io::stdin().read_to_string(&mut buffer).expect(
            "can't read.",
        );
    }
    match text2checkstyle::run(
        buffer.as_ref(),
        matches.value_of("column").unwrap().parse().unwrap(),
        matches.value_of("line").unwrap().parse().unwrap(),
        matches.value_of("severity").unwrap(),
        matches.value_of("source").unwrap(),
        matches.value_of("name").unwrap(),
    ) {
        Ok(checkstyle_text) => println!("{}", checkstyle_text),
        Err(err) => eprintln!("{}", err),
    }
}
