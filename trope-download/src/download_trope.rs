use std::{fs, io::prelude::*, path};
use brotli::BrotliDecompress;
use bytes::{Bytes, Buf};
use reqwest;

use trope_lib;
use crate::header::get_header_map;


/// Download all the pages
pub fn save_trope_page(args: trope_lib::TropeDownloadTropePage) -> Result<(), Box<dyn std::error::Error>> {

  // Set up output directory in the parent trope-correlate dir
  let path_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");
  fs::create_dir_all(&path_dir)?;

  // Get header map for use in each page request (can panic)
  let header_map = get_header_map();

  let trope_name = args.name;

  // Set up output file
  let mut file_name = format!("{}.html", &trope_name);
  if args.encrypted { file_name.push_str(".br"); }
  let mut file = fs::File::create(path_dir.clone().join(file_name))?;

  // Set up url
  let url = reqwest::Url::parse(&args.url)?;

  // Do request, get encoded body
  let encoded_body = get_body(&header_map, url)?;

  if args.encrypted {
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
