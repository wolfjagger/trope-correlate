use trope_lib::EntityType;

use crate::gen_pageid::gen_entity_type_pageids;


/// Loop through pages to generate ids for each
pub fn gen_global_pageids(args: trope_lib::TropeScrapeGlobalPageIds) -> Result<(), Box<dyn std::error::Error>> {

  let entity_types = [EntityType::Trope, EntityType::Media, EntityType::Other];

  for et in entity_types {
    gen_entity_type_pageids(et, args.force)?;
  }

  Ok(())

}
