mod arg;
mod constant;
mod trope;
mod util;


pub use arg::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadPagelist, TropeDownloadTropePage, TropeDownloadTropelist,
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapePagelist,
};
pub use constant::DATA_DIR;
pub use trope::Trope;
pub use util::{Namespace, Pagetype};
