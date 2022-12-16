use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct NamedLink {
  pub name: String,
  pub url: String,
}

impl fmt::Display for NamedLink {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{},{}]", self.name, self.url)
  }
}
