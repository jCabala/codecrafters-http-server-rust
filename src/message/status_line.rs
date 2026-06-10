use std::string::ToString;

use super::http_version::HTTPVersion;
use super::status::Status;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusLine {
    pub version: HTTPVersion,
    pub status: Status,
    pub reason_phrase: Option<String>,
}

impl StatusLine {
    pub fn success() -> Self {
        Self {
            version: HTTPVersion::HTTP1_1,
            status: Status::Success,
            reason_phrase: Some("OK".to_string()),
        }
    }

    pub fn created() -> Self {
        Self {
            version: HTTPVersion::HTTP1_1,
            status: Status::Created,
            reason_phrase: Some("Created".to_string()),
        }
    }

    pub fn not_found() -> Self {
        Self {
            version: HTTPVersion::HTTP1_1,
            status: Status::NotFound,
            reason_phrase: Some("Not Found".to_string()),
        }
    }
}

impl ToString for StatusLine {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.version.to_string(), self.status.clone() as u16, self.reason_phrase.as_ref().unwrap_or(&"".to_string()))
    }
}
