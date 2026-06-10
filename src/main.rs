use std::io::Write;
use std::net::TcpListener;

mod endpoints;
mod message;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let directory = args
        .iter()
        .position(|arg| arg == "--directory")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| ".".to_string());

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let directory = directory.clone();
        std::thread::spawn(move || match stream {
            Ok(mut _stream) => loop {
                let req = match message::Request::from_stream(&mut _stream) {
                    Ok(Some(req)) => req,
                    Ok(None) => break,
                    Err(e) => {
                        println!("read error: {}", e);
                        break;
                    }
                };

                let res = endpoints::handle(&req, &directory);

                if let Err(e) = _stream.write_all(&res.to_bytes()) {
                    println!("write error: {}", e);
                    break;
                }

                if res.is_final() {
                    break;
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
