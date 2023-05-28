pub mod parser {
    use crate::{
        errors::error_decl::HttpResponseBuildError, http_response::http_response::HttpResponse,
    };

    pub fn parse(input: &str) -> Result<HttpResponse, HttpResponseBuildError> {
        let split: Vec<&str> = input.split('\n').collect();
        if split.len() < 3 {
            return Err(HttpResponseBuildError::new(
                "Invalid number of arguments".to_string(),
            ));
        }
        return Ok(());
    }
}
