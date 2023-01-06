mod arg;
mod namespace;
mod path_locator;
mod serialization;
mod util;


pub use arg::*;
pub use namespace::{Namespace, EntityType};
pub use path_locator::{
  workspace_dir, data_dir, download_dir, scrape_dir,
  dl_trope_dir, sc_trope_dir,
  dl_namespace_dir, sc_tropelist_dir
};
pub use serialization::{NamedLink, TropeGeneralJson};
pub use util::Pagetype;
