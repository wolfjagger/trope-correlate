use std::{fs, path};
use scraper::{Html, Selector};
use serde_json;
use trope_lib::TropeGeneralJson;

use crate::read_html::read_html_file;


/// Download all the pages
pub fn scrape_trope_to_medialist(
  in_dir: &path::Path, out_dir: &path::Path, name: &str,
  encrypted: bool
) -> Result<(), Box<dyn std::error::Error>> {

  // Set up output files (first, to panic early)

  fs::create_dir_all(&out_dir)?;

  let general_json_path = out_dir.clone().join("general.json");
  let general_json_file = match fs::File::create(&general_json_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", general_json_path.display(), why),
  };

  let mentioned_media_path = out_dir.clone().join("mentioned_media.csv");
  let mut mentioned_media_csv = match csv::Writer::from_path(&mentioned_media_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", mentioned_media_path.display(), why),
  };


  // Scrape doc

  let in_file_name = in_dir.clone().join(
    if encrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );
  let document = read_html_file(in_file_name, encrypted).expect("Error reading html file");
  let (general_trope_json, mentioned_media) = scrape_doc(&document);


  // Write to output

  serde_json::to_writer_pretty(general_json_file, &general_trope_json)?;

  // Write all the values to the file
  for media in mentioned_media.iter() {
    mentioned_media_csv.write_record(&[media.name.clone(), media.url.clone()])?;
  }

  Ok(())

}


fn scrape_doc(doc: &Html) -> (TropeGeneralJson, Vec<trope_lib::NamedLink>) {

  let title_selector = Selector::parse("title").expect("Error creating title selector");
  let title = doc.select(&title_selector).next().expect("Error finding title").inner_html();

  let subpage_selector = Selector::parse("ul.subpage-links > li > a").expect("Error finding subpages");
  let span_wrapper_selector = Selector::parse("span.wrapper").expect("Error creating span wrapper selector");
  let subpages = doc.select(&subpage_selector).map(|el| {
    trope_lib::NamedLink{
      name: el.select(&span_wrapper_selector).next().unwrap().text().next().unwrap().trim().to_string(),
      url: el.value().attr("href").unwrap().trim().to_string(),
    }
  }).filter(
    // Filter out "Create New" non-subpage
    |named_link| !named_link.name.eq_ignore_ascii_case("create new")
  ).collect::<Vec<_>>();

  let mut general_trope_json = trope_lib::TropeGeneralJson::default();
  general_trope_json.title = title;
  general_trope_json.subpages = subpages;

  // TODO: Need to be more selective here
  let media_selector = Selector::parse("div#main-article a").unwrap();
  let media_links = doc.select(&media_selector);

  // For every media mentioned, get the inner html (media_name) and href (media_url)
  let mut medias: Vec<trope_lib::NamedLink> = Vec::new();
  for element in media_links {
    medias.push(trope_lib::NamedLink{
      name: element.inner_html().trim().to_string(),
      url: element.value().attr("href").unwrap().trim().to_string(),
    })
  }

  return (general_trope_json, medias)

}
