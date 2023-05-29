#[derive(Debug, PartialEq, Eq)]
pub struct InvalidBasicInfoError {
    pub message: String,
}

#[derive(Debug)]
pub struct HttpRequestBuildError {
    pub message: String,
}

#[derive(Debug)]
pub struct HttpResponseBuildError {
    pub message: String,
}
#[derive(Debug)]
pub struct InvalidFormatError {
    pub message: String,
}
