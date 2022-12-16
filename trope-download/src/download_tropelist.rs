use std::{path, thread, time};
use csv;
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


/// Download all the pages
pub fn save_tropelist(args: trope_lib::TropeDownloadTropelist) -> Result<(), Box<dyn std::error::Error>> {

  let out_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");

  let csv_records = csv::Reader::from_path(args.in_path)?.into_records();

  // Page request loop
  for (_idx, record) in (1..args.max_pages+1).zip(csv_records) {

    let (name, url_str) = match record {
      Ok(rec) => (rec[0].to_owned(), rec[1].to_owned()),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up url
    let url = reqwest::Url::parse(&url_str)?;

    save_page_to_path(url, &out_dir, &name, args.encrypted)?;

    // Sleep before next request
    thread::sleep(time::Duration::from_secs(1));

  }

  Ok(())

}
