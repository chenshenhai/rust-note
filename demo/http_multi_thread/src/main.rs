use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;


fn main() {
    let url = "127.0.0.1:3001";
    let listener = TcpListener::bind(url).unwrap();

    println!("The http server is starting at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // TODO
        thread::spawn(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let content = "<h1>hello world</h1>1234567";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        content.chars().count(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
