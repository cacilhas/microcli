#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{:?}", self)]
    Missing(String),

    #[error("{:?}", self)]
    InvalidVersion(String),

    #[error("{:?}", self)]
    WrongLength { expected: usize, got: usize },
}
