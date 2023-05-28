#[derive(Debug, PartialEq, Eq)]
pub struct InvalidBasicInfoError {
    message: String,
}

#[derive(Debug)]
pub struct HttpRequestBuildError {
    message: String,
}

#[derive(Debug)]
pub struct HttpResponseBuildError {
    message: String,
}
