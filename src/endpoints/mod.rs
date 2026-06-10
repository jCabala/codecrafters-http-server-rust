mod echo;
mod files;
mod not_found;
mod root;

use std::io::Write;

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::message::{Method, Request, Response};

pub fn handle(req: &Request, directory: &str) -> Response {
    let path = req.path();
    let segments: Vec<&str> = path.segments().iter().map(String::as_str).collect();

    let mut res = match (req.method(), segments.as_slice()) {
        (Method::Get, []) => root::handle(req),
        (Method::Get, ["echo", value]) => echo::handle(req, value),
        (Method::Get, ["user-agent"]) => {
            echo::handle(req, req.headers().get("User-Agent").unwrap_or("Unknown"))
        }
        (Method::Get, ["files", filename]) => files::get(req, directory, filename),
        (Method::Post, ["files", filename]) => files::post(req, directory, filename),
        _ => not_found::handle(req),
    };

    res = if req.is_final() {
        res.into_final()
    } else {
        res
    };

    apply_encoding(req, res)
}

fn apply_encoding(req: &Request, mut res: Response) -> Response {
    if let Some(body) = &res.body
        && req.headers().contains("Accept-Encoding", "gzip")
    {
        let compressed = gzip(body);
        res = res
            .with_header("Content-Length".to_string(), compressed.len().to_string())
            .with_header("Content-Encoding".to_string(), "gzip".to_string());
        res.body = Some(compressed);
    }

    res
}

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}
