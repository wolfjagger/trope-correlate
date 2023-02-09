use std::str::FromStr;

use trope_lib::{EntityType, PageIdLookup};

use crate::gen_pageid::translate_links_to_pageids;


/// Generate page ids for a page's mentions using the global list
pub fn gen_mention_page_ids(args: trope_lib::TropeScrapeMentionPageIds) -> Result<(), Box<dyn std::error::Error>> {

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  let pagename = args.pagename;
  let ns = trope_lib::Namespace::from_str(&args.namespace)?;
  let sc_page_dir = trope_lib::sc_page_dir(&ns).join(&pagename);

  let trope_mention_raw_path = sc_page_dir.join("mentioned_tropes.csv");
  let media_mention_raw_path = sc_page_dir.join("mentioned_media.csv");
  let trope_mention_pageid_path = sc_page_dir.join("mentioned_trope_pageid.csv");
  let media_mention_pageid_path = sc_page_dir.join("mentioned_media_pageid.csv");

  translate_links_to_pageids(
    &trope_lookup, &pagename, &trope_mention_raw_path, &trope_mention_pageid_path, args.force
  )?;
  translate_links_to_pageids(
    &media_lookup, &pagename, &media_mention_raw_path, &media_mention_pageid_path, args.force
  )?;

  Ok(())

}
