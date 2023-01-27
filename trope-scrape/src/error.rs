use std::io::{Error as IOError, IntoInnerError};
use csv::Error as CSVError;
use serde_json::Error as SerdeError;
use derive_more::Display;


#[derive(Debug, Display)]
pub enum ScrapeError {
  Brotli(IOError),
  IO(IOError),
  CSV(CSVError),
  Serde(SerdeError),
}

impl std::error::Error for ScrapeError { }

impl From<IOError> for ScrapeError {
  fn from(err: IOError) -> Self {
    ScrapeError::IO(err)
  }
}

impl<T> From<IntoInnerError<T>> for ScrapeError {
  fn from(err: IntoInnerError<T>) -> Self {
    ScrapeError::IO(err.into_error())
  }
}

impl From<CSVError> for ScrapeError {
  fn from(err: CSVError) -> Self {
    ScrapeError::CSV(err)
  }
}

impl From<SerdeError> for ScrapeError {
  fn from(err: SerdeError) -> Self {
    ScrapeError::Serde(err)
  }
}
