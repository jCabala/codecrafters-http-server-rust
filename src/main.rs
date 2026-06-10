#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str::FromStr;

mod endpoints;
mod message;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let directory = args.iter()
        .position(|arg| arg == "--directory")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| ".".to_string());

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let directory = directory.clone();
        std::thread::spawn(move || {
            match stream {
                Ok(mut _stream) => {
                    // Read the request
                    let mut buffer = [0; 1024];
                    _stream.read(&mut buffer).unwrap();
                    let request_str = String::from_utf8_lossy(&buffer);
                    let req = message::Request::from_str(&request_str).unwrap();

                    let res = endpoints::handle(&req, &directory);

                    _stream.write(res.to_string().as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
