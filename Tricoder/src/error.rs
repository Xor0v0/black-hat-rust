use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Usage: tricoder <qq.com>")]
    CliUsage,
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value.to_string())
    }
}
