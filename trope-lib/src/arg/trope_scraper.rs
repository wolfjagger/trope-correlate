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
}


/// Scrapes downloaded pagelists for tropelist
#[derive(Debug, ClapArgs)]
pub struct TropeScrapePagelist {

  /// If enabled, assume an encrypted version of the html
  #[clap(long, value_parser, default_value_t = true)]
  pub encrypted: bool,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = Pagetype::Trope.to_string())]
  pub pagetype: String,

  /// Min number of pages to scrape (inclusive; known min: 1)
  #[clap(short, long, value_parser,)]
  pub beg_page: u8,

  /// Max number of pages to scrape (inclusive; known max: 13)
  #[clap(short, long, value_parser,)]
  pub end_page: u8,

}


/// Scrapes downloaded trope page
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropePage {

  /// If enabled, assume an encrypted version of the html
  #[clap(long, value_parser, default_value_t = true)]
  pub encrypted: bool,

  /// Trope name
  #[clap(short, long, value_parser,)]
  pub name: String,

}


/// Downloads trope pages in tropelist from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropelist {

  /// If enabled, assume an encrypted version of the html
  #[clap(long, value_parser, default_value_t = true)]
  pub encrypted: bool,

  /// Path to tropelist
  #[clap(short, long, value_parser,)]
  pub in_path: path::PathBuf,

  /// Min number of records to scrape (inclusive; known min: 0)
  #[clap(short, long, value_parser,)]
  pub beg_record: u8,

  /// Max number of records to scrape (inclusive; unknown max)
  #[clap(short, long, value_parser,)]
  pub end_record: u8,

}
