use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The lyrics fetcher returned an empty string")]
    NoLyrics,
    #[error("Failed to execute request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Unknown fetcher(s): {0}")]
    BadFetcher(String),
    #[error("Failed to read config: {0}")]
    BadConfig(#[from] toml::de::Error),
    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to read file tags: {0}")]
    Lofty(#[from] lofty::LoftyError),
    #[error("The provided file does not have a title or artist available")]
    InvalidTags,
}
