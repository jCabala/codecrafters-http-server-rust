use std::fs;
use std::path::PathBuf;

use crate::message::{Request, Response, StatusLine};

pub fn handle(_req: &Request, directory: &str, filename: &str) -> Response {
    let mut path = PathBuf::from(directory);
    path.push(filename);

    match fs::read_to_string(path) {
        Ok(contents) => Response::new(StatusLine::success()).with_body("application/octet-stream", contents),
        Err(_) => Response::new(StatusLine::not_found()),
    }
}
