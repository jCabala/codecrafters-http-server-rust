#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Success = 200,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Response {
    pub status: Status,
}

impl Response {
    pub fn to_string(&self) -> String {
        format!("HTTP/1.1 {} OK\r\n\r\n", self.status as u16)
    }
}