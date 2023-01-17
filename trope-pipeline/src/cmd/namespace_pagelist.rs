use trope_download;
use trope_lib;
use trope_scrape;


/// Download a trope page
pub fn namespace_pagelist(args: trope_lib::TropePipelineNamespacePagelist) -> Result<(), Box<dyn std::error::Error>> {

  let (namespace, unencrypted, force) = (args.namespace, args.unencrypted, args.force);

  let download_page1_args = trope_lib::TropeDownloadNamespace {
    beg_page: 1,
    end_page: 1,
    namespace: namespace.clone(),
    unencrypted,
    force,
    sleep_sec: 5,
  };

  trope_download::save_namespace(download_page1_args)?;

  let scrape_tot_page_args = trope_lib::TropeScrapeNamespaceTotPages{
    namespace: namespace.clone(),
    unencrypted,
  };

  let tot_pages = trope_scrape::get_namespace_tot_pages(scrape_tot_page_args)? as u8;
  if tot_pages < 1 {
    panic!("no pages found for namespace");
  }

  println!("Tot pages: {}", tot_pages);

  let download_ns_args = trope_lib::TropeDownloadNamespace {
    beg_page: 1,
    end_page: tot_pages,
    namespace: namespace.clone(),
    unencrypted,
    force,
    sleep_sec: 5,
  };

  trope_download::save_namespace(download_ns_args)?;

  let scrape_ns_args = trope_lib::TropeScrapeNamespace {
    beg_page: 1,
    end_page: tot_pages,
    namespace: namespace.clone(),
    unencrypted,
    force,
  };

  trope_scrape::scrape_namespace(scrape_ns_args)?;

  Ok(())

}
