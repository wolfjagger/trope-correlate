use std::io::{Error as IOError, IntoInnerError};
use derive_more::Display;


#[derive(Debug, Display)]
pub enum DownloadError {
  Brotli(IOError),
  IO(IOError),
}

impl std::error::Error for DownloadError { }

impl From<IOError> for DownloadError {
  fn from(err: IOError) -> Self {
    DownloadError::IO(err)
  }
}

impl<T> From<IntoInnerError<T>> for DownloadError {
  fn from(err: IntoInnerError<T>) -> Self {
    DownloadError::IO(err.into_error())
  }
}
