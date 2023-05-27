#[derive(Debug)]
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

impl Error for HttpRequestBuildError {}
