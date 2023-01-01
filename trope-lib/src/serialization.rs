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


#[derive(Debug, Deserialize, Serialize)]
pub struct TropeGeneralJson {
  pub title: String,
  pub subpages: Vec<NamedLink>,
}

impl Default for TropeGeneralJson {
  fn default() -> Self {
    Self {
      title: String::default(),
      subpages: Vec::default(),
    }
  }
}
