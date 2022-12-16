mod arg;

use std::str::FromStr;

pub use arg::{
  TropeDownloadArgs, TropeDownloadMethod, TropeDownloadPagelist,
  TropeScraperArgs, TropeScraperMethod, TropeScraperPagelist,
};

pub const DATA_DIR: &str = "test_data";

pub enum Namespace {
  Main,
}

impl ToString for Namespace {
  fn to_string(&self) -> String {
    match self {
      Namespace::Main => "main".to_string()
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceParseError;

impl FromStr for Namespace {
  type Err = NamespaceParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "main" => Ok(Namespace::Main),
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
