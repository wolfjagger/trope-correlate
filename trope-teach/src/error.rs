use std::io::{Error as IOError, IntoInnerError};
use csv::Error as CSVError;
use derive_more::Display;
use dfdx::numpy::NpyError;

use trope_lib::NamespaceParseError;


#[derive(Debug, Display)]
pub enum TeachError {
  CSV(CSVError),
  IO(IOError),
  Parse(NamespaceParseError),
  Npy(NpyError),
}

impl std::error::Error for TeachError { }

impl From<CSVError> for TeachError {
  fn from(err: CSVError) -> Self {
    TeachError::CSV(err)
  }
}

impl From<IOError> for TeachError {
  fn from(err: IOError) -> Self {
    TeachError::IO(err)
  }
}

impl<T> From<IntoInnerError<T>> for TeachError {
  fn from(err: IntoInnerError<T>) -> Self {
    TeachError::IO(err.into_error())
  }
}

impl From<NamespaceParseError> for TeachError {
  fn from(err: NamespaceParseError) -> Self {
    TeachError::Parse(err)
  }
}

impl From<NpyError> for TeachError {
  fn from(err: NpyError) -> Self {
    TeachError::Npy(err)
  }
}
