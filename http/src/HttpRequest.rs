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
    pub fn new(verb: Verb, uri: String, version: String) {
        BasicInfo { verb, uri, version }
    }
}

pub struct HttpRequest {}
