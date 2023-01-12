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
  Tutorial(TropeLearnTutorial),
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
