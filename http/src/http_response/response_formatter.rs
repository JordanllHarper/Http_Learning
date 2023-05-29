pub mod ResponseFormatter {
    use crate::{
        content_type_mapper::content_type_mapper::ContentTypeMapper,
        http_response::http_response::{HttpResponse, HttpResponseBasicInfo},
        status_mapper::status_mapper::StatusMapper,
    };

    pub fn format(response: HttpResponse) -> Option<String> {
        let basic_info = response.http_response_basic_info;
        let content_type = response.content_type;
        let content_length = response.content_length;
        let content = response.content;

        //
        //
        //basic info
        let mut basic_info_str = match handle_basic_info(basic_info) {
            Some(s) => s,
            None => return None,
        };
        basic_info_str.push_str("\n");

        //
        //
        //content type
        let mut content_type_str = "Content-Type: ".to_string();
        let content_type_mapped = match ContentTypeMapper::new().map_to_string(&content_type) {
            Some(s) => s,
            None => return None,
        };
        content_type_str.push_str(&content_type_mapped);
        content_type_str.push_str("\n");
        //
        //
        //content length
        let mut content_len_str = "Content-Length: ".to_string();
        content_len_str.push_str(&content_length.to_string());
        content_len_str.push_str("\n");

        //headers assembling
        let mut returned = String::from(basic_info_str);
        returned.push_str(&content_type_str);
        returned.push_str(&content_len_str);
        //
        //Content stage
        //
        returned.push_str("\n");
        returned.push_str(&content);
        //
        Some(returned)
    }
    pub fn handle_basic_info(basic_info: HttpResponseBasicInfo) -> Option<String> {
        let mut basic_info_string = String::from(basic_info.status_code.to_string());
        basic_info_string.push_str(" ");
        let status_str = match StatusMapper::new().map_to_string(&basic_info.status) {
            Some(s) => s,
            None => return None,
        };
        basic_info_string.push_str(&status_str);
        basic_info_string.push_str(" ");
        basic_info_string.push_str(&basic_info.version);
        Some(basic_info_string.to_string())
    }
}
#[cfg(test)]
pub mod response_formatter_tests {
    use crate::http_response::http_response::{
        ContentType, HttpResponse, HttpResponseBasicInfo, Status,
    };

    use super::ResponseFormatter;

    #[test]
    pub fn test_format() {
        let data = HttpResponse::new(
            HttpResponseBasicInfo::new("HTTP/1.1".to_string(), 200, Status::OK),
            5,
            ContentType::TextPlain,
            "Hello".to_string(),
        );

        let expected = "200 OK HTTP/1.1\nContent-Type: text/plain\nContent-Length: 5\n\nHello";
        let actual = ResponseFormatter::format(data);
        assert_eq!(expected, actual.unwrap());
    }
}
