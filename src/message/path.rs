use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path {
    segments: Vec<String>,
}

impl Path {
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(String::from)
            .collect();
        Ok(Path { segments })
    }
}
