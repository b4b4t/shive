#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    NotFound(String),
    BadRequest(String),
}
