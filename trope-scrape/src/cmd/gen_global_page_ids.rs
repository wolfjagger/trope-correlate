use std::fs::{create_dir_all, read_dir};

use trope_lib::{EntityType, PageId};


/// Loop through pages to generate ids for each
pub fn gen_global_page_ids(_: trope_lib::TropeScrapeGlobalPageIds) -> Result<(), Box<dyn std::error::Error>> {

  let sc_page_dir = trope_lib::scrape_dir().join("page");
  let sc_pageid_dir = trope_lib::sc_pageid_dir();
  create_dir_all(&sc_pageid_dir)?;

  let entity_types = [EntityType::Trope, EntityType::Media, EntityType::Other];

  for et in entity_types {

    let et_page_dir = sc_page_dir.join(et.to_string());
    let et_pageid_path = sc_pageid_dir.join(format!("{}.csv", et));
    let mut et_pageid_csv = match csv::Writer::from_path(&et_pageid_path) {
      Ok(w) => w,
      Err(why) => panic!("Couldn't write to {}: {}", et_pageid_path.display(), why),
    };

    let mut et_pageid_lookup: Vec<PageId> = vec![];
    let mut idx = 0;

    for ns_dir in read_dir(et_page_dir)? {
      for page_path in read_dir(ns_dir?.path())? {
        // Lossy conversion should be fine
        let page_str = page_path?.file_name().to_string_lossy().into_owned();
        et_pageid_lookup.push(PageId{ id: idx, page: page_str });
        idx += 1;
      }
    }

    for pageid in et_pageid_lookup {
      et_pageid_csv.serialize(pageid)?;
    }

  }

  Ok(())

}
