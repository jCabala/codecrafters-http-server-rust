use std::str::FromStr;

use super::request_line::RequestLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Request {
    request_line: RequestLine,
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\r\n");

        let request_line = RequestLine::from_str(lines.next().ok_or(())?)?;
        Ok(Request { request_line })
    }
}

impl Request {
    pub fn target(&self) -> &str {
        self.request_line.target()
    }
}
