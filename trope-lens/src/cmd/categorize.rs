use std::str::FromStr;

use trope_lib::{EntityType, PageIdLookup, TropeLensCategorize};

use crate::LensError;


pub fn categorize(args: TropeLensCategorize) -> Result<(), LensError> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let page = args.pagename;
  log::info!("Categorizing page {}...", page);
  let sc_page_dir = trope_lib::sc_page_dir(&ns).join(&page);

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  log::trace!(
    "{} total trope pageids, {} total media pageids",
    trope_lookup.len(), media_lookup.len()
  );

  log::info!("Assembling tropes...");
  let mentioned_tropes_path = sc_page_dir.join("mentioned_tropes.csv");
  let (
    mentioned_trope_pageids, _missing_tropes
  ) = trope_lookup.pageids_from_path(&mentioned_tropes_path)?;

  log::info!("Assembling media...");
  let mentioned_media_path = sc_page_dir.join("mentioned_media.csv");
  let (
    mentioned_media_pageids, _missing_media
  ) = media_lookup.pageids_from_path(&mentioned_media_path)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  log::trace!(
    "{} mentioned tropes, {} mentioned media",
    mentioned_trope_pageids.len(), mentioned_media_pageids.len()
  );

  log::warn!("TODO: Feed info into model and get output namespace guess");

  Ok(())

}


fn _do_tensor_propagation() {

  // Look in trope-teach e.g. tutorial.rs for how to feed the input through
  //  the model to get output guess

}
