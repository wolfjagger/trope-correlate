mod cmd;
mod error;
mod gen_pageid;
mod read_html;
mod scrape;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
pub use cmd::{
  gen_global_page_ids::gen_global_page_ids,
  gen_mention_page_ids::gen_mention_page_ids,
  get_namespace_tot_pages::get_namespace_tot_pages,
  get_pagelist_len::get_pagelist_len,
  scrape_namespace::scrape_namespace,
  scrape_page::scrape_page,
  scrape_pagelist::scrape_pagelist,
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
    TropeScrapeMethod::AllPages(method_args) => {
      scrape_all_pages(method_args).expect("Unhandled scrape_all_tropes error");
    },
    TropeScrapeMethod::PagelistLen(method_args) => {
      get_pagelist_len(method_args).expect("Unhandled get_pagelist_len error");
    },
    TropeScrapeMethod::GlobalPageIds(method_args) => {
      gen_global_page_ids(method_args).expect("Unhandled global_page_ids error");
    },
    TropeScrapeMethod::MentionPageIds(method_args) => {
      gen_mention_page_ids(method_args).expect("Unhandled mention_page_ids error");
    },
  }
}
