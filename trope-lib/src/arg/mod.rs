mod trope_download;
mod trope_learn;
mod trope_pipeline;
mod trope_scrape;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadNamespace, TropeDownloadPage, TropeDownloadPagelist,
};
pub use trope_learn::{
  TropeLearnArgs, TropeLearnMethod,
  TropeLearnCategorize, TropeLearnTutorial,
};
pub use trope_pipeline::{
  TropePipelineArgs, TropePipelineMethod,
  TropePipelineNamespacePagelist, TropePipelineAllPagelists,
};
pub use trope_scrape::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapeNamespace, TropeScrapeNamespaceTotPages,
  TropeScrapePage, TropeScrapePagelist, TropeScrapeAllPages,
};
