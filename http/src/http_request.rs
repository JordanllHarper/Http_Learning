use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Verb {
    GET,
    PUT,
    POST,
    DELETE,
}

pub struct BasicInfo {
    verb: Verb,
    uri: String,
    version: String,
}
impl BasicInfo {
    pub fn new(verb: Verb, uri: String, version: String) -> BasicInfo {
        BasicInfo { verb, uri, version }
    }
}

//TODO: Add more fields
pub struct HttpRequest {
    basic_info: BasicInfo,
}
impl HttpRequest {
    pub fn new(basic_info: BasicInfo) -> HttpRequest {
        HttpRequest { basic_info }
    }
}
