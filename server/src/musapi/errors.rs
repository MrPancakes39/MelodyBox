use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

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

impl IntoResponse for RequestorError {
    fn into_response(self) -> Response {
        log::error!("ERROR: {:?}", self);
        let message = match self {
            Self::RequestError(_) | Self::ParseError => None,
            Self::ApiError(_) | Self::StreamError => {
                Some("Couldn't get song source from Youtube Music.")
            }
            Self::DownloadFailed => Some("Failed to download song from Youtube Music."),
        };
        match message {
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Some(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
