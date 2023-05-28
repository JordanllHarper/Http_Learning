pub enum Status {
    OK,
    ERROR,
    NOTFOUND,
}

pub enum ContentType {
    TextPlain,
    TextHtml,
    MultipartFormData,
}
pub struct HttpResponse {
    http_response_basic_info: HttpResponseBasicInfo,
    content_length: i32,
    content_type: ContentType,
}
pub struct HttpResponseBasicInfo {
    version: String,
    status_code: i32,
    status: Status,
}
impl HttpResponse {
    pub fn new(
        http_response_basic_info: HttpResponseBasicInfo,
        content_length: i32,
        content_type: ContentType,
    ) -> HttpResponse {
        HttpResponse {
            http_response_basic_info,
            content_length,
            content_type,
        }
    }
}
