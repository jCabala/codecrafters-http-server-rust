#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str::FromStr;

mod endpoints;
mod message;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Read the request
                let mut buffer = [0; 1024];
                _stream.read(&mut buffer).unwrap();
                let request_str = String::from_utf8_lossy(&buffer);
                let req = message::Request::from_str(&request_str).unwrap();

                let res = endpoints::handle(&req);

                _stream.write(res.to_string().as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
