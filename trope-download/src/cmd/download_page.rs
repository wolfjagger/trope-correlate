use std::str::FromStr;
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


/// Download a trope page
pub fn save_page(args: trope_lib::TropeDownloadPage) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;
  let pagename = args.pagename;

  if trope_lib::PAGE_BLACKLIST.contains(&(ns, &pagename)) {
    log::error!("Page on the blacklist; not downloading: {}", pagename);
    return Ok(())
  }

  let out_dir = trope_lib::dl_page_dir(&ns);

  // Set up url
  let url = reqwest::Url::parse(&args.url)?;

  save_page_to_path(url, &out_dir, &pagename, !args.unencrypted, args.force)?;

  Ok(())

}
