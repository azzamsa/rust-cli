use std::path::PathBuf;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum Error {
    #[error("No such file {0:?}")]
    FileNotFound(PathBuf),

    #[error("Failed to read the file {0:?}")]
    FileUnreadable(String),
}
