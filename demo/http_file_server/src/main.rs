use chunked_transfer;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;
use chunked_transfer::Encoder;

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

fn get_path(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
            println!("GET {}", path[1]);
            // println!("{}", req_str);
            path[1].to_string()
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            "/".to_string()
        }
    }
}

fn response(path: &str, mut stream: TcpStream) {
    let file_path: &str = &("./image".to_string() + path);
    println!("{}", file_path);
    let mut buf = Vec::new();
    let file = File::open(file_path);
    // file.unwrap().read_to_end(&mut buf);

    match file.unwrap().read_to_end(&mut buf) {
        Ok(_) => println!("Read file ok"),
        Err(e) => println!("Failed readinf file: {}", e),
    }

    let mut encoded: Vec<u8> = vec![];
    {
        let mut encoder = Encoder::with_chunks_size(&mut encoded, 8);
        // encoder.write_all(&buf);
        match encoder.write_all(&buf) {
            Ok(_) => println!("write_all ok"),
            Err(e) => println!("Failed write_all {}", e),
        }
    }

    let headers =
        ["HTTP/1.1 200 OK", "Content-type: image/png", "Transfer-Encoding: chunked", "\r\n"];
    let mut response: Vec<u8> = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(encoded);

    match stream.write(&response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    response(&get_path(&stream), stream);
}