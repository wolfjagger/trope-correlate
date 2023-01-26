use trope_download;
use trope_lib;
use trope_scrape;

use crate::namespace_pagelist;


/// Download a trope page
pub fn namespace_pages(args: trope_lib::TropePipelineNamespacePages) -> Result<(), Box<dyn std::error::Error>> {

  let (
    namespace, unencrypted, force, sleep_sec, random_seed, progress
  ) = (
    args.namespace, args.unencrypted, args.force, args.sleep_sec, args.random_seed, args.progress
  );

  log::info!("Fetching pagelist...");

  let namespace_pagelist_args = trope_lib::TropePipelineNamespacePagelist{
    namespace: namespace.clone(),
    unencrypted,
    force,
  };
  namespace_pagelist(namespace_pagelist_args)?;

  let get_pagelist_len_args = trope_lib::TropeScrapePagelistLen{
    namespace: namespace.clone(),
    unencrypted,
  };
  let end_record = trope_scrape::get_pagelist_len(get_pagelist_len_args)? as u64;

  let save_pagelist_args = trope_lib::TropeDownloadPagelist{
    beg_record: 0,
    end_record,
    namespace: namespace.clone(),
    unencrypted,
    force,
    sleep_sec,
    random_seed,
    progress,
  };
  trope_download::save_pagelist(save_pagelist_args)?;

  let scrape_pagelist_args = trope_lib::TropeScrapePagelist{
    beg_record: 0,
    end_record,
    namespace: namespace.clone(),
    unencrypted,
    force,
    random_seed,
    progress,
  };
  trope_scrape::scrape_pagelist(scrape_pagelist_args)?;

  Ok(())

}
