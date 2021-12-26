use std::net::TcpListener;

mod mutil_thread;
mod file_server;
use crate::mutil_thread::ThreadPool;
use crate::file_server::{ handle_client };

fn main() {
    let url = "127.0.0.1:3001";
    let listener = TcpListener::bind(url).unwrap();
    let pool = ThreadPool::new(4);
    println!("The http server is starting at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

