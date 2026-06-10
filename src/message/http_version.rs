use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HTTPVersion {
    HTTP1_1,
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTTPVersion::HTTP1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

impl FromStr for HTTPVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HTTPVersion::HTTP1_1),
            _ => Err(()),
        }
    }
}
