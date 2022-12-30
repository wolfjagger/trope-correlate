use std::{fs, path};
use scraper::{Html, Selector};
use serde_json;
use trope_lib::TropeGeneralJson;

use crate::read_html::read_html_file;


/// Scrape trope page for e.g. title, subpages, mentioned media
pub fn scrape_trope(
  name: &str, in_path: path::PathBuf, out_dir: &path::Path,
  encrypted: bool, force: bool
) -> Result<(), Box<dyn std::error::Error>> {

  if out_dir.exists() {
    if force {
      println!("Trope directory exists, scraping and overwriting {}...", name);
      fs::remove_dir_all(&out_dir)?;
    } else {
      println!("Trope directory exists, skipping {}...", name);
      return Ok(());
    }
  } else {
    println!("Scraping {}...", name);
  }

  fs::create_dir_all(&out_dir)?;

  let in_doc = read_html_file(in_path, encrypted);

  let general_json_path = out_dir.clone().join("general.json");
  let general_json_file = match fs::File::create(&general_json_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", general_json_path.display(), why),
  };

  let mentioned_tropes_path = out_dir.clone().join("mentioned_tropes.csv");
  let mut mentioned_tropes_csv = match csv::Writer::from_path(&mentioned_tropes_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", mentioned_tropes_path.display(), why),
  };

  let mentioned_media_path = out_dir.clone().join("mentioned_media.csv");
  let mut mentioned_media_csv = match csv::Writer::from_path(&mentioned_media_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", mentioned_media_path.display(), why),
  };


  // Scrape doc

  let (general_trope_json, mentioned_tropes, mentioned_media) = scrape_doc(&in_doc);


  // Write to output

  serde_json::to_writer_pretty(general_json_file, &general_trope_json)?;

  // Write all mentioned tropes
  for trope_link in mentioned_tropes.iter() {
    mentioned_tropes_csv.write_record(&[trope_link.name.clone(), trope_link.url.clone()])?;
  }

  // Write all mentioned media
  for media_link in mentioned_media.iter() {
    mentioned_media_csv.write_record(&[media_link.name.clone(), media_link.url.clone()])?;
  }

  Ok(())

}


fn scrape_doc(doc: &Html) -> (
  TropeGeneralJson,
  Vec<trope_lib::NamedLink>,
  Vec<trope_lib::NamedLink>,
) {

  // Selectors

  let title_selector = Selector::parse("title").expect("Error creating title selector");
  let subpage_selector = Selector::parse("ul.subpage-links > li > a").expect("Error creating subpage selector");
  let span_wrapper_selector = Selector::parse("span.wrapper").expect("Error creating span wrapper selector");
  let link_selector = Selector::parse("div#main-article a").expect("Error creating link selector");


  // Scrape general

  let title = doc.select(&title_selector).next().expect("Error finding title").inner_html();

  let subpages = doc.select(&subpage_selector).map(|el| {
    trope_lib::NamedLink::new(
      el.select(&span_wrapper_selector).next().unwrap().text().next().unwrap().trim().to_string(),
      el.value().attr("href").unwrap().trim().to_string(),
    )
  }).filter(
    // Filter out "Create New" non-subpage
    |named_link| !named_link.name.eq_ignore_ascii_case("create new")
  ).collect::<Vec<_>>();

  let mut general_trope_json = trope_lib::TropeGeneralJson::default();
  general_trope_json.title = title;
  general_trope_json.subpages = subpages;


  // Scrape mentioned media
  // For every link mentioned, get the inner html (name) and href (url)
  // TODO: Very important to get this right. Consider:
  //  - Only pick links after the <h2> containing "Examples"
  //  - Consider bullets and sub-bullets separately
  //  - Some tropes do not organize media under folders, but on separate pages under the namespace of the trope,
  //   e.g. /pmwiki/pmwiki.php/Main/ShipToShipCombat with /pmwiki/pmwiki.php/ShipToShipCombat/AnimeAndManga

  // Use partition instead of filter so we can debug & see what we are dropping
  let (wiki_link_els, _nonwiki_link_els): (Vec<_>, Vec<_>) = doc.select(&link_selector).partition(
    // Filter out links that don't start with the pmwiki address
    |el| el.value().attr("href").filter(|attr| attr.starts_with("/pmwiki/pmwiki.php/")).is_some()
  );

  let named_wiki_links = wiki_link_els.into_iter().map(|el| trope_lib::NamedLink::new(
    el.inner_html().trim().to_string(),
    el.value().attr("href").unwrap().trim().to_string(),
  ));

  let (nonhtml_wiki_links, _html_wiki_links): (Vec<_>, Vec<_>) = named_wiki_links.partition(|link| {
    !(link.name.contains("<") || link.name.contains(">"))
  });

  let (mut trope_links, mut media_links, mut _other_links): (Vec<_>, Vec<_>, Vec<_>) = (vec![], vec![], vec![]);
  for link in nonhtml_wiki_links {
    match link.link_type() {
      trope_lib::NamedLinkType::Trope => trope_links.push(link),
      trope_lib::NamedLinkType::Media => media_links.push(link),
      trope_lib::NamedLinkType::Other => _other_links.push(link),
    }
  }

  // println!("=================");
  // println!("{:?}", _nonwiki_link_els);
  // println!("=================");
  // println!("{:?}", _html_wiki_links);
  // println!("=================");
  // println!("{:?}", _other_links);
  // println!("=================");
  // println!("{:?}", nonhtml_media_links);
  // println!("=================");

  (general_trope_json, trope_links, media_links)

}
