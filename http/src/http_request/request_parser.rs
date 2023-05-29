pub mod parser {
    use crate::{
        errors::error_decl::{HttpRequestBuildError, InvalidBasicInfoError},
        http_request::http_request::{BasicInfo, HttpRequest},
        verb_mapper::verb_mapper::VerbMapper,
    };

    pub fn parse(input: &str) -> Result<HttpRequest, HttpRequestBuildError> {
        let split: Vec<&str> = input.split('\n').collect();
        let basic_info_line = split[0];
        let basic_info = match parse_basic_info(basic_info_line) {
            Ok(b) => b,
            Err(_) => {
                return Err(HttpRequestBuildError::new(
                    "Couldn't build http request".to_string(),
                ))
            }
        };

        let http_request = HttpRequest::new(basic_info);
        Ok(http_request)
    }

    pub fn parse_basic_info(input: &str) -> Result<BasicInfo, InvalidBasicInfoError> {
        let split: Vec<&str> = input.split(' ').collect();

        //guard clause -> check all *required* info is supplied + no more
        if split.len() != 3 {
            return Err(InvalidBasicInfoError::new(
                "Invalid number of arguments for request".to_string(),
            ));
        }

        let verb_mapper = VerbMapper::new();
        let verb = verb_mapper.map_to_verb(&split[0].to_string());
        let uri = split[1];
        //TODO: Parse out the version here -> make sure is correct
        let version = split[2];

        Ok(BasicInfo::new(
            verb.to_owned(),
            uri.to_string(),
            version.to_string(),
        ))
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
        errors::error_decl::InvalidBasicInfoError,
        http_request::http_request::{BasicInfo, Verb},
    };

    use super::parser::parse_basic_info;

    #[test]
    pub fn test_basic_info_parse() {
        let data = "GET /hello.txt HTTP/1.1";

        let expected = BasicInfo::new(Verb::GET, "/hello.txt".to_string(), "HTTP/1.1".to_string());
        let actual = parse_basic_info(data);
        assert_eq!(actual.unwrap(), expected);
    }
    #[test]
    pub fn test_invalid_request() {
        let data = "GET  /hello.txt HTTP/1.1";

        let expected =
            InvalidBasicInfoError::new("Invalid number of arguments for request".to_string());

        let actual = parse_basic_info(data).expect_err("Invalid url - additional space");
        assert_eq!(actual, expected);
    }
}
