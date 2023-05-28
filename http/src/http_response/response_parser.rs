pub mod parser {
    use crate::http_response::HttpResponse;

    pub fn parse(input: &str) -> Result<HttpResponse> {
        let split: Vec<&str> = input.split('\n').collect();
        if split.len() < 3 {}
    }
}
