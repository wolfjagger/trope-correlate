use std::str::FromStr;


pub enum Pagetype {
  Trope,
}
impl ToString for Pagetype {
  fn to_string(&self) -> String {
    match self {
      Pagetype::Trope => "trope".to_string()
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PagetypeParseError;

impl FromStr for Pagetype {
  type Err = PagetypeParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "trope" => Ok(Pagetype::Trope),
      _ => Err(PagetypeParseError)
    }
  }
}
