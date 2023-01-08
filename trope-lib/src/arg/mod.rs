mod trope_download;
mod trope_pipeline;
mod trope_scrape;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadNamespace, TropeDownloadPagelist, TropeDownloadTropePage, TropeDownloadTropelist,
};
pub use trope_pipeline::{
  TropePipelineArgs, TropePipelineMethod,
  TropePipelineNamespaceTropelist, TropePipelineAllTropelists,
};
pub use trope_scrape::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapeNamespace, TropeScrapePagelist, TropeScrapeNamespaceTotPages,
  TropeScrapeTropePage, TropeScrapeTropelist, TropeScrapeAllTropes,
};
