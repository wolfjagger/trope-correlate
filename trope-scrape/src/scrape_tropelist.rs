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

  let csv_records = csv::Reader::from_path(args.in_path)?.into_records();

  // Page request loop
  let min_record = 1.min(args.beg_record);
  let max_record = args.end_record + 1;
  for (_idx, record) in (min_record..max_record).zip(csv_records.skip(min_record.into())) {

    let (name, _url_str) = match record {
      Ok(rec) => (rec[0].to_owned(), rec[1].to_owned()),
      Err(why) => panic!("Problem reading csv record {}", &why),
    };

    println!("Scraping {}...", name);

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

    scrape_trope(&in_html, &out_dir)?;

  }

  Ok(())

}
