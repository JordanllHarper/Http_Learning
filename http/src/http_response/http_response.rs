#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    INTERNALSERVERERROR,
    NOTFOUND,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    TextPlain,
    TextHtml,
}
pub struct HttpResponse {
    pub http_response_basic_info: HttpResponseBasicInfo,

    pub content_length: i32,
    pub content_type: ContentType,
    pub content: String,
}
pub struct HttpResponseBasicInfo {
    pub version: String,
    pub status_code: i32,
    pub status: Status,
}
impl HttpResponse {
    pub fn new(
        http_response_basic_info: HttpResponseBasicInfo,
        content_length: i32,
        content_type: ContentType,
        content: String,
    ) -> HttpResponse {
        HttpResponse {
            http_response_basic_info,
            content_length,
            content_type,
            content,
        }
    }
}
impl HttpResponseBasicInfo {
    pub fn new(version: String, status_code: i32, status: Status) -> HttpResponseBasicInfo {
        HttpResponseBasicInfo {
            version,
            status_code,
            status,
        }
    }
}
