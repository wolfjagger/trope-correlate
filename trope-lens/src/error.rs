use std::io::{Error as IOError, IntoInnerError};
use csv::Error as CSVError;
use derive_more::Display;

use trope_lib::NamespaceParseError;


#[derive(Debug, Display)]
pub enum LensError {
  CSV(CSVError),
  IO(IOError),
  Parse(NamespaceParseError),
}

impl std::error::Error for LensError { }

impl From<CSVError> for LensError {
  fn from(err: CSVError) -> Self {
    LensError::CSV(err)
  }
}

impl From<IOError> for LensError {
  fn from(err: IOError) -> Self {
    LensError::IO(err)
  }
}

impl<T> From<IntoInnerError<T>> for LensError {
  fn from(err: IntoInnerError<T>) -> Self {
    LensError::IO(err.into_error())
  }
}

impl From<NamespaceParseError> for LensError {
  fn from(err: NamespaceParseError) -> Self {
    LensError::Parse(err)
  }
}
