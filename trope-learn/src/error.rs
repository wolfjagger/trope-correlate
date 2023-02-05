use std::io::{Error as IOError, IntoInnerError};
use csv::Error as CSVError;
use derive_more::Display;

use trope_lib::NamespaceParseError;


#[derive(Debug, Display)]
pub enum LearnError {
  CSV(CSVError),
  IO(IOError),
  Parse(NamespaceParseError),
}

impl std::error::Error for LearnError { }

impl From<CSVError> for LearnError {
  fn from(err: CSVError) -> Self {
    LearnError::CSV(err)
  }
}

impl From<IOError> for LearnError {
  fn from(err: IOError) -> Self {
    LearnError::IO(err)
  }
}

impl<T> From<IntoInnerError<T>> for LearnError {
  fn from(err: IntoInnerError<T>) -> Self {
    LearnError::IO(err.into_error())
  }
}

impl From<NamespaceParseError> for LearnError {
  fn from(err: NamespaceParseError) -> Self {
    LearnError::Parse(err)
  }
}
