mod root;
mod echo;
mod not_found;

use crate::message::{Request, Response};

pub fn handle(req: &Request) -> Response {
    let path = req.path();
    let segments: Vec<&str> = path.segments().iter().map(String::as_str).collect();

    match segments.as_slice() {
        [] => root::handle(req),
        ["echo", value] => echo::handle(req, value),
        _ => not_found::handle(req),
    }
}
