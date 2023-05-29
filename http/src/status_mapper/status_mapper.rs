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
        self.to_string.get(status).cloned()
    }
    pub fn new() -> StatusMapper {
        let to_string: HashMap<Status, String> = HashMap::from([
            (Status::OK, String::from("OK")),
            (Status::NOTFOUND, String::from("NOT FOUND")),
            (
                Status::INTERNALSERVERERROR,
                String::from("INTERNAL SERVER ERROR"),
            ),
        ]);
        let to_status: HashMap<String, Status> = HashMap::from([
            (String::from("OK"), Status::OK),
            (String::from("NOT FOUND"), Status::NOTFOUND),
            (
                String::from("INTERNAL SERVER ERROR"),
                Status::INTERNALSERVERERROR,
            ),
        ]);

        StatusMapper {
            to_string,
            to_status,
        }
    }
}
