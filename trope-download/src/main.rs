mod arg;

// Download TvTropes pages

use std::{fs, path, thread, time};
use reqwest;

use arg::Args;


const NAMESPACE_PREFIX: &str = "n";
const PAGETYPE_PREFIX: &str = "t";
const PAGENUM_PREFIX: &str = "page";
const TVTROPES_SEARCH_PAGE: &str =
  "https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php";


fn main() {
  download_all_pages().unwrap();
}


/// Download all the pages
fn download_all_pages() -> Result<(), Box<dyn std::error::Error>> {

  // clap tutorial
  let args = Args::parse_args();

  let path_dir = path::PathBuf::from(&format!("test_data{}_{}", args.namespace, args.pagetype));
  fs::create_dir_all(&path_dir)?;

  for page in 1..args.max_pages+1 {

    let page_str = page.to_string();
    let mut filename = path_dir.clone();
    filename.push(format!("page{}.html", &page_str));

    let url = create_url(&args.namespace, &args.pagetype, &page_str)?;

    let body = get_body(url)?;

    fs::write(filename, body)?;
    thread::sleep(time::Duration::from_secs(1));

  }

  Ok(())

}


/// Define the url string from the query arguments.
fn create_url(namespace: &str, pagetype: &str, page: &str) -> Result<reqwest::Url, url::ParseError> {
  reqwest::Url::parse_with_params(
    TVTROPES_SEARCH_PAGE,
    &[(NAMESPACE_PREFIX, namespace), (PAGETYPE_PREFIX, pagetype), (PAGENUM_PREFIX, page)]
  )
}


fn get_body(url: reqwest::Url) -> Result<String, reqwest::Error> {
  reqwest::blocking::get(url)?.text()
}
