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

struct ErrorPiece {
    column: u32,
    line: u32,
    message: String,
    severity: String,
    source: String,
}
struct ErrorFile {
    name: String,
    error_pieces: Vec<ErrorPiece>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_error_piece() {
        let column = 1;
        let line = 2;
        let message = "some message";
        let severity = "info";
        let source = "some checkstyle";
        let piece = ErrorPiece {
            column,
            line,
            message: message.to_string(),
            severity: severity.to_string(),
            source: source.to_string(),
        };
        assert_eq!(piece.line, 2);
    }
    #[test]
    fn build_error_file() {
        let column = 1;
        let line = 2;
        let message = "some message";
        let severity = "info";
        let source = "some checkstyle";
        let piece = ErrorPiece {
            column,
            line,
            message: message.to_string(),
            severity: severity.to_string(),
            source: source.to_string(),
        };
        let name = "path/to/file";
        let file = ErrorFile {
            name: name.to_string(),
            error_pieces: vec![piece],
        };
        assert_eq!(file.error_pieces[0].line, 2);
    }
}

//<?xml version='1.0'?>
//<checkstyle>
//    <file name='path/to/file'>
//        <error column='0' line='0' message='valid text'
// severity='info' source='TextToCheckstyle'/>
//    </file>
//</checkstyle>
