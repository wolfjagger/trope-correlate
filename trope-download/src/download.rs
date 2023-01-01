use std::{fs, io::prelude::*, path};
use brotli::BrotliDecompress;
use bytes::{Bytes, Buf};
use reqwest;

use crate::header::get_header_map;


/// Download all the pages
pub fn save_page_to_path(
  url: reqwest::Url, out_dir: &path::Path, out_name: &str,
  encrypted: bool
) -> Result<(), Box<dyn std::error::Error>> {

  fs::create_dir_all(&out_dir)?;

  // Get header map for use in each page request (can panic)
  let header_map = get_header_map();

  // Set up output file
  let mut file_name = format!("{}.html", &out_name);
  if encrypted { file_name.push_str(".br"); }
  let mut file = fs::File::create(out_dir.clone().join(file_name))?;

  // Do request, get encoded body
  let encoded_body = get_body(&header_map, url)?;

  if encrypted {
    file.write_all(&encoded_body)?;
  } else {
    // Decode using brotli decompression
    BrotliDecompress(&mut encoded_body.reader(), &mut file)?;
  }

  Ok(())

}


/// Do request for a specific url
fn get_body(header_map: &reqwest::header::HeaderMap, url: reqwest::Url) -> Result<Bytes, reqwest::Error> {

  let client = reqwest::blocking::Client::new();
  let req_builder = client.request(reqwest::Method::GET, url).headers(header_map.clone());

  let req = req_builder.build()?;
  client.execute(req)?.bytes()

}
