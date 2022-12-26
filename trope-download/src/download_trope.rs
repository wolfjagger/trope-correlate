use std::path;
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


/// Download a trope page
pub fn save_trope_page(args: trope_lib::TropeDownloadTropePage) -> Result<(), Box<dyn std::error::Error>> {

  let out_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");

  // Set up url
  let url = reqwest::Url::parse(&args.url)?;

  save_page_to_path(url, &out_dir, &args.name, args.encrypted, args.force)?;

  Ok(())

}
