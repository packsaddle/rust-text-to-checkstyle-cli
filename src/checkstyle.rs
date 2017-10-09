extern crate quick_xml;

use std::error::Error;

use self::quick_xml::writer::Writer;
use self::quick_xml::events::{BytesDecl, Event, BytesStart, BytesEnd};

pub struct ErrorPiece {
    pub column: u32,
    pub line: u32,
    pub message: String,
    pub severity: String,
    pub source: String,
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
    pub name: String,
    pub error_pieces: Vec<ErrorPiece>,
}

impl ErrorFile {
    pub fn to_xml_events(&self) -> Result<Vec<Event>, Box<Error>> {
        let mut events = Vec::new();
        let file = b"file";
        let mut start_element = BytesStart::borrowed(file, file.len());
        start_element.push_attribute(("name".as_bytes(), self.name.as_bytes()));
        events.push(Event::Start(start_element));
        for error_piece in &self.error_pieces {
            match error_piece.to_xml_events() {
                Ok(piece_events) => {
                    for event in piece_events {
                        events.push(event);
                    }
                }
                Err(err) => return Err(err),
            }
        }
        events.push(Event::End(BytesEnd::borrowed(file)));
        Ok(events)
    }
}

pub struct Container {
    pub error_files: Vec<ErrorFile>,
}

impl Container {
    pub fn to_xml_events(&self) -> Result<Vec<Event>, Box<Error>> {
        let mut events = Vec::new();
        let name = b"checkstyle";
        events.push(Event::Decl(BytesDecl::new(b"1.0", None, None)));
        events.push(Event::Start(BytesStart::borrowed(name, name.len())));
        for error_file in &self.error_files {
            match error_file.to_xml_events() {
                Ok(file_events) => {
                    for event in file_events {
                        events.push(event);
                    }
                }
                Err(err) => return Err(err),
            }
        }
        events.push(Event::End(BytesEnd::borrowed(name)));
        Ok(events)
    }

    pub fn to_xml(&self) -> Result<String, Box<Error>> {
        let mut writer = Writer::new(Vec::new());
        match self.to_xml_events() {
            Ok(events) => {
                for event in events {
                    writer.write_event(&event)?;
                }
            }
            Err(err) => return Err(err),
        }
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
    fn error_file_to_events() {
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

        let line1 = r#"<file name="path/to/file">"#;
        let line2 = r#"<error column="1" line="2" message="some message" "#;
        let line3 = r#"severity="info" source="some checkstyle"/>"#;
        let line4 = r#"</file>"#;
        let expected = format!("{}{}{}{}", line1, line2, line3, line4);

        let mut writer = Writer::new(Vec::new());
        let events = file.to_xml_events().unwrap();
        for event in events {
            writer.write_event(event).expect("can't write");
        }
        let result = writer.into_inner();
        assert_eq!(String::from_utf8(result).unwrap(), expected);
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

        let l1 = r#"<?xml version="1.0"?>"#;
        let l2 = r#"<checkstyle>"#;
        let l3 = r#"<file name="path/to/file">"#;
        let l4 = r#"<error column="1" line="2" message="some message" "#;
        let l5 = r#"severity="info" source="some checkstyle"/>"#;
        let l6 = r#"</file>"#;
        let l7 = r#"</checkstyle>"#;
        let expected = format!("{}{}{}{}{}{}{}", l1, l2, l3, l4, l5, l6, l7);

        assert_eq!(checkstyle.to_xml().unwrap(), expected);
    }
}

//<?xml version='1.0'?>
//<checkstyle>
//    <file name='path/to/file'>
//        <error column='0' line='0' message='valid text'
// severity='info' source='TextToCheckstyle'/>
//    </file>
//</checkstyle>
