use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let url = "127.0.0.1:3001";
    let listener = TcpListener::bind(url).unwrap();
    println!("The http server is starting at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let text = String::from("hello world! 您好，世界！");
    let content = "<html><head></head><h1>".to_string() + &text +  &"</h1>1234567</html>".to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html;charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        content.as_bytes().len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
