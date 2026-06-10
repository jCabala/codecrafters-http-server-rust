use std::fs;
use std::path::PathBuf;

use crate::message::{Request, Response, StatusLine};

pub fn get(_req: &Request, directory: &str, filename: &str) -> Response {
    let path = PathBuf::from(directory).join(filename);

    match fs::read_to_string(path) {
        Ok(contents) => {
            Response::new(StatusLine::success()).with_body("application/octet-stream", contents)
        }
        Err(_) => Response::new(StatusLine::not_found()),
    }
}

pub fn post(req: &Request, directory: &str, filename: &str) -> Response {
    let path = PathBuf::from(directory).join(filename);

    fs::write(path, req.body()).unwrap();
    Response::new(StatusLine::created())
}
