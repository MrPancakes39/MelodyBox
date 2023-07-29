#[derive(Debug, thiserror::Error)]
pub enum DownloadErrors {
    #[error("There was an Error in doing the Request")]
    RequestError(reqwest::Error),
    #[error("Couldn't Parse JSON")]
    ParseError,
    #[error("API returned an Error")]
    ApiError(String),
    #[error("No Audio Streams Available")]
    StreamError,
    #[error("Failed to Download to File")]
    DownloadFailed,
}

impl From<reqwest::Error> for DownloadErrors {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("There was an Error in doing the Request")]
    RequestError(reqwest::Error),
    #[error("Couldn't Parse JSON")]
    ParseError,
}

impl From<reqwest::Error> for IdError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}
