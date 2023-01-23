use std::{fs, str::FromStr};

use csv;
use scraper::Selector;

use trope_lib;
use crate::read_html::read_html_file;


/// Scrape pagelist to create pagelist
pub fn scrape_namespace(args: trope_lib::TropeScrapeNamespace) -> Result<(), Box<dyn std::error::Error>> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let dl_ns_path = trope_lib::dl_namespace_dir(&ns);
  let out_dir = trope_lib::sc_pagelist_dir(&ns);

  fs::create_dir_all(&out_dir)?;

  let links_path = out_dir.join("links.csv");

  let mut csv_writer = match csv::Writer::from_path(&links_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", links_path.display(), why),
  };


  // Inclusive
  let beg_page = 1.max(args.beg_page);
  let end_page = 1.max(args.end_page);
  if end_page < beg_page {
    panic!("end_page should not be less than beg_page");
  }

  // Page parse loop
  for page in beg_page..end_page+1 {

    let page_str = page.to_string();

    log::info!("Scraping page {}...", page_str);

    let file_name = dl_ns_path.join(
      if !args.unencrypted {
        format!("page{}.html.br", &page_str)
      } else {
        format!("page{}.html", &page_str)
      }
    );

    let document = read_html_file(file_name, !args.unencrypted);

    // Create a selector for the element we want
    // For the tropes page, every link in a table cell should get us what we want
    // This can be done outside of the main loop, since it's the same each time and passed by reference
    let link_selector = Selector::parse("#main-article li>a").unwrap();

    // Select all elements in the document
    let links = document.select(&link_selector).map(|el| {
      // In raw form, there are two non-breaking spaces, possible line breaks and possible
      // spaces in the middle. Let's get rid of those.
      trope_lib::NamedLink::new(
        el.inner_html().replace("&nbsp;", "").split_whitespace().collect::<String>(),
        el.value().attr("href").unwrap().to_string(),
      )
    }).collect::<Vec<_>>();

    // Write all the values to the file
    for link in links {
      csv_writer.serialize(link)?;
    }

  }

  Ok(())

}
