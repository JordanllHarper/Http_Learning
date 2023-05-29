use std::collections::HashMap;

use crate::http_request::http_request::Verb;

pub struct VerbMapper {
    pub to_verb: HashMap<String, Verb>,
    pub to_string: HashMap<Verb, String>,
}

impl VerbMapper {
    pub fn new() -> VerbMapper {
        let to_verb = VerbMapper::provide_string_to_verb();
        let to_string = VerbMapper::provide_verb_to_string();
        VerbMapper { to_verb, to_string }
    }
    pub fn map_to_string(&self, verb: &Verb) -> Option<String> {
        self.to_string.get(verb).cloned()
    }
    pub fn map_to_verb(&self, verb_string: &String) -> Verb {
        self.to_verb[verb_string]
    }
    pub fn provide_string_to_verb() -> HashMap<String, Verb> {
        let map: HashMap<String, Verb> = HashMap::from([
            (String::from("GET"), Verb::GET),
            (String::from("POST"), Verb::POST),
            (String::from("PUT"), Verb::PUT),
            (String::from("DELETE"), Verb::DELETE),
        ]);
        map
    }

    pub fn provide_verb_to_string() -> HashMap<Verb, String> {
        let map: HashMap<Verb, String> = HashMap::from([
            (Verb::GET, String::from("GET")),
            (Verb::POST, String::from("POST")),
            (Verb::PUT, String::from("PUT")),
            (Verb::DELETE, String::from("DELETE")),
        ]);
        map
    }
}
