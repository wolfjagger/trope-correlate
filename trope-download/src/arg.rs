use clap::{Args as ClapArgs, Parser, Subcommand};

use trope_lib;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[command(subcommand)]
  pub method: DownloadMethodArgs
}


#[derive(Debug, Subcommand)]
pub enum DownloadMethodArgs {
  Pagelist(PagelistArgs),
}


/// Downloads index pages in bulk from tvtropes.
#[derive(Debug, ClapArgs)]
pub struct PagelistArgs {

  /// If enabled, save an encrypted version of the html
  #[clap(short, long, value_parser, default_value_t = false)]
  pub encrypted: bool,

  /// Namespace for page search
  #[clap(short, long, value_parser, default_value_t = trope_lib::Namespace::Main.to_string())]
  pub namespace: String,

  /// Pagetype for page search
  #[clap(short, long, value_parser, default_value_t = trope_lib::Pagetype::Trope.to_string())]
  pub pagetype: String,

  /// Max number of pages to call for
  #[clap(short, long, value_parser, default_value_t = 2)]
  pub max_pages: u8,

}
