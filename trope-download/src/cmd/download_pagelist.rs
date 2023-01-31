use std::{str::FromStr, thread, time};
use csv;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


/// Download all the pages
pub fn save_pagelist(args: trope_lib::TropeDownloadPagelist) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let pagelist_path = trope_lib::sc_pagelist_dir(&ns).join("links.csv");
  let page_dir = trope_lib::dl_page_dir(&ns);

  // Inclusive
  let beg_record = 0.max(args.beg_record) as usize;
  let end_record = 0.max(args.end_record) as usize;
  if end_record < beg_record {
    panic!("end_record should not be less than beg_record");
  }

  let mut csv_records: Vec<_> = csv::Reader::from_path(pagelist_path)?.into_records().collect();
  let tot_records = csv_records.len();
  let record_iter = match args.random_seed {
    None => {
      log::warn!("No seed, download directly");
      csv_records.into_iter()
    },
    Some(seed) => {
      log::warn!("Seed {}, download randomly", seed);
      let mut rng = SmallRng::seed_from_u64(seed);
      csv_records.as_mut_slice().shuffle(&mut rng);
      csv_records.into_iter()
    }
  };

  log::info!("Downloading {} to {} of {} records...", beg_record, end_record, tot_records);

  if args.progress {
    let expected_duration = chrono::Duration::seconds(
      (tot_records as u64 * args.sleep_sec) as i64
    );
    let expected_duration_str = if expected_duration.num_days() > 0 {
      let d = expected_duration.num_days();
      let h = expected_duration.num_hours() - 24 * expected_duration.num_days();
      format!("{} d, {} h", d, h)
    } else if expected_duration.num_hours() > 0 {
      let h = expected_duration.num_hours();
      let m = expected_duration.num_minutes() - 60 * expected_duration.num_hours();
      format!("{} h, {} m", h, m)
    } else if expected_duration.num_minutes() > 0 {
      let m = expected_duration.num_minutes();
      let s = expected_duration.num_seconds() - 60 * expected_duration.num_minutes();
      format!("{} m, {} s", m, s)
    } else if expected_duration.num_seconds() > 0 {
      format!("{} s", expected_duration.num_seconds())
    } else {
      format!("< 1s")
    };
    log::warn!("Should take about {}", expected_duration_str);
  }

  // Page request loop with peekable iterator
  let mut tup_iter = (beg_record..end_record+1).zip(record_iter.skip(beg_record)).peekable();
  let mut progress_tick = 0;
  while let Some((idx, record)) = tup_iter.next() {

    if args.progress && (100 * idx > progress_tick * tot_records) {
      log::warn!("Progress: {}%", progress_tick);
      progress_tick += 1;
    }

    let (name, url_str) = match record {
      Ok(rec) => (rec[0].to_owned(), rec[1].to_owned()),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up url
    let url = reqwest::Url::parse(&url_str)?;
    let download_occurred = save_page_to_path(url, &page_dir, &name, !args.unencrypted, args.force)?;

    if download_occurred && tup_iter.peek().is_some() {
      // Sleep before next request
      thread::sleep(time::Duration::from_secs(args.sleep_sec));
    }

  }

  Ok(())

}
