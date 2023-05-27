use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    //write a response

    println!("||| Incoming message |||");

    let mut buffer = vec![];
    stream.read_to_end(&mut buffer)?;
    let message = std::str::from_utf8(&buffer).unwrap();
    println!("Received message {message}");

    stream.write(b"Hello, client!")?;

    println!("Sent message to client");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1:8000";
    let listener = TcpListener::bind(ip)?;

    println!("Listening on {ip}");
    for stream in listener.incoming() {
        handle_client(&mut stream?)?
    }
    Ok(())
}
