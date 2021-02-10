use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("ttl: {}", stream.ttl().unwrap());
        println!("연결됨");
    }
}
