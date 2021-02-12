use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream_result in listener.incoming() {
        let str = stream_result.unwrap();
        handle_connection(str);
    }
}
