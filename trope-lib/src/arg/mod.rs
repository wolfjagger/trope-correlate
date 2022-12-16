mod trope_download;
mod trope_scraper;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadPagelist, TropeDownloadTropePage, TropeDownloadTropelist,
};
pub use trope_scraper::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapePagelist, TropeScrapeTropePage,
};
