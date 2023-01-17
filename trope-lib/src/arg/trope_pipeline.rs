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
  NamespacePagelist(TropePipelineNamespacePagelist),
  AllPagelists(TropePipelineAllPagelists),
}


/// Download and scrape namespace for pagelist
#[derive(Debug, ClapArgs)]
pub struct TropePipelineNamespacePagelist {

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing pagelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropePipelineNamespacePagelist> for TropePipelineArgs {
  fn from(method_args: TropePipelineNamespacePagelist) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::NamespacePagelist(method_args) }
  }
}

/// Download and scrape pagelists for all namespaces
#[derive(Debug, ClapArgs)]
pub struct TropePipelineAllPagelists {

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing pagelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropePipelineAllPagelists> for TropePipelineArgs {
  fn from(method_args: TropePipelineAllPagelists) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::AllPagelists(method_args) }
  }
}
