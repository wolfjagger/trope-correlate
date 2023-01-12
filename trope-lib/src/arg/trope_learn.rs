use clap::{Args as ClapArgs, Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropeLearnArgs {
  #[command(subcommand)]
  pub method: TropeLearnMethod
}

impl TropeLearnArgs {
  pub fn parse_args() -> Self {
    Self::parse()
  }
}

#[derive(Debug, Subcommand)]
pub enum TropeLearnMethod {
  Categorize(TropeLearnCategorize),
  Tutorial(TropeLearnTutorial),
}


/// Categorize media based on mentioned tropes
#[derive(Debug, ClapArgs)]
pub struct TropeLearnCategorize {

}
impl From<TropeLearnCategorize> for TropeLearnArgs {
  fn from(method_args: TropeLearnCategorize) -> Self {
    TropeLearnArgs { method: TropeLearnMethod::Categorize(method_args) }
  }
}

/// Runs dfdx tutorials
#[derive(Debug, ClapArgs)]
pub struct TropeLearnTutorial {

}
impl From<TropeLearnTutorial> for TropeLearnArgs {
  fn from(method_args: TropeLearnTutorial) -> Self {
    TropeLearnArgs { method: TropeLearnMethod::Tutorial(method_args) }
  }
}
