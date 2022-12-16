mod scrape_tropelist;
mod read_html;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
use crate::scrape_tropelist::scrape_tropelist;

fn main() {
  let args = TropeScrapeArgs::parse_args();
  match args.method {
    TropeScrapeMethod::Pagelist(method_args) => {
      scrape_tropelist(method_args).expect("Unhandled parse_tropelist error");
    },
  }
}
