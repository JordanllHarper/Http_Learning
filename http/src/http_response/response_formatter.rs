pub mod ResponseFormatter {
    use crate::{
        http_response::http_response::HttpResponse, status_mapper::status_mapper::StatusMapper,
    };

    pub fn format(response: HttpResponse) -> String {
        let basic_info = response.http_response_basic_info;
        let content_type = response.content_type;
        let content_length = response.content_length;
        let content = response.content;

        //basic info
        let string_response = String::from(basic_info.status_code.to_string());
        string_response.push_str(" ");
        let status = StatusMapper::new().map_to_string(&basic_info.status);
        string_response.push_str(&status);

        string_response.push_str(" ");
        string_response.push_str(&basic_info.version);
        string_response.push_str("\n");
        string_response.push_str("Content-Type: ");
        string_response.push_str(content_type);
        string_response.push('\n');
        string_response.push_str("Content-Length: ")
        string_response.push_str(content_length)
    }
}
