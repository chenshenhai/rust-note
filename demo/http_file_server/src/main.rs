use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;

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
    let file_path: &str = &("./files".to_string() + path);
    println!("{}", file_path);
    let mut buf = Vec::new();
    let file = File::open(file_path);

    match file.unwrap().read_to_end(&mut buf) {
        Ok(_) => println!("Read file ok"),
        Err(e) => println!("Failed readinf file: {}", e),
    }
    
    // println!("buf = {:?}", buf);
    let chunks = read_file_chunks(&buf, 8);
    // println!("chunks = {:?}", chunks);
    let mut headers = vec![
        "HTTP/1.1 200 OK",
    ];

    if path.ends_with(".html") {
        headers.push("Content-type: text/html")
    } else if path.ends_with(".png") {
        headers.push("Content-type: image/png")
    } else {
        headers.push("Content-type: text/plain")
    }
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


fn read_file_chunks(buf: &Vec<u8>, size: usize) -> Vec<u8> {
    let size_code: u8 = (size + 48) as u8;
    let r_code = 13;
    let n_code = 10;
    let mut _rest = buf.len() % size;
    let mut chunks: Vec<u8> = vec![];
    chunks.push(size_code);
    chunks.push(r_code);
    chunks.push(n_code);
    let mut i = 0;
    while i < buf.len() { 
        _rest = i % size;
        if _rest == 0 && i >= size as usize  {
            if buf.len() - i <= size as usize {
                chunks.push(r_code);
                chunks.push(n_code);
                chunks.push((buf.len() - i) as u8 + 48);
                chunks.push(r_code);
                chunks.push(n_code);
            } else {
                chunks.push(r_code);
                chunks.push(n_code);
                chunks.push(size_code);
                chunks.push(r_code);
                chunks.push(n_code);
            } 
        }
        chunks.push(buf[i]);
        i += 1;
    }
    chunks.push(r_code);
    chunks.push(n_code);
    chunks.push(48);
    chunks.push(r_code);
    chunks.push(n_code);
    chunks.push(r_code);
    chunks.push(n_code);
    return chunks;
}