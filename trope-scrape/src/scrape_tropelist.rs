use std::path;

use trope_lib;
use crate::{
  read_html::read_html_file,
  scrape::scrape_trope
};


/// Download all the pages
pub fn scrape_tropelist(args: trope_lib::TropeScrapeTropelist) -> Result<(), Box<dyn std::error::Error>> {

  let tropes_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("tropes");

  // Inclusive
  let beg_record = 0.max(args.beg_record);
  let end_record = 0.max(args.end_record);
  if end_record < beg_record {
    panic!("end_record should not be less than beg_record");
  }

  let mut reader = csv::Reader::from_path(&args.in_path)?;
  let csv_records = reader.deserialize::<trope_lib::NamedLink>();

  // Page request loop
  for (_idx, record) in (beg_record..end_record+1).zip(csv_records.skip((beg_record).into())) {

    let (name, _url_str) = match record {
      Ok(rec) => (rec.name, rec.url),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    // Set up input html
    let in_dir = path::PathBuf::from("..")
      .join(trope_lib::DATA_DIR)
      .join("trope_page");
    let in_path = in_dir.join(
      if args.encrypted {
        format!("{}.html.br", &name)
      } else {
        format!("{}.html", &name)
      }
    );
    let in_html = read_html_file(in_path, args.encrypted).expect("Error reading html file");

    // Save output to a subdir of the tropes dir
    let out_dir = tropes_dir.clone().join(&name);

    scrape_trope(&name, &in_html, &out_dir, args.force)?;

  }

  Ok(())

}
