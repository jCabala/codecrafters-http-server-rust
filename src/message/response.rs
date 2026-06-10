use super::headers::Headers;
use super::status_line::StatusLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Response {
    pub status: StatusLine,
    pub headers: Headers,
    pub body: Option<String>,
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
        self.headers.insert("Content-Type".to_string(), content_type.to_string());
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self.body = Some(body);
        self
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let status_str = self.status.to_string();
        let headers_str: String = self.headers.iter()
            .map(|(name, value)| format!("{}: {}\r\n", name, value))
            .collect();
        let body_str = self.body.as_deref().unwrap_or("");
        format!("{}\r\n{}\r\n{}", status_str, headers_str, body_str)
    }
}
