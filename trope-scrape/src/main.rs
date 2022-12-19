mod read_html;
mod scrape;
mod scrape_trope;
mod scrape_pagelist;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
use crate::{
  scrape_pagelist::scrape_pagelist,
  scrape_trope::scrape_trope_page,
};

fn main() {
  let args = TropeScrapeArgs::parse_args();
  match args.method {
    TropeScrapeMethod::Tropelist(method_args) => {
      scrape_pagelist(method_args).expect("Unhandled scrape_pagelist error");
    },
    TropeScrapeMethod::TropePage(method_args) => {
      scrape_trope_page(method_args).expect("Unhandled scrape_trope_page error");
    },
  }
}
