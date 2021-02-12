use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_request) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream_result in listener.incoming() {
        let str = stream_result.unwrap();
        handle_connection(str);
    }
}
