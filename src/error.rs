use std::fmt::{Display, Formatter};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    NoLyrics,
    Request(#[from] reqwest::Error),
    BadFetcher(String),
    BadConfig(#[from] toml::de::Error),
    IO(#[from] std::io::Error),
    Lofty(#[from] lofty::LoftyError),
    InvalidTags,
    NoMatches,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoLyrics => write!(f, "The lyrics fetcher returned an empty string"),
            Error::Request(ref err) => write!(f, "Failed to execute request: {err}"),
            Error::BadFetcher(ref err) => write!(f, "Unknown fetcher(s): {err}"),
            Error::BadConfig(ref err) => write!(f, "Failed to read config: {err}"),
            Error::IO(ref err) => write!(f, "An IO error occurred: {err}"),
            Error::Lofty(ref err) => write!(f, "Failed to read file tags: {err}"),
            Error::InvalidTags => write!(
                f,
                "The provided file does not have a title or artist available"
            ),
            Error::NoMatches => write!(f, "No match found in fetcher's output"),
        }
    }
}
