mod trope_download;
mod trope_learn;
mod trope_pipeline;
mod trope_scrape;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadNamespace, TropeDownloadTropePage, TropeDownloadTropelist,
};
pub use trope_learn::{
  TropeLearnArgs, TropeLearnMethod,
  TropeLearnCategorize, TropeLearnTutorial,
};
pub use trope_pipeline::{
  TropePipelineArgs, TropePipelineMethod,
  TropePipelineNamespaceTropelist, TropePipelineAllTropelists,
};
pub use trope_scrape::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapeNamespace, TropeScrapeNamespaceTotPages,
  TropeScrapeTropePage, TropeScrapeTropelist, TropeScrapeAllTropes,
};
