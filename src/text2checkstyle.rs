use std::error::Error;
use checkstyle;

pub fn run(
    message: &str,
    column: u32,
    line: u32,
    severity: &str,
    source: &str,
    name: &str,
) -> Result<String, Box<Error>> {
    let piece = checkstyle::ErrorPiece {
        column,
        line,
        message: message.to_string(),
        severity: severity.to_string(),
        source: source.to_string(),
    };
    let file = checkstyle::ErrorFile {
        name: name.to_string(),
        error_pieces: vec![piece],
    };
    let container = checkstyle::Container { error_files: vec![file] };
    return container.to_xml();
}
