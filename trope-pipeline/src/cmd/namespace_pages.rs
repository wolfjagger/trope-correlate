use trope_lib;

use crate::namespace_pagelist;


/// Download a trope page
pub fn namespace_pages(args: trope_lib::TropePipelineNamespacePages) -> Result<(), Box<dyn std::error::Error>> {

  let (namespace, unencrypted, force) = (args.namespace, args.unencrypted, args.force);

  println!("Fetching pagelist...");

  let namespace_pagelist_args = trope_lib::TropePipelineNamespacePagelist{
    namespace,
    unencrypted,
    force,
  };
  namespace_pagelist(namespace_pagelist_args)?;

  Ok(())

}
