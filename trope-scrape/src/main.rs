mod scrape_tropelist;
mod read_html;

use trope_lib::{TropeScraperArgs, TropeScraperMethod};
use crate::scrape_tropelist::scrape_tropelist;

fn main() {
  let args = TropeScraperArgs::parse_args();
  match args.method {
    TropeScraperMethod::Pagelist(method_args) => {
      scrape_tropelist(method_args).expect("Unhandled parse_tropelist error");
    },
  }
}
