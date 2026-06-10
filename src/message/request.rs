use std::str::FromStr;

use super::headers::Headers;
use super::method::Method;
use super::path::Path;
use super::request_line::RequestLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: String,
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, body) = s.split_once("\r\n\r\n").ok_or(())?;
        let mut lines = head.split("\r\n");

        let request_line = RequestLine::from_str(lines.next().ok_or(())?)?;
        let headers = Headers::from_str(&lines.collect::<Vec<&str>>().join("\r\n"))?;

        let content_length = headers.get("Content-Length")
            .and_then(|len| len.parse::<usize>().ok())
            .unwrap_or(0);
        let body = body[..content_length.min(body.len())].to_string();

        Ok(Request { request_line, headers, body })
    }
}

impl Request {
    pub fn method(&self) -> &Method {
        self.request_line.method()
    }

    pub fn path(&self) -> &Path {
        self.request_line.target()
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
