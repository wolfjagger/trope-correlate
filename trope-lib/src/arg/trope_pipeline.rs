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
  NamespacePages(TropePipelineNamespacePages),
  AllPages(TropePipelineAllPages),
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

/// Download and scrape all pages in namespace
#[derive(Debug, ClapArgs)]
pub struct TropePipelineNamespacePages {

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing pagelist file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

  /// Number of seconds to sleep between requests (default: 5)
  #[clap(short, long, value_parser=clap::value_parser!(u64).range(1..), default_value_t = 5)]
  pub sleep_sec: u64,

  /// If a seed is given, download pages out-of-order (default: None)
  #[clap(short, long, value_parser, required = false)]
  pub random_seed: Option<u64>,

}
impl From<TropePipelineNamespacePages> for TropePipelineArgs {
  fn from(method_args: TropePipelineNamespacePages) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::NamespacePages(method_args) }
  }
}

/// Download and scrape pages for all namespaces
#[derive(Debug, ClapArgs)]
pub struct TropePipelineAllPages {

  /// If enabled, assume an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing page file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

  /// Number of seconds to sleep between requests (default: 5)
  #[clap(short, long, value_parser=clap::value_parser!(u64).range(1..), default_value_t = 5)]
  pub sleep_sec: u64,

  /// If a seed is given, download pages out-of-order (default: None)
  #[clap(short, long, value_parser, required = false)]
  pub random_seed: Option<u64>,

}
impl From<TropePipelineAllPages> for TropePipelineArgs {
  fn from(method_args: TropePipelineAllPages) -> Self {
    TropePipelineArgs { method: TropePipelineMethod::AllPages(method_args) }
  }
}
