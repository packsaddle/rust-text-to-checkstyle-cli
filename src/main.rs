extern crate atty;

use atty::Stream;
use std::io::{self, Read};

fn main() {
    if atty::is(Stream::Stdin) {
        println!("I'm a terminal");
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect(
            "can't read.",
        );
        println!("<xml>{}</xml>", buffer);
    }
}

//<?xml version='1.0'?>
//<checkstyle>
//    <file name='path/to/file'>
//        <error column='0' line='0' message='valid text' severity='info' source='TextToCheckstyle'/>
//    </file>
//</checkstyle>
