use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod lib;
use crate::lib::ThreadPool;

fn main() {
    let url = "127.0.0.1:3001";
    let listener = TcpListener::bind(url).unwrap();
    let pool = ThreadPool::new(4);
    println!("The http server is starting at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n",
            "./html/hello.html",
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n",
            "./html/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n",
            "./html/404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.as_bytes().len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
