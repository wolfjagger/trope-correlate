use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::Namespace;


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropeLensArgs {
  #[command(subcommand)]
  pub method: TropeLensMethod
}

impl TropeLensArgs {
  pub fn parse_args() -> Self {
    log::info!("Parse trope-lens args");
    Self::parse()
  }
}

#[derive(Debug, Subcommand)]
pub enum TropeLensMethod {
  Categorize(TropeLensCategorize),
  Tutorial(TropeLensTutorial),
}


/// Categorize media based on mentioned tropes
#[derive(Debug, ClapArgs)]
pub struct TropeLensCategorize {

  /// Name
  #[clap(short, long, value_parser,)]
  pub pagename: String,

  /// Namespace of page
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// Overwrite existing trope directory if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeLensCategorize> for TropeLensArgs {
  fn from(method_args: TropeLensCategorize) -> Self {
    TropeLensArgs { method: TropeLensMethod::Categorize(method_args) }
  }
}

/// Runs dfdx tutorials
#[derive(Debug, ClapArgs)]
pub struct TropeLensTutorial {

}
impl From<TropeLensTutorial> for TropeLensArgs {
  fn from(method_args: TropeLensTutorial) -> Self {
    TropeLensArgs { method: TropeLensMethod::Tutorial(method_args) }
  }
}
