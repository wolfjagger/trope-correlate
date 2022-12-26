use std::{path, thread, time};
use csv;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use reqwest;

use trope_lib;
use crate::download::save_page_to_path;


/// Download all the pages
pub fn save_tropelist(args: trope_lib::TropeDownloadTropelist) -> Result<(), Box<dyn std::error::Error>> {

  let out_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");

  // Inclusive
  let beg_record = 1.max(args.beg_record);
  let end_record = 1.max(args.end_record);
  if end_record < beg_record {
    panic!("end_record should not be less than beg_record");
  }

  let mut csv_records: Vec<_> = csv::Reader::from_path(args.in_path)?.into_records().collect();
  let record_iter = match args.random_seed {
    None => {
      println!("No seed, download directly");
      csv_records.into_iter()
    },
    Some(seed) => {
      println!("Seed {}, download randomly", seed);
      let mut rng = SmallRng::seed_from_u64(seed);
      csv_records.as_mut_slice().shuffle(&mut rng);
      csv_records.into_iter()
    }
  };

  println!("Downloading records {} to {}...", beg_record, end_record);

  // Page request loop with peekable iterator
  let mut tup_iter = (beg_record..end_record+1).zip(record_iter.skip((beg_record-1).into())).peekable();
  while let Some((_idx, record)) = tup_iter.next() {

    let (name, url_str) = match record {
      Ok(rec) => (rec[0].to_owned(), rec[1].to_owned()),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up url
    let url = reqwest::Url::parse(&url_str)?;
    let download_occurred = save_page_to_path(url, &out_dir, &name, !args.unencrypted, args.force)?;

    if download_occurred && tup_iter.peek().is_some() {
      // Sleep before next request
      thread::sleep(time::Duration::from_secs(args.sleep_sec));
    }

  }

  Ok(())

}
