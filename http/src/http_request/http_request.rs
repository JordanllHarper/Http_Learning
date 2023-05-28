use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verb {
    GET,
    PUT,
    POST,
    DELETE,
}

impl Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Verb::GET => "GET",
            Verb::PUT => "PUT",
            Verb::POST => "POST",
            Verb::DELETE => "DELETE",
        };
        f.write_str(string);

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BasicInfo {
    pub verb: Verb,
    pub uri: String,
    pub version: String,
}
impl BasicInfo {
    pub fn new(verb: Verb, uri: String, version: String) -> BasicInfo {
        BasicInfo { verb, uri, version }
    }
}

//TODO: Add more fields

#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
    pub basic_info: BasicInfo,
}
impl HttpRequest {
    pub fn new(basic_info: BasicInfo) -> HttpRequest {
        HttpRequest { basic_info }
    }
}
