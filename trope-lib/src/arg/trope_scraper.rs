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
}


/// Downloads index pages in bulk from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeScrapePagelist {

  /// If enabled, save an encrypted version of the html
  #[clap(short, long, value_parser, default_value_t = false)]
  pub encrypted: bool,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = Pagetype::Trope.to_string())]
  pub pagetype: String,

  /// Max number of pages to call for
  #[clap(short, long, value_parser, default_value_t = 2)]
  pub max_pages: u8,

}


/// Downloads index pages in bulk from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeScrapeTropePage {

  /// If enabled, save an encrypted version of the html
  #[clap(short, long, value_parser, default_value_t = false)]
  pub encrypted: bool,

  /// Trope name
  #[clap(short, long, value_parser,)]
  pub name: String,

  /// Trope url
  #[clap(short, long, value_parser,)]
  pub url: String,

}
