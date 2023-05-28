pub mod parser {
    use std::{error::Error, fmt, path::Display};

    use crate::{
        errors::{HttpRequestBuildError, InvalidBasicInfoError},
        http_request::{self, BasicInfo, HttpRequest, Verb},
    };

    pub fn parse(input: &str) -> Result<HttpRequest, HttpRequestBuildError> {
        let split: Vec<&str> = input.split('\n').collect();
        let basic_info_line = split[0];
        let basic_info = match parse_basic_info(basic_info_line) {
            Ok(b) => b,
            Err(_) => return Err(HttpRequestBuildError::new()),
        };

        let http_request = HttpRequest::new(basic_info);
        Ok(http_request)
    }

    pub fn parse_basic_info(input: &str) -> Result<BasicInfo, InvalidBasicInfoError> {
        let split: Vec<&str> = input.split(' ').collect();

        //guard clause -> check all *required* info is supplied
        if split.len() != 3 {
            return Err(InvalidBasicInfoError::new());
        }

        let verb = match parse_verb(split[0]) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let uri = split[1];
        //TODO: Parse out the version here -> make sure is correct
        let version = split[2];

        Ok(BasicInfo::new(verb, uri.to_string(), version.to_string()))
    }

    pub fn parse_verb(input: &str) -> Result<Verb, InvalidBasicInfoError> {
        match input {
            "GET" => Ok(Verb::GET),
            "POST" => Ok(Verb::POST),
            "PUT" => Ok(Verb::PUT),
            "DELETE" => Ok(Verb::DELETE),
            _ => Err(InvalidBasicInfoError::new()),
        }
    }
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
    use crate::{
        errors::InvalidBasicInfoError,
        http_request::{BasicInfo, Verb},
    };

    use super::parser::{parse_basic_info, parse_verb};

    #[test]
    pub fn test_verb_parse() {
        let data = "GET";

        let expected = Verb::GET;

        let actual = parse_verb(data).unwrap();
        assert_eq!(expected, actual);

        let data = "POST";
        let expected = Verb::POST;
        let actual = parse_verb(data).unwrap();
        assert_eq!(expected, actual);
        let data = "PUT";
        let expected = Verb::PUT;
        let actual = parse_verb(data).unwrap();
        assert_eq!(expected, actual);
        let data = "DELETE";
        let expected = Verb::DELETE;
        let actual = parse_verb(data).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_invalid_verb_parse() {
        let data = "OTHER";
        let expected = InvalidBasicInfoError::new();
        let actual = parse_verb(data).expect_err("Invalid basic info supplied");
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
