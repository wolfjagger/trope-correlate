use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::Namespace;


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TropeDownloadArgs {
  #[command(subcommand)]
  pub method: TropeDownloadMethod
}

impl TropeDownloadArgs {
  pub fn parse_args() -> Self {
    log::info!("Parse trope-download args");
    Self::parse()
  }
}


#[derive(Debug, Subcommand)]
pub enum TropeDownloadMethod {
  Namespace(TropeDownloadNamespace),
  Page(TropeDownloadPage),
  Pagelist(TropeDownloadPagelist),
}


/// Downloads index pages in bulk from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeDownloadNamespace {

  /// Min number of pages to download (inclusive; known min: 1)
  #[clap(short, long, value_parser,)]
  pub beg_page: u8,

  /// Max number of pages to download (inclusive; known max: 58)
  #[clap(short, long, value_parser,)]
  pub end_page: u8,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, save an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing page file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

  /// Number of seconds to sleep between requests (default: 5)
  #[clap(short, long, value_parser=clap::value_parser!(u64).range(1..), default_value_t = 5)]
  pub sleep_sec: u64,

}
impl From<TropeDownloadNamespace> for TropeDownloadArgs {
  fn from(method_args: TropeDownloadNamespace) -> Self {
    TropeDownloadArgs { method: TropeDownloadMethod::Namespace(method_args) }
  }
}


/// Downloads specific pages from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeDownloadPage {

  /// Trope name
  #[clap(short, long, value_parser,)]
  pub name: String,

  /// Trope url
  #[clap(short, long, value_parser,)]
  pub url: String,

  /// Namespace of page to download
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

  /// If enabled, save an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing trope file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

}
impl From<TropeDownloadPage> for TropeDownloadArgs {
  fn from(method_args: TropeDownloadPage) -> Self {
    TropeDownloadArgs { method: TropeDownloadMethod::Page(method_args) }
  }
}


/// Downloads pages in pagelist from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct TropeDownloadPagelist {

  /// Min number of records to download (inclusive; known min: 0)
  #[clap(short, long, value_parser,)]
  pub beg_record: u64,

  /// Max number of records to download (inclusive; unknown max)
  #[clap(short, long, value_parser,)]
  pub end_record: u64,

  /// Namespace for of pagelist, to find correct directory
  #[clap(short, long, value_parser, default_value_t = Namespace::Main.to_string())]
  pub namespace: String,

 /// If enabled, save an unencrypted version of the html (default: false)
  #[clap(long, value_parser, default_value_t = false)]
  pub unencrypted: bool,

  /// Overwrite existing trope file if enabled (default: false)
  #[clap(short, long, value_parser, default_value_t = false)]
  pub force: bool,

  /// Number of seconds to sleep between requests (default: 5)
  #[clap(short, long, value_parser=clap::value_parser!(u64).range(1..), default_value_t = 5)]
  pub sleep_sec: u64,

  /// If a seed is given, download pages out-of-order (default: None)
  #[clap(short, long, value_parser, required = false)]
  pub random_seed: Option<u64>,

}
impl From<TropeDownloadPagelist> for TropeDownloadArgs {
  fn from(method_args: TropeDownloadPagelist) -> Self {
    TropeDownloadArgs { method: TropeDownloadMethod::Pagelist(method_args) }
  }
}
