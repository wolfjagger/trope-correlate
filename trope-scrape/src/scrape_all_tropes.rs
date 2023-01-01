use std::{fs, path};

use trope_lib;
use crate::scrape::scrape_trope;


/// Download all the pages
pub fn scrape_all_tropes(args: trope_lib::TropeScrapeAllTropes) -> Result<(), Box<dyn std::error::Error>> {

  let in_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");
  let tropes_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("tropes");

  let in_files = fs::read_dir(&in_dir)?;
  let ext = if args.unencrypted { ".html" } else { ".html.br" };
  let mut trope_names = in_files.into_iter().filter_map(|in_file| {
    let some_file = in_file.ok();
    let fname = some_file.and_then(|in_fi| in_fi.path().file_name().map(|s| s.to_os_string()));
    let fname_str = fname.and_then(|fname| fname.into_string().ok());
    let name = fname_str.and_then(|fname_str| fname_str.strip_suffix(&ext).map(|s| s.to_string()));
    name
  }).collect::<Vec<_>>();
  trope_names.sort();

  println!("Scraping {} records from download directory...", trope_names.len());

  for name in trope_names {

    // Set up input html
    let in_path = in_dir.join(format!("{}{}", &name, &ext));

    // Save output to a subdir of the tropes dir
    let out_dir = tropes_dir.clone().join(&name);

    scrape_trope(&name, in_path, &out_dir, !args.unencrypted, args.force)?;

  }

  Ok(())

}
