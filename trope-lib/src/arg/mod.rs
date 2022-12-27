mod trope_download;
mod trope_scrape;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadPagelist, TropeDownloadTropePage, TropeDownloadTropelist,
};
pub use trope_scrape::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapePagelist, TropeScrapeTropePage, TropeScrapeTropelist, TropeScrapeAllTropes,
};
