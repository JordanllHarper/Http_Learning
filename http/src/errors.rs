use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
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

impl InvalidBasicInfoError {
    pub fn new() -> InvalidBasicInfoError {
        InvalidBasicInfoError {}
    }
}
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
impl HttpRequestBuildError {
    pub fn new() -> HttpRequestBuildError {
        HttpRequestBuildError {}
    }
}

impl Error for HttpRequestBuildError {}
