use trope_lib;

use crate::namespace_pagelist;


/// Download a trope page
pub fn all_pagelists(args: trope_lib::TropePipelineAllPagelists) -> Result<(), Box<dyn std::error::Error>> {

  let (unencrypted, force) = (args.unencrypted, args.force);

  for ns in trope_lib::ALL_NAMESPACES {
    log::info!("\n\n=====");
    log::info!("Namespace {}", &ns);
    let ns_tl_args = trope_lib::TropePipelineNamespacePagelist{
      namespace: ns.to_string(),
      unencrypted,
      force,
    };
    namespace_pagelist(ns_tl_args)?;
  }

  Ok(())

}
