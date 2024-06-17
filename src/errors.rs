#[derive(Debug)]
pub(crate) enum AppError {
    FileIoError(std::io::Error),
    JsonParsingError(serde_json::Error),
    HttpError(reqwest::Error),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::FileIoError(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::JsonParsingError(error)
    }
}
