pub mod Parser {
    use crate::http_request::{BasicInfo, Verb};

    pub fn parse(input: &str) {
        let split = input.split('\n');
        let basic_info_line = split[0];
        let basic_info = parse_basic_info(basic_info_line);
    }

    pub fn parse_basic_info(input: &str) -> BasicInfo {}
    pub fn parse_verb(input: &str) -> Verb {
        match input {
            "GET" => Verb::GET,
            "POST" => Verb::POST,
            "PUT" => Verb::PUT,
            "DELETE" => Verb::DELETE,
        }
    }
}
#[cfg(test)]
mod parser_tests {
    use crate::http_request::{BasicInfo, Verb};

    use super::Parser::parse_verb;

    #[test]
    pub fn test_verb_parse() {
        let data = "GET";

        let expected = Verb::GET;

        let actual = parse_verb(data);
    }

    #[test]
    pub fn test_basic_info_parse() {
        let data = "GET /hello.txt HTTP/1.1";

        let expected = BasicInfo::new(Verb::GET, "/hello.txt", "HTTP/1.1");

        let actual = parse_basic_info();
    }
}
