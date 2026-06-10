use std::str::FromStr;

use super::headers::Headers;
use super::path::Path;
use super::request_line::RequestLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Request {
    request_line: RequestLine,
    headers: Headers,
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, _body) = s.split_once("\r\n\r\n").ok_or(())?;
        let mut lines = head.split("\r\n");

        let request_line = RequestLine::from_str(lines.next().ok_or(())?)?;
        let headers = Headers::from_str(&lines.collect::<Vec<&str>>().join("\r\n"))?;

        Ok(Request { request_line, headers })
    }
}

impl Request {
    pub fn path(&self) -> &Path {
        self.request_line.target()
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }
}
