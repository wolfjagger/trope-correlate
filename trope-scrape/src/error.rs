use std::io::Error as IOError;
use csv::Error as CSVError;
use serde_json::Error as SerdeError;
use derive_more::Display;


#[derive(Debug, Display)]
pub enum ScrapeError {
  File(String),
  Brotli(String),
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
