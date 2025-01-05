use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


const contents: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
"#;


fn handle_client(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_>= buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";



    let status_line = "HTTP/1.1 200 OK";
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080");

    for stream in listener.unwrap().incoming(){

        stream.and_then(|stream| Ok(thread::spawn(|| {
            handle_client(stream)
        })))
            .expect("TODO: panic message");
    }

}

