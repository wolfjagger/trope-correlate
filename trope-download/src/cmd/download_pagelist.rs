use std::{str::FromStr, thread, time};
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


const NAMESPACE_PREFIX: &str = "n";
const PAGETYPE_PREFIX: &str = "t";
const PAGENUM_PREFIX: &str = "page";
const PAGELIST_SEARCH_PAGE: &str =
  "https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php";


/// Download all the pages
pub fn save_pagelist(args: trope_lib::TropeDownloadPagelist) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  // Set up output directory in the parent trope-correlate dir
  let out_dir = trope_lib::download_dir().join("pagelist")
    .join(ns.to_string()).join(&args.pagetype.to_lowercase());

  // Inclusive
  let beg_page = 1.max(args.beg_page);
  let end_page = 1.max(args.end_page);
  if end_page < beg_page {
    panic!("end_page should not be less than beg_page");
  }

  // Page request loop with peekable iterator
  let mut page_iter = (beg_page..end_page+1).peekable();
  while let Some(page) = page_iter.next() {

    let page_str = page.to_string();

    println!("Downloading page {}...", page_str);

    // Set up output file
    let file_name = format!("page{}", &page_str);

    // Set up url
    let url = create_url(&args.namespace, &args.pagetype, &page_str)?;

    let download_occurred = save_page_to_path(url, &out_dir, &file_name, !args.unencrypted, args.force)?;

    if download_occurred && page_iter.peek().is_some() {
      // Sleep before next request
      thread::sleep(time::Duration::from_secs(args.sleep_sec));
    }

  }

  Ok(())

}


/// Define the url string from the query arguments.
fn create_url(namespace: &str, pagetype: &str, page: &str) -> Result<reqwest::Url, url::ParseError> {
  reqwest::Url::parse_with_params(
    PAGELIST_SEARCH_PAGE,
    &[(NAMESPACE_PREFIX, namespace), (PAGETYPE_PREFIX, pagetype), (PAGENUM_PREFIX, page)]
  )
}
