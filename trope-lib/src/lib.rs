mod arg;
mod namespace;
mod path_locator;
mod serialization;
mod util;


pub use arg::*;
pub use namespace::{Namespace, EntityType, ALL_NAMESPACES};
pub use path_locator::{
  workspace_dir, data_dir, download_dir, scrape_dir,
  dl_page_dir, sc_page_dir,
  dl_namespace_dir, sc_pagelist_dir
};
pub use serialization::{NamedLink, TropeGeneralJson};
pub use util::Pagetype;
