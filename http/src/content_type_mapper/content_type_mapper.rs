use std::collections::HashMap;

use crate::http_response::http_response::ContentType;

pub struct ContentTypeMapper {
    pub to_string: HashMap<ContentType, String>,
    pub to_content_type: HashMap<String, ContentType>,
}
impl ContentTypeMapper {
    pub fn map_to_string(&self, content_type: &ContentType) -> Option<String> {
        self.to_string.get(content_type).cloned()
    }
    pub fn map_to_content_type(&self, content_type_string: &String) -> Option<ContentType> {
        self.to_content_type.get(content_type_string).copied()
    }
    pub fn new() -> ContentTypeMapper {
        let to_string: HashMap<ContentType, String> = HashMap::from([
            (ContentType::TextHtml, String::from("text/html")),
            (ContentType::TextPlain, String::from("text/plain")),
        ]);
        let to_content_type: HashMap<String, ContentType> = HashMap::from([
            (String::from("text/html"), ContentType::TextHtml),
            (String::from("text/plain"), ContentType::TextPlain),
        ]);
        ContentTypeMapper {
            to_string,
            to_content_type,
        }
    }
}
