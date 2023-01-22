mod cmd;
mod read_html;
mod scrape;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
pub use cmd::{
  scrape_namespace::scrape_namespace,
  get_namespace_tot_pages::get_namespace_tot_pages,
  scrape_page::scrape_page,
  scrape_pagelist::scrape_pagelist,
  get_pagelist_len::get_pagelist_len,
  scrape_all_pages::scrape_all_pages,
};

pub fn run(args: TropeScrapeArgs) {
  match args.method {
    TropeScrapeMethod::Namespace(method_args) => {
      scrape_namespace(method_args).expect("Unhandled scrape_namespace error");
    },
    TropeScrapeMethod::NamespaceTotPages(method_args) => {
      get_namespace_tot_pages(method_args).expect("Unhandled get_namespace_tot_pages error");
    }
    TropeScrapeMethod::Page(method_args) => {
      scrape_page(method_args).expect("Unhandled scrape_page error");
    },
    TropeScrapeMethod::Pagelist(method_args) => {
      scrape_pagelist(method_args).expect("Unhandled scrape_pagelist error");
    },
    TropeScrapeMethod::PagelistLen(method_args) => {
      get_pagelist_len(method_args).expect("Unhandled get_pagelist_len error");
    },
    TropeScrapeMethod::AllPages(method_args) => {
      scrape_all_pages(method_args).expect("Unhandled scrape_all_tropes error");
    },
  }
}
