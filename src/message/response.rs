use super::status_line::StatusLine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Response {
    pub status: StatusLine,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let status_str = self.status.to_string();
        format!("{}\r\n\r\n", status_str)
    }
}
