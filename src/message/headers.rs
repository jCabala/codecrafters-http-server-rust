use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Headers {
    entries: Vec<(String, String)>,
}

impl Headers {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries.iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    pub fn contains(&self, name: &str, value: &str) -> bool {
        self.get(name)
            .map(|header| header.split(',').any(|item| item.trim().eq_ignore_ascii_case(value)))
            .unwrap_or(false)
    }

    pub fn insert(&mut self, name: String, value: String) {
        self.entries.push((name, value));
    }

    pub fn set(&mut self, name: String, value: String) {
        match self.entries.iter_mut().find(|(key, _)| key.eq_ignore_ascii_case(&name)) {
            Some(entry) => entry.1 = value,
            None => self.entries.push((name, value)),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.entries.iter()
    }
}

impl FromStr for Headers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut headers = Headers::new();

        for line in s.split("\r\n") {
            if line.is_empty() {
                continue;
            }

            let (name, value) = line.split_once(':').ok_or(())?;
            headers.insert(name.trim().to_string(), value.trim().to_string());
        }

        Ok(headers)
    }
}
