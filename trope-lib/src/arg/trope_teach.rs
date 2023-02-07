use std::path::PathBuf;
use clap::{Args as ClapArgs, Parser, Subcommand};


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

  /// File for input model
  #[clap(short, long)]
  pub in_model: Option<PathBuf>,

  /// File for output model
  #[clap(short, long)]
  pub out_model: PathBuf,

  /// Training parameters
  #[clap(short, long)]
  pub training_params: PathBuf,

  /// Overwrite existing model if enabled (default: false)
  #[clap(short, long, default_value_t = false)]
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
