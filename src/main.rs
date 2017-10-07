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
