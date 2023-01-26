use derive_more::Display;


#[derive(Debug, Display)]
pub enum ScrapeError {
  File(String),
  Brotli(String),
  Other(String),
}

impl std::error::Error for ScrapeError { }

impl From<Box<dyn std::error::Error>> for ScrapeError {
  fn from(err: Box<dyn std::error::Error>) -> Self {
    ScrapeError::Other(err.to_string())
  }
}
