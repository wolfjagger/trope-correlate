use std::str::FromStr;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use trope_lib;
use crate::scrape;


/// Download all the pages
pub fn scrape_pagelist(args: trope_lib::TropeScrapePagelist) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let pagelist_path = trope_lib::sc_pagelist_dir(&ns).join("links.csv");
  let page_dir = trope_lib::dl_page_dir(&ns);
  let scraped_page_dir = trope_lib::sc_page_dir(&ns);

  // Inclusive
  let beg_record = 0.max(args.beg_record) as usize;
  let end_record = 0.max(args.end_record) as usize;
  if end_record < beg_record {
    panic!("end_record should not be less than beg_record");
  }

  let mut reader = csv::Reader::from_path(pagelist_path)?;
  let mut csv_records: Vec<_> = reader.deserialize::<trope_lib::NamedLink>().collect();
  let tot_records = csv_records.len();
  let record_iter = match args.random_seed {
    None => {
      log::warn!("No seed, scrape in order");
      csv_records.into_iter()
    },
    Some(seed) => {
      log::warn!("Seed {}, scrape randomly", seed);
      let mut rng = SmallRng::seed_from_u64(seed);
      csv_records.as_mut_slice().shuffle(&mut rng);
      csv_records.into_iter()
    }
  };

  log::info!("Scraping {} to {} of {} records...", beg_record, end_record, tot_records);

  // Page request loop
  let mut tup_iter = (beg_record..end_record+1).zip(record_iter.skip(beg_record));
  let mut progress_tick = 0;
  while let Some((idx, record)) = tup_iter.next() {

    if args.progress && (100 * idx > progress_tick * tot_records) {
      log::warn!("Progress: {}%", progress_tick);
      progress_tick += 1;
    }

    let (name, _url_str) = match record {
      Ok(rec) => (rec.name, rec.url),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up input html
    let page_path = page_dir.join(
      if !args.unencrypted {
        format!("{}.html.br", &name)
      } else {
        format!("{}.html", &name)
      }
    );

    // Save output to a subdir of the pages dir
    let out_dir = scraped_page_dir.join(&name);

    scrape::scrape_page(&name, page_path, &out_dir, !args.unencrypted, args.force)?;

  }

  Ok(())

}
