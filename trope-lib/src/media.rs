use std::fmt;


#[derive(Debug)]
pub struct Media {
  pub name: String,
  pub url: String,
}

impl fmt::Display for Media {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{},{}]", self.name, self.url)
  }
}
