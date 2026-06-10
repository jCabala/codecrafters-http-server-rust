mod root;
mod echo;
mod files;
mod not_found;

use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::message::{Method, Request, Response};

pub fn handle(req: &Request, directory: &str) -> Response {
    let path = req.path();
    let segments: Vec<&str> = path.segments().iter().map(String::as_str).collect();

    let res = match (req.method(), segments.as_slice()) {
        (Method::GET, []) => root::handle(req),
        (Method::GET, ["echo", value]) => echo::handle(req, value),
        (Method::GET, ["user-agent"]) => echo::handle(req, req.headers().get("User-Agent").unwrap_or(&"Unknown".to_string())),
        (Method::GET, ["files", filename]) => files::get(req, directory, filename),
        (Method::POST, ["files", filename]) => files::post(req, directory, filename),
        _ => not_found::handle(req),
    };

    apply_encoding(req, res)
}

fn apply_encoding(req: &Request, mut res: Response) -> Response {
    if let Some(body) = &res.body {
        if req.headers().contains("Accept-Encoding", "gzip") {
            let compressed = gzip(body);
            res.headers.set("Content-Length".to_string(), compressed.len().to_string());
            res.headers.set("Content-Encoding".to_string(), "gzip".to_string());
            res.body = Some(compressed);
        }
    }

    res
}

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}
