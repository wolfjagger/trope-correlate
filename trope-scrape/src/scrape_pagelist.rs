use std::path;
use csv;
use scraper::Selector;

use trope_lib;
use crate::read_html::read_html_file;


/// Scrape pagelist to create tropelist
pub fn scrape_pagelist(args: trope_lib::TropeScrapePagelist) -> Result<(), Box<dyn std::error::Error>> {

  // Set up input directory in the parent trope-correlate dir
  let path_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join(&args.namespace)
    .join(&args.pagetype);

  // Set up output file in same parent dir
  let output_path = path::PathBuf::from("..")
    .join("test_data")
    .join("tropes.csv");
  let mut csv_writer = match csv::Writer::from_path(&output_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", output_path.display(), why),
  };


  // Page parse loop
  let min_page = 1.min(args.beg_page);
  let max_page = args.end_page + 1;
  for page in min_page..max_page {

    let page_str = page.to_string();

    println!("Scraping page {}...", page_str);

    let file_name = path_dir.clone().join(
      if !args.unencrypted {
        format!("page{}.html.br", &page_str)
      } else {
        format!("page{}.html", &page_str)
      }
    );

    let document = read_html_file(file_name, !args.unencrypted).expect("Error reading html file");

    // Create a selector for the element we want
    // For the tropes page, every link in a table cell should get us what we want
    // This can be done outside of the main loop, since it's the same each time and passed by reference
    let trope_selector = Selector::parse("td>a").unwrap();

    // Select all elements in the document
    let tropes = document.select(&trope_selector).map(|el| {
      // In raw form, there are two non-breaking spaces, possible line breaks and possible
      // spaces in the middle. Let's get rid of those.
      trope_lib::NamedLink{
        name: el.inner_html().replace("&nbsp;", "").split_whitespace().collect::<String>(),
        url: el.value().attr("href").unwrap().to_string(),
      }
    }).collect::<Vec<_>>();

    // Write all the values to the file
    for trope in tropes {
      csv_writer.serialize(trope)?;
    }

  }

  Ok(())

}
