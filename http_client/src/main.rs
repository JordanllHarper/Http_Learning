use http::errors::error_decl::InvalidFormatError;
use http::http_response::response_parser;
use http::{
    self,
    http_request::{
        self,
        http_request::{BasicInfo, Verb},
        request_formatter::request_formatter::format,
    },
};
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1:8000";
    let mut stream = TcpStream::connect(ip)?;

    let request = http_request::http_request::HttpRequest::new(BasicInfo::new(
        Verb::GET,
        "/".to_string(),
        "HTTP/1.1".to_string(),
    ));
    let request_formatted = match format(request) {
        Some(r) => r,
        None => return Err(InvalidFormatError::new("Invalid format".to_string())),
    };

    println!("Sending message: {request_formatted}");
    stream.write(request_formatted.as_bytes());

    stream.shutdown(std::net::Shutdown::Write)?;

    println!("Sent message");

    let mut buf = vec![];
    stream.read_to_end(&mut buf)?;
    let message = std::str::from_utf8(&mut buf).unwrap();
    println!("Received response: {message}. Parsing to http response. ");
    let response = response_parser::parser::parse(message);

    Ok(())
}
