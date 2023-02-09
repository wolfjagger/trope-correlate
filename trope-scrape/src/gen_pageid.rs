use std::path::Path;

use trope_lib::{PageIdLookup, NamedLink};

use crate::error::ScrapeError;


pub fn translate_links_to_pageids(
  pageid_lookup: &PageIdLookup,
  pagename: &str, link_csv_path: &Path, pageid_csv_path: &Path,
  force: bool
) -> Result<(), ScrapeError> {

  if pageid_csv_path.try_exists()? {
    if force {
      log::info!("Pageid file exists, overwriting {}...", pagename);
    } else {
      log::info!("Pageid file exists, skipping {}...", pagename);
      return Ok(());
    }
  }

  let link_csv = match csv::Reader::from_path(&link_csv_path) {
    Ok(r) => r,
    Err(why) => panic!("Couldn't read from {}: {}", link_csv_path.display(), why),
  };
  let mut pageid_csv = match csv::Writer::from_path(&pageid_csv_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", pageid_csv_path.display(), why),
  };

  for mention in link_csv.into_deserialize::<NamedLink>() {
    match mention {
      Ok(named_link) => {
        let true_name = named_link.url_page_name().to_string();
        let pageid = pageid_lookup.pageid_from_page(&true_name);
        match pageid {
          Some(id) => {
            pageid_csv.serialize(id)?;
          },
          None => {
            log::debug!("Could not find pageid for page {}", pagename)
          }
        }
      },
      Err(err) => {
        panic!("Error deserializing mention list in {}: {}", pagename, err)
      }
    }
  }

  Ok(())

}
