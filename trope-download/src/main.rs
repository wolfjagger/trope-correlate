mod arg;

// Download TvTropes pages

use std::{fs, path, thread, time};
use brotli::BrotliDecompress;
use bytes::{Bytes, Buf};
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

  let args = Args::parse_args();

  // Set up output directory
  let path_dir = path::PathBuf::from(&format!("test_data{}_{}", args.namespace, args.pagetype));
  fs::create_dir_all(&path_dir)?;

  // Get header map for use in each page request (can panic)
  let header_map = get_header_map();

  // Page request loop
  for page in 1..args.max_pages+1 {

    let page_str = page.to_string();

    // Set up output file
    let mut filename = path_dir.clone();
    filename.push(format!("page{}.html", &page_str));
    let mut file = fs::File::create(filename)?;

    // Set up url
    let url = create_url(&args.namespace, &args.pagetype, &page_str)?;

    // Do request, get encoded body
    let encoded_body = get_body(&header_map, url)?;

    // Decode using brotli decompression
    BrotliDecompress(&mut encoded_body.reader(), &mut file)?;

    // Sleep before next request
    thread::sleep(time::Duration::from_secs(1));

  }

  Ok(())

}


/// Get header from local file and parse for headers (can panic!)
fn get_header_map() -> reqwest::header::HeaderMap {

  let header_str = fs::read_to_string(".header").expect("You need to set the .header file; see readme");
  if header_str.is_empty() {
    panic!("Your .header file is empty; see readme");
  }

  let mut map = reqwest::header::HeaderMap::new();

  for header_line in header_str.lines() {
    let (key, val) = header_line.split_once(": ").expect("A header line is improperly formatted");
    let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes()).expect("Incorrect header name in header file");
    let header_value = val.parse().expect("Header value could not be parsed");
    map.append(header_name, header_value);
  }

  // Force brotli compression so we can decompress uniformly
  map.insert("Accept-Encoding", "br".parse().expect("Encoding header value could not be parsed"));

  map

}


/// Define the url string from the query arguments.
fn create_url(namespace: &str, pagetype: &str, page: &str) -> Result<reqwest::Url, url::ParseError> {
  reqwest::Url::parse_with_params(
    TVTROPES_SEARCH_PAGE,
    &[(NAMESPACE_PREFIX, namespace), (PAGETYPE_PREFIX, pagetype), (PAGENUM_PREFIX, page)]
  )
}


/// Do request for a specific url
fn get_body(header_map: &reqwest::header::HeaderMap, url: reqwest::Url) -> Result<Bytes, reqwest::Error> {

  let client = reqwest::blocking::Client::new();
  let req_builder = client.request(reqwest::Method::GET, url).headers(header_map.clone());

  let req = req_builder.build()?;
  client.execute(req)?.bytes()

}
