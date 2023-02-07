mod trope_download;
mod trope_lens;
mod trope_pipeline;
mod trope_scrape;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadNamespace, TropeDownloadPage, TropeDownloadPagelist,
};
pub use trope_lens::{
  TropeLensArgs, TropeLensMethod,
  TropeLensCategorize, TropeLensTutorial,
};
pub use trope_pipeline::{
  TropePipelineArgs, TropePipelineMethod,
  TropePipelineNamespacePagelist, TropePipelineAllPagelists,
  TropePipelineNamespacePages, TropePipelineAllPages,
};
pub use trope_scrape::{
  TropeScrapeArgs, TropeScrapeMethod,
  TropeScrapeNamespace, TropeScrapeNamespaceTotPages,
  TropeScrapePage, TropeScrapePagelist, TropeScrapeAllPages,
  TropeScrapePagelistLen, TropeScrapeGeneratePageIds,
};
