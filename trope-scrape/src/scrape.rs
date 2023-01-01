use std::{fs, path};
use regex::Regex;
use scraper::{Html, Selector};
use serde_json;
use trope_lib::TropeGeneralJson;


/// Scrape trope page for e.g. title, subpages, mentioned media
pub fn scrape_trope(
  in_doc: &Html, out_dir: &path::Path
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

  let (general_trope_json, mentioned_media) = scrape_doc(&in_doc);


  // Write to output

  serde_json::to_writer_pretty(general_json_file, &general_trope_json)?;

  // Write all the values to the file
  for media in mentioned_media.iter() {
    mentioned_media_csv.write_record(&[media.name.clone(), media.url.clone()])?;
  }

  Ok(())

}


fn scrape_doc(doc: &Html) -> (TropeGeneralJson, Vec<trope_lib::NamedLink>) {

  // Selectors

  let title_selector = Selector::parse("title").expect("Error creating title selector");
  let subpage_selector = Selector::parse("ul.subpage-links > li > a").expect("Error creating subpage selector");
  let span_wrapper_selector = Selector::parse("span.wrapper").expect("Error creating span wrapper selector");
  let pmwiki_link_selector = Selector::parse("div#main-article a").expect("Error creating pmwiki link selector");

  // Regexes

  let namespace_re = Regex::new("/pmwiki/pmwiki.php/([^/]*)/").expect("Error creating namespace regex");


  // Scrape general

  let title = doc.select(&title_selector).next().expect("Error finding title").inner_html();

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


  // Scrape mentioned media
  // For every link mentioned, get the inner html (name) and href (url)
  // TODO: Very important to get this right. Consider:
  //  - Only pick links after the <h2> containing "Examples"
  //  - Consider bullets and sub-bullets separately
  //  - Some tropes do not organize media under folders, but on separate pages under the namespace of the trope,
  //   e.g. /pmwiki/pmwiki.php/Main/ShipToShipCombat with /pmwiki/pmwiki.php/ShipToShipCombat/AnimeAndManga

  // Use partition instead of filter so we can debug & see what we are dropping
  let (wiki_link_els, _nonwiki_link_els): (Vec<_>, Vec<_>) = doc.select(&pmwiki_link_selector).partition(
    // Filter out links that don't start with the pmwiki address
    |el| el.value().attr("href").filter(|attr| attr.starts_with("/pmwiki/pmwiki.php/")).is_some()
  );

  let (media_links, _nonmedia_links): (Vec<_>, Vec<_>) = wiki_link_els.into_iter().map(|el| {
    trope_lib::NamedLink{
      name: el.inner_html().trim().to_string(),
      url: el.value().attr("href").unwrap().trim().to_string(),
    }
  }).partition(|link| {
    // Try to filter to media based on url namespace
    namespace_re.captures(&link.url).and_then(
      |cap| cap.get(1).map(|m| m.as_str())
    ).filter(
      |namespace| trope_lib::KNOWN_MEDIA_NAMESPACES.iter().any(|s| s == namespace)
    ).is_some()
  });

  let (nonhtml_media_links, _html_media_links): (Vec<_>, Vec<_>) = media_links.into_iter().partition(|link| {
    !(link.name.contains("<") || link.name.contains(">"))
  });

  // println!("=================");
  // println!("{:?}", _nonwiki_link_els);
  // println!("=================");
  // println!("{:?}", _nonmedia_links);
  // println!("=================");
  // println!("{:?}", _html_media_links);
  // println!("=================");
  // println!("{:?}", nonhtml_media_links);
  // println!("=================");

  (general_trope_json, nonhtml_media_links)

}
