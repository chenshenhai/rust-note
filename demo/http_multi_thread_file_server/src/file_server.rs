
use std::net::{TcpStream};
use std::io::{Read};
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
 
pub fn get_path(mut stream: &TcpStream) -> String {
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

pub fn is_file_exist(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn get_content_type(path: &str) -> String  {
    let mut map = HashMap::new();
    map.insert("html", "text/html");
    map.insert("text", "text/plain");
    map.insert("js", "text/javascript");
    map.insert("png", "image/png");
    map.insert("jpg", "image/jpeg");
    map.insert("css", "text/css");
    let names: Vec<&str> = path.split('.').collect();
    let extname = names.last().unwrap();
    let content_type = map.get(extname.to_owned());
    return content_type.unwrap_or(&"text/html").to_string();
}


pub fn read_file_chunks(buf: &Vec<u8>, size: usize) -> Vec<u8> {
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

pub fn handle_client(stream: TcpStream) {
  response(&get_path(&stream), stream);
}

