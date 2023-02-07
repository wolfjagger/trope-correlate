mod trope_download;
mod trope_lens;
mod trope_pipeline;
mod trope_scrape;
mod trope_teach;


pub use trope_download::{
  TropeDownloadArgs, TropeDownloadMethod,
  TropeDownloadNamespace, TropeDownloadPage, TropeDownloadPagelist,
};
pub use trope_lens::{
  TropeLensArgs, TropeLensMethod,
  TropeLensCategorize,
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
pub use trope_teach::{
  TropeTeachArgs, TropeTeachMethod,
  TropeTeachCategorize, TropeTeachTutorial,
};
