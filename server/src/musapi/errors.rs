#[derive(Debug, thiserror::Error)]
pub enum RequestorError {
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

impl From<reqwest::Error> for RequestorError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}
