#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Success = 200,
    Created = 201,
    NotFound = 404,
}
