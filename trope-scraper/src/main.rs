mod parse_tropelist;
mod read_html;

use trope_lib::{TropeScraperArgs, TropeScraperMethod};
use crate::parse_tropelist::parse_tropelist;

fn main() {
  let args = TropeScraperArgs::parse_args();
  match args.method {
    TropeScraperMethod::Pagelist(method_args) => {
      parse_tropelist(method_args).expect("Unhandled parse_tropelist error");
    },
  }
}
