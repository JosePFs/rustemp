#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    NotFound(String),
    BadRequest(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e.to_string()),
            Error::Serde(e) => write!(f, "Serde error: {}", e.to_string()),
            Error::Io(e) => write!(f, "IO error: {}", e.to_string()),
            Error::NotFound(e) => write!(f, "Not found error: {}", e),
            Error::BadRequest(e) => write!(f, "Bad request error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Serde(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            error.to_string(),
        ))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
