use std::io::{self, Read};
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

impl Request {
    fn parse_head(head: &str) -> Result<(RequestLine, Headers), ()> {
        let mut lines = head.split("\r\n");

        let request_line = RequestLine::from_str(lines.next().ok_or(())?)?;
        let headers = Headers::from_str(&lines.collect::<Vec<&str>>().join("\r\n"))?;

        Ok((request_line, headers))
    }

    /// Reads a full HTTP request (head + body) from the stream, accounting
    /// for requests that span multiple `read` calls. Returns `Ok(None)` if
    /// the connection was closed before any data was read.
    pub fn from_stream<R: Read>(stream: &mut R) -> io::Result<Option<Self>> {
        let mut data = Vec::new();
        let mut chunk = [0; 1024];

        let header_end = loop {
            if let Some(pos) = data.windows(4).position(|window| window == b"\r\n\r\n") {
                break pos + 4;
            }

            match stream.read(&mut chunk)? {
                0 if data.is_empty() => return Ok(None),
                0 => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "incomplete request",
                    ));
                }
                n => data.extend_from_slice(&chunk[..n]),
            }
        };

        let (request_line, headers) =
            Self::parse_head(&String::from_utf8_lossy(&data[..header_end - 4]))
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "malformed request"))?;

        let content_length = headers
            .get("Content-Length")
            .and_then(|len| len.parse::<usize>().ok())
            .unwrap_or(0);

        while data.len() < header_end + content_length {
            match stream.read(&mut chunk)? {
                0 => break,
                n => data.extend_from_slice(&chunk[..n]),
            }
        }

        let body_end = (header_end + content_length).min(data.len());
        let body = String::from_utf8_lossy(&data[header_end..body_end]).into_owned();

        Ok(Some(Request {
            request_line,
            headers,
            body,
        }))
    }

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

    pub fn is_final(&self) -> bool {
        self.headers
            .get("Connection")
            .map(|v| v.to_lowercase() == "close")
            .unwrap_or(false)
    }
}
