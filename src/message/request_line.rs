use std::str::FromStr;

use super::http_version::HTTPVersion;
use super::method::Method;
use super::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestLine {
    method: Method,
    target: Path,
    version: HTTPVersion,
}

impl FromStr for RequestLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(());
        }

        let method = Method::from_str(parts[0])?;
        let target = Path::from_str(parts[1])?;
        let version = HTTPVersion::from_str(parts[2])?;
        Ok(RequestLine { method, target, version })
    }
}

impl RequestLine {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn target(&self) -> &Path {
        &self.target
    }
}
