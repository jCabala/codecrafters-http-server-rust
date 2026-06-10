use crate::message::{Request, Response, StatusLine};

pub fn handle(_req: &Request, value: &str) -> Response {
    Response::new(StatusLine::success()).with_body("text/plain", value.to_string())
}
