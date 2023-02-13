use std::fs::read_dir;

use trope_lib::{EntityType, PageIdLookup};

use crate::gen_pageid::translate_links_to_pageids;


/// Generate page ids for a page's mentions using the global list
pub fn gen_all_mention_pageids(args: trope_lib::TropeScrapeAllMentionPageIds) -> Result<(), Box<dyn std::error::Error>> {

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  for ns in trope_lib::ALL_NAMESPACES {
    log::info!("Generating for namespace {}", &ns);
    let sc_page_dir = trope_lib::sc_page_dir(&ns);
    for page_dir_result in read_dir(sc_page_dir)? {

      let page_dir_entry = page_dir_result?;
      let pagename = page_dir_entry.file_name().to_string_lossy().into_owned();
      let page_dir = page_dir_entry.path();

      let trope_mention_raw_path = page_dir.join("mentioned_tropes.csv");
      let media_mention_raw_path = page_dir.join("mentioned_media.csv");
      let trope_mention_pageid_path = page_dir.join("mentioned_trope_pageid.csv");
      let media_mention_pageid_path = page_dir.join("mentioned_media_pageid.csv");

      match translate_links_to_pageids(
        &trope_lookup, &pagename, &trope_mention_raw_path, &trope_mention_pageid_path, args.force
      ) {
        Ok(_) => {},
        Err(why) => {
          log::warn!(
            "Error translating mentioned trope links to pageids for {}: {}",
            pagename, why
          );
        }
      };
      match translate_links_to_pageids(
        &media_lookup, &pagename, &media_mention_raw_path, &media_mention_pageid_path, args.force
      ) {
        Ok(_) => {},
        Err(why) => {
          log::warn!(
            "Error translating mentioned media links to pageids for {}: {}",
            pagename, why
          );
        }
      }

    }
  }

  Ok(())

}
