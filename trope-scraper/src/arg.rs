use clap::Parser;

use trope_lib;


/// Downloads index pages in bulk from tvtropes.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

  /// If enabled, save an encrypted version of the html
  #[clap(short, long, value_parser, default_value_t = false)]
  pub encrypted: bool,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = trope_lib::Namespace::Main.to_string())]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = trope_lib::Pagetype::Trope.to_string())]
  pub pagetype: String,

  /// Max number of pages to call for
  #[clap(short, long, value_parser, default_value_t = 2)]
  pub max_pages: u8,

}

impl Args {
  pub fn parse_args() -> Self {
    Self::parse()
  }
}
