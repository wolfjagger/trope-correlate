use std::{fs, io::prelude::*, path};
use brotli::BrotliDecompress;
use bytes::{Bytes, Buf};
use reqwest;

use crate::header::get_header_map;


/// Download all the pages
///  Returns true if a page was downloaded and false otherwise
pub fn save_page_to_path(
  url: reqwest::Url, out_dir: &path::Path, out_name: &str,
  encrypted: bool, force: bool
) -> Result<bool, Box<dyn std::error::Error>> {

  fs::create_dir_all(&out_dir)?;

  let mut file_name = format!("{}.html", &out_name);
  if encrypted { file_name.push_str(".br"); }
  let out_path = out_dir.clone().join(file_name);

  if out_path.exists() {
    if force {
      println!("File exists, downloading and overwriting {}...", out_name);
      fs::remove_file(&out_path)?;
    } else {
      println!("File exists, skipping {}...", out_name);
      return Ok(false);
    }
  } else {
    println!("Downloading {}...", out_name);
  }

  // Get header map for use in each page request (can panic)
  let header_map = get_header_map();

  // Set up output file
  let mut file = fs::File::create(out_path)?;

  // Do request, get encoded body
  let encoded_body = get_body(&header_map, url)?;

  if encrypted {
    file.write_all(&encoded_body)?;
  } else {
    // Decode using brotli decompression
    BrotliDecompress(&mut encoded_body.reader(), &mut file)?;
  }

  Ok(true)

}


/// Do request for a specific url
fn get_body(header_map: &reqwest::header::HeaderMap, url: reqwest::Url) -> Result<Bytes, reqwest::Error> {

  let client = reqwest::blocking::Client::new();
  let req_builder = client.request(reqwest::Method::GET, url).headers(header_map.clone());

  let req = req_builder.build()?;
  client.execute(req)?.bytes()

}
