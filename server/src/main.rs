use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {

    let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    \r\n\
                    Hello, World!\r\n";

    stream.write(response.as_bytes()).expect("Failed to write response");
    stream.flush().expect("Failed to flush stream");
    println!("{:?}", stream);
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080");

    for stream in listener.unwrap().incoming(){

        stream.and_then(|stream| Ok(handle_client(stream))).expect("TODO: panic message");
    }

}

