mod read_html;
mod scrape;
mod scrape_trope;
mod scrape_tropelist;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
use crate::{
  scrape_tropelist::scrape_tropelist,
  scrape_trope::scrape_trope_page,
};

fn main() {
  let args = TropeScrapeArgs::parse_args();
  match args.method {
    TropeScrapeMethod::Pagelist(method_args) => {
      scrape_tropelist(method_args).expect("Unhandled parse_tropelist error");
    },
    TropeScrapeMethod::TropePage(method_args) => {
      scrape_trope_page(method_args).expect("Unhandled scrape_trope_page error");
    },
  }
}
