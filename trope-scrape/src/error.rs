use derive_more::Display;


#[derive(Debug, Display)]
pub enum ScrapeError {
  FileError(String),
  BrotliError(String),
}

impl std::error::Error for ScrapeError { }
