pub mod request_formatter {

    use crate::{http_request::http_request::HttpRequest, verb_mapper};

    pub fn format(request: HttpRequest) -> Option<String> {
        let mapper = verb_mapper::verb_mapper::VerbMapper::new();
        let verb = match mapper.map_to_string(&request.basic_info.verb) {
            Some(v) => v,
            None => return None,
        };
        let uri = request.basic_info.uri;
        let version = request.basic_info.version;
        let mut returned_string = String::from(verb);
        returned_string.push_str(" ");
        returned_string.push_str(&uri);
        returned_string.push_str(" ");
        returned_string.push_str(&version);
        Some(returned_string)
    }
}
#[cfg(test)]
pub mod request_formatter_tests {
    use crate::http_request::http_request::{BasicInfo, HttpRequest, Verb};

    use super::request_formatter::format;

    #[test]
    pub fn test_format() {
        let basic = BasicInfo::new(Verb::GET, "/".to_string(), "HTTP/1.1".to_string());
        let data = HttpRequest::new(basic);

        let expected = "GET / HTTP/1.1".to_string();

        let actual = format(data);
        assert_eq!(expected, actual.unwrap());
    }
}
