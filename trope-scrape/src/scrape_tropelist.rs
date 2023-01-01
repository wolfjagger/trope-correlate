use std::path;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use trope_lib;
use crate::scrape::scrape_trope;


/// Download all the pages
pub fn scrape_tropelist(args: trope_lib::TropeScrapeTropelist) -> Result<(), Box<dyn std::error::Error>> {

  let data_dir = path::PathBuf::from("..").join(trope_lib::DATA_DIR);
  let download_dir = data_dir.join("download");
  let scrape_dir = data_dir.join("scrape");
  let tropelist_path = scrape_dir.join("tropelist").join("tropes.csv");
  let trope_page_dir = download_dir.join("trope");
  let scraped_trope_dir = scrape_dir.join("trope");

  // Inclusive
  let beg_record = 0.max(args.beg_record);
  let end_record = 0.max(args.end_record);
  if end_record < beg_record {
    panic!("end_record should not be less than beg_record");
  }

  let mut reader = csv::Reader::from_path(tropelist_path)?;
  let mut csv_records: Vec<_> = reader.deserialize::<trope_lib::NamedLink>().collect();
  let tot_records = csv_records.len();
  let record_iter = match args.random_seed {
    None => {
      println!("No seed, scrape in order");
      csv_records.into_iter()
    },
    Some(seed) => {
      println!("Seed {}, scrape randomly", seed);
      let mut rng = SmallRng::seed_from_u64(seed);
      csv_records.as_mut_slice().shuffle(&mut rng);
      csv_records.into_iter()
    }
  };

  println!("Scraping {} to {} of {} records...", beg_record, end_record, tot_records);

  // Page request loop
  let mut tup_iter = (beg_record..end_record+1).zip(record_iter.skip(beg_record as usize));
  while let Some((_idx, record)) = tup_iter.next() {

    let (name, _url_str) = match record {
      Ok(rec) => (rec.name, rec.url),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up input html
    let trope_page_path = trope_page_dir.join(
      if !args.unencrypted {
        format!("{}.html.br", &name)
      } else {
        format!("{}.html", &name)
      }
    );

    // Save output to a subdir of the tropes dir
    let out_dir = scraped_trope_dir.join(&name);

    scrape_trope(&name, trope_page_path, &out_dir, !args.unencrypted, args.force)?;

  }

  Ok(())

}
