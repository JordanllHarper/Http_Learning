use std::{error::Error, fmt};

use super::error_decl::{HttpRequestBuildError, HttpResponseBuildError, InvalidBasicInfoError};

//Basic error declarations
impl Error for InvalidBasicInfoError {}

impl Error for HttpRequestBuildError {}

impl Error for HttpResponseBuildError {}

//constructors for errors
impl HttpResponseBuildError {
    pub fn new(message: String) -> HttpResponseBuildError {
        HttpResponseBuildError { message }
    }
}
impl InvalidBasicInfoError {
    pub fn new(message: String) -> InvalidBasicInfoError {
        InvalidBasicInfoError { message }
    }
}

impl HttpRequestBuildError {
    pub fn new(message: String) -> HttpRequestBuildError {
        HttpRequestBuildError { message }
    }
}

//Display implementations
impl fmt::Display for InvalidBasicInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match write!(f, "Error with the parsing of basic information") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}
impl fmt::Display for HttpResponseBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match write!(f, "Error with the parsing of http response building") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}
impl fmt::Display for HttpRequestBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match write!(f, "Error with the parsing of http request building") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}
