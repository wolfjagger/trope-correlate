use std::{fs, path};
use scraper::Selector;

use crate::read_html::read_html_file;


/// Download all the pages
pub fn scrape_trope_to_medialist(
  in_dir: &path::Path, out_dir: &path::Path, name: &str,
  encrypted: bool
) -> Result<(), Box<dyn std::error::Error>> {

  fs::create_dir_all(&out_dir)?;

  let in_file_name = in_dir.clone().join(
    if encrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );

  // Set up output file in same parent dir
  let output_path = out_dir.clone().join(format!("{}.csv", name));
  let mut csv_writer = match csv::Writer::from_path(&output_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", output_path.display(), why),
  };

  let document = read_html_file(in_file_name, encrypted).expect("Error reading html file");

  // TODO: Need to be more selective here
  let media_selector = Selector::parse("div#main-article a").unwrap();
  let media_links = document.select(&media_selector);

  // For every media mentioned, get the inner html (media_name) and href (media_url)
  let mut medias: Vec<trope_lib::Media> = Vec::new();
  for element in media_links {
    medias.push(trope_lib::Media{
      name: element.inner_html(),
      url: element.value().attr("href").unwrap().to_string(),
    })
  }

  // Write all the values to the file
  for media in medias.iter() {
    csv_writer.write_record(&[media.name.clone(), media.url.clone()])?;
  }

  Ok(())

}