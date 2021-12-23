use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;

mod util;
use util::{ get_content_type, get_path, read_file_chunks, is_file_exist };

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
    let is_exist = is_file_exist(file_path);
    let mut _chunks: Vec<u8> = vec![];
    let mut headers: Vec<&str> = vec![];
    if is_exist && !file_path.ends_with("/") {
        let mut buf = Vec::new();
        let file = File::open(file_path);
        match file.unwrap().read_to_end(&mut buf) {
            Ok(_) => println!("Read file ok"),
            Err(e) => println!("Failed readinf file: {}", e),
        }
        _chunks = read_file_chunks(&buf, 8);
        headers.push("HTTP/1.1 200 OK",)
    } else {
        let text: &str = &format!("{} Not Found", path);
        let text_u8: &[u8] = text.as_bytes();
        let text_vec: Vec<u8> = text_u8.to_vec();
        _chunks = read_file_chunks(&text_vec, 8);
        headers.push("HTTP/1.1 404",)
    }
    
    let mut content_type = get_content_type(path).to_owned();
    content_type = format!("Content-type: {}", content_type);
    headers.push(&content_type);
    headers.push("Transfer-Encoding: chunked");
    headers.push("\r\n");

    let mut response: Vec<u8> = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(_chunks);

    match stream.write(&response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    response(&get_path(&stream), stream);
}

