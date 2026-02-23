use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("TSETMC API error: {0}")]
    Api(String),

    #[error("Data parsing error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
