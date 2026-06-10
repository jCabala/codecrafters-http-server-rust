use crate::message::{Request, Response, StatusLine};

pub fn handle(_req: &Request) -> Response {
    Response::new(StatusLine::success())
}
