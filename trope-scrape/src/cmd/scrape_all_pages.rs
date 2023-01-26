use std::{fs, str::FromStr};

use trope_lib;
use crate::scrape;


/// Download all the pages
pub fn scrape_all_pages(args: trope_lib::TropeScrapeAllPages) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let in_dir = trope_lib::dl_page_dir(&ns);
  let tropes_dir = trope_lib::sc_page_dir(&ns);

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
  let tot_pages = trope_names.len();

  log::info!("Scraping {} records from download directory...", trope_names.len());

  let mut progress_tick = 0;
  for (idx, name) in trope_names.iter().enumerate() {

    if args.progress && (100 * idx > progress_tick * tot_pages) {
      log::warn!("Progress: {}%", progress_tick);
      progress_tick += 1;
    }

    // Set up input html
    let in_path = in_dir.join(format!("{}{}", &name, &ext));

    // Save output to a subdir of the tropes dir
    let out_dir = tropes_dir.clone().join(&name);

    scrape::scrape_page(&name, in_path, &out_dir, !args.unencrypted, args.force)?;

  }

  Ok(())

}
