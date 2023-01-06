use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::{Namespace, Pagetype};


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropeScrapeArgs {
  #[command(subcommand)]
  pub method: TropeScrapeMethod
}

impl TropeScrapeArgs {
  pub fn parse_args() -> Self {
    Self::parse()
  }
}

#[derive(Debug, Subcommand)]
pub enum TropeScrapeMethod {
  Namespace(TropeScrapeNamespace),
  Pagelist(TropeScrapePagelist),
  NamespaceTotPages(TropeScrapeNamespaceTotPages),
  TropePage(TropeScrapeTropePage),
  Tropelist(TropeScrapeTropelist),
  AllTropes(TropeScrapeAllTropes),
}


/// Scrapes downloaded namespace pagelists for tropelist
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeNamespace {

  /// Min number of pages to scrape (inclusive; known min: 1)
  #[clap(short, long, value_parser,)]
  pub beg_page: u8,

  /// Max number of pages to scrape (inclusive; known max: 58)
  #[clap(short, long, value_parser,)]
  pub end_page: u8,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing tropelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeScrapeNamespace> for TropeScrapeArgs {
  fn from(method_args: TropeScrapeNamespace) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::Namespace(method_args) }
  }
}


/// Scrapes downloaded pagelists for tropelist
#[derive(Debug, ClapArgs)]
pub struct TropeScrapePagelist {

  /// Min number of pages to scrape (inclusive; known min: 1)
  #[clap(short, long, value_parser,)]
  pub beg_page: u8,

  /// Max number of pages to scrape (inclusive; known max: 58)
  #[clap(short, long, value_parser,)]
  pub end_page: u8,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = Pagetype::Trope.to_string())]
  pub pagetype: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing tropelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeScrapePagelist> for TropeScrapeArgs {
  fn from(method_args: TropeScrapePagelist) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::Pagelist(method_args) }
  }
}


/// Scrapes downloaded namespace page 1 and reports number of pages in namespace
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeNamespaceTotPages {

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

}
impl From<TropeScrapeNamespaceTotPages> for TropeScrapeArgs {
  fn from(method_args: TropeScrapeNamespaceTotPages) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::NamespaceTotPages(method_args) }
  }
}


/// Scrapes downloaded trope page
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropePage {

  /// Trope name
  #[clap(short, long, value_parser,)]
  pub name: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing trope directory if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeScrapeTropePage> for TropeScrapeArgs {
  fn from(method_args: TropeScrapeTropePage) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::TropePage(method_args) }
  }
}


/// Scrapes downloaded trope pages specified in tropelist
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropelist {

  /// Min number of records to scrape (inclusive; known min: 0)
  #[clap(short, long, value_parser,)]
  pub beg_record: u64,

  /// Max number of records to scrape (inclusive; unknown max)
  #[clap(short, long, value_parser,)]
  pub end_record: u64,

  /// Namespace for of tropelist, to find correct directory
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing trope directory if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

  /// If a seed is given, scrape pages out-of-order (default: None)
  /// This will be in the same order as downloaded with the same seed
  #[clap(short, long, value_parser, required = false)]
  pub random_seed: Option<u64>,

}
impl From<TropeScrapeTropelist> for TropeScrapeArgs {
  fn from(method_args: TropeScrapeTropelist) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::Tropelist(method_args) }
  }
}


/// Scrapes downloaded trope pages that exist in the tropes directory
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeAllTropes {

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing trope directory if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeScrapeAllTropes> for TropeScrapeArgs {
  fn from(method_args: TropeScrapeAllTropes) -> Self {
    TropeScrapeArgs { method: TropeScrapeMethod::AllTropes(method_args) }
  }
}
