use std::{fs, io, io::BufRead, str::FromStr};

use trope_lib;


/// Scrape namespace to find number of total pages for download
pub fn get_pagelist_len(args: trope_lib::TropeScrapePagelistLen) -> Result<u32, Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let sc_pl_path = trope_lib::sc_pagelist_dir(&ns).join("links.csv");

  let fi = fs::File::open(&sc_pl_path).expect(&format!("Cannot open {}", &sc_pl_path.display()));

  let len = io::BufReader::new(fi).lines().count() - 1;

  log::info!("There are {} pages in the pagelist", len);

  Ok(len as u32)

}
