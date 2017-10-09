extern crate quick_xml;

use std::error::Error;

use self::quick_xml::writer::Writer;
use self::quick_xml::events::{BytesDecl, Event, BytesStart, BytesEnd, BytesText};

pub struct ErrorPiece {
    column: u32,
    line: u32,
    message: String,
    severity: String,
    source: String,
}

impl ErrorPiece {
    pub fn to_xml_events(&self) -> Result<Vec<Event>, Box<Error>> {
        let mut events = Vec::new();
        let error = b"error";
        let mut element = BytesStart::borrowed(error, error.len());
        element.push_attribute(("column".as_bytes(), self.column.to_string().as_bytes()));
        element.push_attribute(("line".as_bytes(), self.line.to_string().as_bytes()));
        element.push_attribute(("message".as_bytes(), self.message.as_bytes()));
        element.push_attribute(("severity".as_bytes(), self.severity.as_bytes()));
        element.push_attribute(("source".as_bytes(), self.source.as_bytes()));
        events.push(Event::Empty(element));
        Ok(events)
    }
}

pub struct ErrorFile {
    name: String,
    error_pieces: Vec<ErrorPiece>,
}

pub struct Container {
    error_files: Vec<ErrorFile>,
}

impl Container {
    fn to_xml(&self) -> Result<String, Box<Error>> {
        let mut writer = Writer::new(Vec::new());
        let name = b"checkstyle";
        writer.write_event(
            Event::Decl(BytesDecl::new(b"1.0", None, None)),
        )?;
        writer.write_event(
            Event::Start(BytesStart::borrowed(name, name.len())),
        )?;
        for error_file in &self.error_files {
            writer.write_event(Event::Text(
                BytesText::borrowed(error_file.name.as_bytes()),
            ))?;
        }
        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        let result = writer.into_inner();
        Ok(String::from_utf8(result).expect("utf-8 output"))
    }
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
    fn piece_to_xml_events() {
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
        let line1 = r#"<error column="1" line="2" message="some message" "#;
        let line2 = r#"severity="info" source="some checkstyle"/>"#;
        let expected = format!("{}{}", line1, line2);
        let mut writer = Writer::new(Vec::new());
        let events = piece.to_xml_events().unwrap();
        for event in events {
            writer.write_event(event).expect("can't write");
        }
        let result = writer.into_inner();
        assert_eq!(String::from_utf8(result).unwrap(), expected);
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

    #[test]
    fn build_checkstyle() {
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
        let checkstyle = Container { error_files: vec![file] };
        assert_eq!(checkstyle.error_files[0].error_pieces[0].line, 2);
    }
    #[test]
    fn build_checkstyle_to_xml() {
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
        let checkstyle = Container { error_files: vec![file] };
        assert_eq!(
            checkstyle.to_xml().unwrap(),
            r#"<?xml version="1.0"?><checkstyle>path/to/file</checkstyle>"#
        );
    }
}

//<?xml version='1.0'?>
//<checkstyle>
//    <file name='path/to/file'>
//        <error column='0' line='0' message='valid text'
// severity='info' source='TextToCheckstyle'/>
//    </file>
//</checkstyle>
