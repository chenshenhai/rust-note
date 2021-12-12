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
    let url_path:String = get_path(&stream);
    let content = "<html><head></head><h1> URL Path = ".to_string() + &url_path +  &"</h1></html>".to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html;charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        content.as_bytes().len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_path(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
            path[1].to_string()
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            "/".to_string()
        }
    }
}
