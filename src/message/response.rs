use super::headers::Headers;
use super::status_line::StatusLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Response {
    pub status: StatusLine,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: StatusLine) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn with_body(mut self, content_type: &str, body: String) -> Self {
        self.headers
            .insert("Content-Type".to_string(), content_type.to_string());
        self.headers
            .insert("Content-Length".to_string(), body.len().to_string());
        self.body = Some(body.into_bytes());
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status_str = self.status.to_string();
        let headers_str: String = self
            .headers
            .iter()
            .map(|(name, value)| format!("{}: {}\r\n", name, value))
            .collect();

        let mut bytes = format!("{}\r\n{}\r\n", status_str, headers_str).into_bytes();
        if let Some(body) = &self.body {
            bytes.extend_from_slice(body);
        }
        bytes
    }

    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.set(name, value);
        self
    }

    pub fn is_final(&self) -> bool {
        self.headers
            .get("Connection")
            .map(|v| v.to_lowercase() == "close")
            .unwrap_or(false)
    }

    pub fn into_final(mut self) -> Self {
        self.headers
            .insert("Connection".to_string(), "close".to_string());
        self
    }
}
