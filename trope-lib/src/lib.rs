use std::str::FromStr;

pub const DATA_DIR: &str = "test_data";

pub enum Namespace {
  Main,
}

impl ToString for Namespace {
  fn to_string(&self) -> String {
    match self {
      Namespace::Main => "Main".to_string()
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceParseError;

impl FromStr for Namespace {
  type Err = NamespaceParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Main" => Ok(Namespace::Main),
      _ => Err(NamespaceParseError)
    }
  }
}

pub enum Pagetype {
  Trope,
}
impl ToString for Pagetype {
  fn to_string(&self) -> String {
    match self {
      Pagetype::Trope => "Trope".to_string()
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PagetypeParseError;

impl FromStr for Pagetype {
  type Err = PagetypeParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "trope" => Ok(Pagetype::Trope),
      _ => Err(PagetypeParseError)
    }
  }
}
