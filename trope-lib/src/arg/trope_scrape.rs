use std::path;

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
  Pagelist(TropeScrapePagelist),
  TropePage(TropeScrapeTropePage),
  Tropelist(TropeScrapeTropelist),
  AllTropes(TropeScrapeAllTropes),
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


/// Scrapes downloaded trope pages specified in tropelist
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropelist {

  /// Path to tropelist
  #[clap(short, long, value_parser,)]
  pub in_path: path::PathBuf,

  /// Min number of records to scrape (inclusive; known min: 0)
  #[clap(short, long, value_parser,)]
  pub beg_record: u64,

  /// Max number of records to scrape (inclusive; unknown max)
  #[clap(short, long, value_parser,)]
  pub end_record: u64,

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
