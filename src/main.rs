#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::Write;

mod response;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
               let res = response::Response {
                   status: response::Status::Success,
               };

               _stream.write(res.to_string().as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
