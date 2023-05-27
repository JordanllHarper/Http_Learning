pub mod parser {
    use std::{error::Error, fmt, path::Display};

    use crate::http_request::{self, BasicInfo, HttpRequest, Verb};

    pub fn parse(input: &str) -> Result<HttpRequest, HttpRequestBuildError> {
        let split: Vec<&str> = input.split('\n').collect();
        let basic_info_line = split[0];
        let basic_info = match parse_basic_info(basic_info_line) {
            Ok(b) => b,
            Err(_) => return Err(HttpRequestBuildError {}),
        };

        let http_request = HttpRequest::new(basic_info);
        Ok(http_request)
    }

    pub fn parse_basic_info(input: &str) -> Result<BasicInfo, InvalidBasicInfoError> {
        let split: Vec<&str> = input.split(' ').collect();
        let verb = match parse_verb(split[0]) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let uri = split[1];
        let version = split[2];

        Ok(BasicInfo::new(verb, uri.to_string(), version.to_string()))
    }

    pub fn parse_verb(input: &str) -> Result<Verb, InvalidBasicInfoError> {
        match input {
            "GET" => Ok(Verb::GET),
            "POST" => Ok(Verb::POST),
            "PUT" => Ok(Verb::PUT),
            "DELETE" => Ok(Verb::DELETE),
            _ => Err(InvalidBasicInfoError {}),
        }
    }

    #[derive(Debug)]
    pub struct InvalidBasicInfoError {}

    impl fmt::Display for InvalidBasicInfoError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return match write!(f, "Error with the parsing of basic information") {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
        }
    }

    impl Error for InvalidBasicInfoError {}
    #[derive(Debug)]
    pub struct HttpRequestBuildError {}

    impl fmt::Display for HttpRequestBuildError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return match write!(f, "Error with the parsing of http request building") {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
        }
    }

    impl Error for HttpRequestBuildError {}
}
//
//
//
//
//
//
//
//PARSER TESTS//
#[cfg(test)]
mod parser_tests {
    use crate::http_request::{BasicInfo, Verb};

    use super::parser::{parse_basic_info, parse_verb};

    #[test]
    pub fn test_verb_parse() {
        let data = "GET";

        let expected = Verb::GET;

        let actual = parse_verb(data).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_basic_info_parse() {
        let data = "GET /hello.txt HTTP/1.1";

        let expected = BasicInfo::new(Verb::GET, "/hello.txt".to_string(), "HTTP/1.1".to_string());
        let actual = parse_basic_info(data);
        assert_eq!(actual.unwrap(), expected);
    }
}
