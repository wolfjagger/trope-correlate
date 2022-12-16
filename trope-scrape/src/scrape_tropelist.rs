use std::path;
use csv;
use scraper::Selector;

use trope_lib;
use crate::read_html::read_html_file;


/// Download all the pages
pub fn scrape_tropelist(args: trope_lib::TropeScrapeTropelist) -> Result<(), Box<dyn std::error::Error>> {

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
  for page in 1..args.max_pages+1 {

    let page_str = page.to_string();

    let file_name = path_dir.clone().join(
      if args.encrypted {
        format!("page{}.html.br", &page_str)
      } else {
        format!("page{}.html", &page_str)
      }
    );

    let document = read_html_file(file_name, args.encrypted).expect("Error reading html file");

    // Create a selector for the element we want
    // For the tropes page, every link in a table cell should get us what we want
    // This can be done outside of the main loop, since it's the same each time and passed by reference
    let trope_selector = Selector::parse("td>a").unwrap();

    // Select all elements in the document
    let trope_links = document.select(&trope_selector);

    // For every trope, get the inner html (trope_name) and href (trope_url)
    let mut tropes: Vec<trope_lib::NamedLink> = Vec::new();
    for element in trope_links {
      tropes.push(trope_lib::NamedLink {
        name: element.inner_html(),
        url: element.value().attr("href").unwrap().to_string(),
      });
    }

    // In raw form, there are two non-breaking spaces, possible line breaks and possible
    // spaces in the middle. Let's get rid of those for simplicity. Todo: break this into
    // seprate function.
    for trope in tropes.iter_mut() {
      trope.name = String::from(&trope.name[12..]);
      trope.name.retain(|c| c != '\n' && c != ' ' && c != '\t');
    }

    // Write all the values to the file
    for trope in tropes.iter() {
      csv_writer.write_record(&[trope.name.clone(), trope.url.clone()])?;
    }

  }

  Ok(())

}
