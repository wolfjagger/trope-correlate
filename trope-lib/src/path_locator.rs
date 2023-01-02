use std::{env, path};

use once_cell::sync::Lazy;


static LAZY_WORKSPACE_DIR: Lazy<path::PathBuf> = Lazy::new(|| {
  let ws_str = env::var("CARGO_WORKSPACE_DIR")
    .expect("Cannot find CARGO_WORKSPACE_DIR envvar; is it set in .cargo/config.toml?");
  path::PathBuf::from(ws_str)
});
pub fn workspace_dir() -> &'static path::Path { &*LAZY_WORKSPACE_DIR }
static LAZY_DATA_DIR: Lazy<path::PathBuf> = Lazy::new(||
  LAZY_WORKSPACE_DIR.join("test_data")
);
pub fn data_dir() -> &'static path::Path { &*LAZY_DATA_DIR }
static LAZY_DOWNLOAD_DIR: Lazy<path::PathBuf> = Lazy::new(||
  LAZY_DATA_DIR.join("download")
);
pub fn download_dir() -> &'static path::Path { &*LAZY_DOWNLOAD_DIR }
static LAZY_SCRAPE_DIR: Lazy<path::PathBuf> = Lazy::new(||
  LAZY_DATA_DIR.join("scrape")
);
pub fn scrape_dir() -> &'static path::Path { &*LAZY_SCRAPE_DIR }
