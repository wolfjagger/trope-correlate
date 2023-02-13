use std::{
  fs::{create_dir_all, read_dir},
  path::Path,
};

use trope_lib::{PageId, PageIdLookup, NamedLink, EntityType};

use crate::error::ScrapeError;


pub fn gen_entity_type_pageids(et: EntityType, force: bool) -> Result<(), ScrapeError> {

  let sc_page_dir = trope_lib::scrape_dir().join("page");
  let sc_pageid_dir = trope_lib::sc_pageid_dir();
  create_dir_all(&sc_pageid_dir)?;

  let et_page_dir = sc_page_dir.join(et.to_string());
  let et_pageid_path = sc_pageid_dir.join(format!("{}.csv", et));

  if et_pageid_path.try_exists()? {
    if force {
      log::debug!("Pageid file exists, overwriting {}...", et);
    } else {
      log::debug!("Pageid file exists, skipping {}...", et);
      return Ok(());
    }
  }

  let mut et_pageid_csv = match csv::Writer::from_path(&et_pageid_path) {
    Ok(w) => w,
    Err(why) => panic!("Couldn't write to {}: {}", et_pageid_path.display(), why),
  };

  let mut et_pageid_lookup: Vec<PageId> = vec![];
  let mut idx = 0;

  for ns_dir in read_dir(et_page_dir)? {
    for page_path in read_dir(ns_dir?.path())? {
      // Lossy conversion should be fine
      let page_str = page_path?.file_name().to_string_lossy().to_lowercase();
      et_pageid_lookup.push(PageId{ id: idx, page: page_str });
      idx += 1;
    }
  }

  for pageid in et_pageid_lookup {
    et_pageid_csv.serialize(pageid)?;
  }

  Ok(())

}



pub fn translate_links_to_pageids(
  pageid_lookup: &PageIdLookup,
  pagename: &str, link_csv_path: &Path, pageid_csv_path: &Path,
  force: bool
) -> Result<(), ScrapeError> {

  // NOTE: Consider saving JUST the ids, to make the files faster to read in trope-teach

  if pageid_csv_path.try_exists()? {
    if force {
      log::debug!("Pageid file exists, overwriting {}...", pageid_csv_path.display());
    } else {
      log::debug!("Pageid file exists, skipping {}...", pageid_csv_path.display());
      return Ok(());
    }
  }

  let link_csv = csv::Reader::from_path(&link_csv_path)?;
  let mut pageid_csv = csv::Writer::from_path(&pageid_csv_path)?;

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
            log::debug!("Could not find pageid for page {}", true_name)
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
