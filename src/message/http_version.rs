use std::string::ToString;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HTTPVersion {
    HTTP1_1,
}

impl ToString for HTTPVersion {
    fn to_string(&self) -> String {
        match self {
            HTTPVersion::HTTP1_1 => "HTTP/1.1".to_string(),
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
