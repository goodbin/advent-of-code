pub mod combinator;
pub mod fs;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] fs::IoError),
    #[error("error: {0}")]
    Parse(String),
}
