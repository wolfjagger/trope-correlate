use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::Namespace;


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropePipelineArgs {
  #[command(subcommand)]
  pub method: TropePipelineMethod
}

impl TropePipelineArgs {
  pub fn parse_args() -> Self {
    Self::parse()
  }
}

#[derive(Debug, Subcommand)]
pub enum TropePipelineMethod {
  NamespaceTropelist(TropePipelineNamespaceTropelist),
  AllTropelists(TropePipelineAllTropelists),
}


/// Download and scrape tropelist for namespace
#[derive(Debug, ClapArgs)]
pub struct TropePipelineNamespaceTropelist {

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing tropelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropePipelineNamespaceTropelist> for TropePipelineArgs {
  fn from(method_args: TropePipelineNamespaceTropelist) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::NamespaceTropelist(method_args) }
  }
}

/// Download and scrape tropelists for all namespaces
#[derive(Debug, ClapArgs)]
pub struct TropePipelineAllTropelists {

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing tropelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropePipelineAllTropelists> for TropePipelineArgs {
  fn from(method_args: TropePipelineAllTropelists) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::AllTropelists(method_args) }
  }
}
