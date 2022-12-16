use std::path;

use trope_lib;
use crate::scrape::scrape_trope_to_medialist;


/// Download all the pages
pub fn scrape_trope_page(args: trope_lib::TropeScrapeTropePage) -> Result<(), Box<dyn std::error::Error>> {

  // Set up input directory in the parent trope-correlate dir
  let in_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");

  // Set up output file in same parent dir
  let out_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("tropes");

  scrape_trope_to_medialist(&in_dir, &out_dir, &args.name, args.encrypted)?;

  Ok(())

}
