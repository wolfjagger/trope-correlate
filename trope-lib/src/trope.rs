use std::fmt;


#[derive(Debug)]
pub struct Trope {
  pub name: String,
  pub url: String,
}

impl fmt::Display for Trope {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{},{}]", self.name, self.url)
  }
}
