use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;

mod util;
use util::{ get_content_type, get_path, read_file_chunks };

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening for connections on port {}", 3001);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}

fn response(path: &str, mut stream: TcpStream) {
    let file_path: &str = &("./files".to_string() + path);
    println!("{}", file_path);
    let mut buf = Vec::new();
    let file = File::open(file_path);

    match file.unwrap().read_to_end(&mut buf) {
        Ok(_) => println!("Read file ok"),
        Err(e) => println!("Failed readinf file: {}", e),
    }
    let chunks = read_file_chunks(&buf, 8);
    let mut headers: Vec<&str> = vec![
        "HTTP/1.1 200 OK",
    ];
    let mut content_type = get_content_type(path).to_owned();
    content_type = format!("Content-type: {}", content_type);
    headers.push(&content_type);
    headers.push("Transfer-Encoding: chunked");
    headers.push("\r\n");

    let mut response: Vec<u8> = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(chunks);

    match stream.write(&response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    response(&get_path(&stream), stream);
}

