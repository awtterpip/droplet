use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered error while trying to parse config: {0}")]
    InvalidConfig(#[from] toml::de::Error),
    #[error("Encountered an IO error: {0}")]
    IoError(#[from] std::io::Error),
}
