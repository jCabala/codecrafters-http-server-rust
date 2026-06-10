use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            _ => Err(()),
        }
    }
}
