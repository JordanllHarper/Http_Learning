pub enum Status {
    OK,
    ERROR,
    NOTFOUND,
}

pub enum ContentType {
    TextPlain,
}
pub struct HttpResponse {
    version: String,
    status_code: i32,
    status: Status,
    content_length: i32,
    content_type: ContentType,
}
impl HttpResponse {
    pub fn new(
        version: String,
        status_code: i32,
        status: Status,
        content_length: i32,
        content_type: ContentType,
    ) -> HttpResponse {
        HttpResponse {
            version,
            status_code,
            status,
            content_length,
            content_type,
        }
    }
}
