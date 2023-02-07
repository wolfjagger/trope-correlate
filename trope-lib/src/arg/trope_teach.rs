use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::Namespace;


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropeTeachArgs {
  #[command(subcommand)]
  pub method: TropeTeachMethod
}

impl TropeTeachArgs {
  pub fn parse_args() -> Self {
    log::info!("Parse trope-teach args");
    Self::parse()
  }
}

#[derive(Debug, Subcommand)]
pub enum TropeTeachMethod {
  Categorize(TropeTeachCategorize),
  Tutorial(TropeTeachTutorial),
}


/// Categorize media based on mentioned tropes
#[derive(Debug, ClapArgs)]
pub struct TropeTeachCategorize {

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
impl From<TropeTeachCategorize> for TropeTeachArgs {
  fn from(method_args: TropeTeachCategorize) -> Self {
    TropeTeachArgs { method: TropeTeachMethod::Categorize(method_args) }
  }
}

/// Runs dfdx tutorials
#[derive(Debug, ClapArgs)]
pub struct TropeTeachTutorial {

}
impl From<TropeTeachTutorial> for TropeTeachArgs {
  fn from(method_args: TropeTeachTutorial) -> Self {
    TropeTeachArgs { method: TropeTeachMethod::Tutorial(method_args) }
  }
}
