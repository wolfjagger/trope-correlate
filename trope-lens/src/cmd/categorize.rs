use std::{path::Path, str::FromStr};
use dfdx::{prelude::*, gradients::Gradients};

use trope_lib::{EntityType, NamedLink, PageId, PageIdLookup, TropeLensCategorize};

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
  ) = assemble_pageids(&mentioned_tropes_path, &trope_lookup)?;

  log::info!("Assembling media...");
  let mentioned_media_path = sc_page_dir.join("mentioned_media.csv");
  let (
    mentioned_media_pageids, _missing_media
  ) = assemble_pageids(&mentioned_media_path, &media_lookup)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  log::trace!(
    "{} mentioned tropes, {} mentioned media",
    mentioned_trope_pageids.len(), mentioned_media_pageids.len()
  );

  log::warn!("TODO: Feed info into model and get output namespace guess");

  Ok(())

}


fn assemble_pageids(p: &Path, page_lookup: &PageIdLookup)
-> Result<(Vec<PageId>, Vec<String>), csv::Error> {

  let mentioned_pages = path_to_page_names(&p)?;

  let (found_pages, missing_pages): (Vec<_>, Vec<_>) = mentioned_pages.into_iter().partition(
    |name| page_lookup.contains_page(&name)
  );
  let ment_page_pageids: Vec<_> = found_pages.into_iter().map(
    |name| page_lookup.pageid_from_page(&name).unwrap()
  ).collect();

  log::trace!("Found pageids:");
  for t_id in &ment_page_pageids {
    log::trace!("{}", t_id);
  }

  log::trace!("Missing pages:");
  for missing_page in &missing_pages {
    log::trace!("{}", missing_page);
  }

  Ok((ment_page_pageids, missing_pages))

}


fn path_to_page_names(p: &Path) -> Result<Vec<String>, csv::Error> {
  // Note: url is the source of truth in these mentions; short name is different page-to-page
  let mentions = csv::Reader::from_path(p)?.into_deserialize::<NamedLink>();
  mentions.map(|m_result| m_result.map(|m| m.url_page_name().to_string())).collect()
}


fn _do_tensor_propagation() {

  // Look in trope-teach e.g. tutorial.rs for how to feed the input through
  //  the model to get output guess

}
