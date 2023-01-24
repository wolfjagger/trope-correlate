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

  fs::create_dir_all(&out_dir).expect(
    &format!("Could not create directory {}", out_dir.display())
  );

  if out_name.contains("/") {
    log::error!("Filename {} contains bad character; not saving to path", out_name);
    return Ok(false);
  }

  let mut file_name = format!("{}.html", &out_name);
  if encrypted { file_name.push_str(".br"); }
  let out_path = out_dir.clone().join(file_name);

  if out_path.exists() {
    if force {
      log::info!("File exists, downloading and overwriting {}...", out_name);
      fs::remove_file(&out_path)?;
    } else {
      log::info!("File exists, skipping {}...", out_name);
      return Ok(false);
    }
  } else {
    log::info!("Downloading {}...", out_name);
  }

  // Get header map for use in each page request (can panic)
  let header_map = get_header_map();

  // Set up output file
  let mut file = fs::File::create(&out_path)
    .expect(&format!("Error writing file to {}", &out_path.display()));

  // Do request, get encoded body
  let encoded_body = match get_body(&header_map, url) {
    Ok(b) => b,
    Err(why) => {
      log::error!("Problem encountered when requesting encoded body: {}", why);
      return Ok(true);
    }
  };

  if encrypted {
    file.write_all(&encoded_body)
      .expect(&format!("Error writing encrypted content for {}", &out_path.display()));
  } else {
    // Decode using brotli decompression
    BrotliDecompress(&mut encoded_body.reader(), &mut file)
      .expect(&format!("Error during brotli decompression for {}", &out_path.display()));
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
