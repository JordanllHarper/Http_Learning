use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1:8000";
    let mut stream = TcpStream::connect(ip)?;

    let message = "Hello, Server!\n";
    stream.write(message.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write);

    println!("Sent message {message}");

    println!("Flushed. Waiting for response...");

    let mut buf = vec![];
    stream.read_to_end(&mut buf)?;
    let message = std::str::from_utf8(&mut buf).unwrap();

    println!("Received message {message}");
    Ok(())
}
