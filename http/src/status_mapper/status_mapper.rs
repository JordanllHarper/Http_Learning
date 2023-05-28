use std::collections::HashMap;

use crate::http_response::http_response::Status;

pub struct StatusMapper {
    pub to_string: HashMap<Status, String>,

    pub to_status: HashMap<String, Status>,
}
impl StatusMapper {
    pub fn map_to_status(&self, status_str: &String) -> Option<Status> {
        self.to_status.get(status_str).copied()
    }
    pub fn map_to_string(&self, status: &Status) -> Option<String> {
        self.to_string.get(status).copied()
    }
    pub fn new() -> StatusMapper {
        let to_string: HashMap<Status, String> = HashMap::from([
            (Status::OK, String::from("200")),
            (Status::NOTFOUND, String::from("404")),
            (Status::INTERNALSERVERERROR, String::from("500")),
        ]);
        let to_status: HashMap<String, Status> = HashMap::from([
            (String::from("200"), Status::OK),
            (String::from("404"), Status::NOTFOUND),
            (String::from("500"), Status::INTERNALSERVERERROR),
        ]);

        StatusMapper {
            to_string,
            to_status,
        }
    }
}
