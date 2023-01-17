mod cmd;
mod read_html;
mod scrape;

use trope_lib::{TropeScrapeArgs, TropeScrapeMethod};
pub use cmd::{
  scrape_namespace::scrape_namespace,
  get_namespace_tot_pages::get_namespace_tot_pages,
  scrape_trope::scrape_trope_page,
  scrape_tropelist::scrape_tropelist,
  scrape_all_tropes::scrape_all_tropes
};

pub fn run(args: TropeScrapeArgs) {
  match args.method {
    TropeScrapeMethod::Namespace(method_args) => {
      scrape_namespace(method_args).expect("Unhandled scrape_namespace error");
    },
    TropeScrapeMethod::NamespaceTotPages(method_args) => {
      get_namespace_tot_pages(method_args).expect("Unhandled get_namespace_tot_pages error");
    }
    TropeScrapeMethod::TropePage(method_args) => {
      scrape_trope_page(method_args).expect("Unhandled scrape_trope_page error");
    },
    TropeScrapeMethod::Tropelist(method_args) => {
      scrape_tropelist(method_args).expect("Unhandled scrape_tropelist error");
    },
    TropeScrapeMethod::AllTropes(method_args) => {
      scrape_all_tropes(method_args).expect("Unhandled scrape_all_tropes error");
    },
  }
}
