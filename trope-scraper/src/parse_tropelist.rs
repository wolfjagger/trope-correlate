use std::{fmt, fs, io::prelude::*, path};
use scraper::Selector;

use trope_lib;
use crate::{arg::Args, read_html::read_html_file};


#[derive(Debug)]
struct Trope {
  name: String,
  url: String,
}

impl fmt::Display for Trope {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{},{}]", self.name, self.url)
  }
}


/// Download all the pages
pub fn parse_tropelist(args: Args) -> Result<(), Box<dyn std::error::Error>> {

  // Set up input directory in the parent trope-correlate dir
  let path_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join(&args.namespace)
    .join(&args.pagetype);

  // Set up output file in same parent dir
  let output_path = "../test_data/tropes.csv";
  let mut output = match fs::File::create(&output_path) {
    Ok(output) => output,
    Err(why) => panic!("Couldn't write to {}: {}", output_path, why),
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

    // For every trope, get the inner html (trope_name) and
    let mut tropes: Vec<Trope> = Vec::new();
    for element in trope_links {
      tropes.push(Trope {
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
      let s = format!("{}, {}\n", trope.name, trope.url);
      output
        .write_all(s.as_bytes())
        .expect("Unable to write string");
    }

  }

  Ok(())

}
