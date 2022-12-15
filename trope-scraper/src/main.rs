mod arg;
mod parse_tropelist;
mod read_html;

use crate::{arg::Args, parse_tropelist::parse_tropelist};

fn main() {
  let args = Args::parse_args();
  // TODO: Translate Args to more specific TropelistArgs based on "method" of scrape
  parse_tropelist(args).expect("Unhandled parse_tropelist error");
}
