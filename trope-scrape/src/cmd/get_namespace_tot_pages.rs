use scraper::Selector;

use trope_lib;
use crate::read_html::read_html_file;


/// Scrape pagelist to create tropelist
pub fn get_namespace_tot_pages(args: trope_lib::TropeScrapeNamespaceTotPages) -> Result<u32, Box<dyn std::error::Error>> {

  let pagelist_path = trope_lib::download_dir().join("namespace")
    .join(&args.namespace);

  let page = 1;
  let page_str = page.to_string();

  println!("Scraping page {}...", page_str);

  let file_name = pagelist_path.join(
    if !args.unencrypted {
      format!("page{}.html.br", &page_str)
    } else {
      format!("page{}.html", &page_str)
    }
  );

  let document = read_html_file(file_name, !args.unencrypted);

  // Create a selector for the element we want
  // For tot pages, we are looking for the navigation button, which has an attribute with the number
  let pagination_box_selector = Selector::parse("#main-article>div>nav.pagination-box").unwrap();

  // Select first element in the document
  let tot_pages: u32 = document.select(&pagination_box_selector).next().and_then(
    |el| el.value().attr("data-total-pages").and_then(|s| s.parse().ok())
  ).expect("Could not find pagination box with tot pages details; see url for sanity check");

  println!("There are {} pages in the namespace", tot_pages);

  Ok(tot_pages)

}
