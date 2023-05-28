pub mod parser {
    use crate::{
        errors::error_decl::{HttpResponseBuildError, InvalidBasicInfoError},
        http_response::http_response::{ContentType, HttpResponse, HttpResponseBasicInfo, Status},
    };

    pub fn parse(input: &str) -> Result<HttpResponse, HttpResponseBuildError> {
        let split: Vec<&str> = input.split('\n').collect();
        if split.len() < 3 {
            return Err(HttpResponseBuildError::new(
                "Invalid number of arguments provided.".to_string(),
            ));
        }

        let basic_info = match parse_response_basic_info(split[0]){
            Ok(basic_info) => basic_info,
            Err(e) => return Err(HttpResponseBuildError::new("Something went wrong with the basic parsing. Make sure it's in the correct format.".to_string())),
        };
        let content_length = match str::parse::<i32>(&split[1].trim()) {
            Ok(content_length) => content_length,
            Err(e) => {
                return Err(HttpResponseBuildError::new(
                    "Invalid content length".to_string(),
                ))
            }
        };
        let content_type = match parse_content_type(split[2]) {
            Ok(ct) => ct,
            Err(_) => {
                return Err(HttpResponseBuildError::new(
                    "Invalid content type provided.".to_string(),
                ))
            }
        };

        let response = HttpResponse::new(basic_info, content_length, content_type);

        return Ok(response);
    }
    pub fn parse_content_type(input: &str) -> Result<ContentType, HttpResponseBuildError> {
        match input {
            "text/html" => Ok(ContentType::TextHtml),
            "text/plain" => Ok(ContentType::TextPlain),
            _ => Err(HttpResponseBuildError::new(
                "Invalid http response build error".to_string(),
            )),
        }
    }
    pub fn parse_response_basic_info(
        line: &str,
    ) -> Result<HttpResponseBasicInfo, InvalidBasicInfoError> {
        //[Version] [Status code] [Status]

        let split: Vec<&str> = line.split(' ').collect();
        let version = split[0];
        let status_code = split[1];
        let status_code = match parse_status_code(status_code) {
            Ok(status_code) => status_code,
            Err(e) => {
                return Err(InvalidBasicInfoError::new(
                    "Invalid status code given".to_string(),
                ))
            }
        };
        let status = split[2];
        let status = match parse_status(status) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(HttpResponseBasicInfo::new(
            version.to_string(),
            status_code,
            status,
        ))
    }

    pub fn parse_status_code(code_str: &str) -> Result<i32, std::num::ParseIntError> {
        let code_str = code_str.trim();

        let code = str::parse::<i32>(code_str);

        code
    }
    pub fn parse_status(status_str: &str) -> Result<Status, InvalidBasicInfoError> {
        match status_str {
            "OK" => Ok(Status::OK),
            "NOTFOUND" => Ok(Status::NOTFOUND),
            "ERROR" => Ok(Status::ERROR),
            _ => Err(InvalidBasicInfoError::new("Invalid status".to_string())),
        }
    }
}
