pub enum Term {
    OK,
    ERROR,
    NOTFOUND,
}
pub struct HttpResponse {
    status_code: i32,
    term: Term,
}
impl HttpResponse {
    pub fn new(status_code: i32, term: Term) -> HttpResponse {
        HttpResponse { status_code, term }
    }
}
