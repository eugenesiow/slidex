use std::collections::HashMap;
use std::io::Cursor;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::core::{CoreError, Result};
use crate::core::xml::local_name;

pub fn parse_relationships(bytes: &[u8]) -> Result<HashMap<String, String>> {
    let mut reader = Reader::from_reader(Cursor::new(bytes));
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut rels = HashMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e))
                if local_name(e.name().as_ref()) == b"Relationship" =>
            {
                let mut id = None;
                let mut target = None;
                for attr in e.attributes().flatten() {
                    match local_name(attr.key.as_ref()) {
                        b"Id" => id = Some(attr.unescape_value()?.to_string()),
                        b"Target" => target = Some(attr.unescape_value()?.to_string()),
                        _ => {}
                    }
                }
                if let (Some(id), Some(target)) = (id, target) {
                    rels.insert(id, target);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => return Err(CoreError::Xml(err.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(rels)
}
