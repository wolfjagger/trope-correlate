use clap::Parser;


/// Downloads index pages in bulk from tvtropes.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = String::from("Main"))]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = String::from("trope"))]
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
