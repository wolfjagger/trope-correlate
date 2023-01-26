use std::str::FromStr;
use scraper::Selector;

use trope_lib;
use crate::read_html::read_html_file;


/// Scrape namespace to find number of total pages for download
pub fn get_namespace_tot_pages(args: trope_lib::TropeScrapeNamespaceTotPages) -> Result<u32, Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let dl_ns_path = trope_lib::dl_namespace_dir(&ns);

  let page = 1;
  let page_str = page.to_string();

  log::info!("Scraping page {}...", page_str);

  let file_name = dl_ns_path.join(
    if !args.unencrypted {
      format!("page{}.html.br", &page_str)
    } else {
      format!("page{}.html", &page_str)
    }
  );

  let document = read_html_file(file_name, !args.unencrypted)?;

  // Create a selector for the element we want
  // For tot pages, we are looking for the navigation button, which has an attribute with the number
  let pagination_box_selector = Selector::parse("#main-article>div>nav.pagination-box").unwrap();

  // Select first element in the document
  let tot_pages_result = document.select(&pagination_box_selector).next().and_then(
    |el| el.value().attr("data-total-pages").and_then(|s| s.parse().ok())
  );

  let tot_pages = match tot_pages_result {
    Some(0) => {
      // Zero pages probably means bad namespace
      panic!("Zero pages returned; is the namespace valid?");
    },
    Some(num) => {
      // Non-zero pages means it's the tot pages
      num
    },
    None => {
      // Could not find pagination box with tot pages details,
      //  but that likely means it's just one page
      1
    }
  };

  log::info!("There are {} pages in the namespace", tot_pages);

  Ok(tot_pages)

}
