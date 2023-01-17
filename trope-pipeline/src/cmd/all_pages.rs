use trope_lib;

use crate::{all_pagelists, namespace_pages};


/// Download a trope page
pub fn all_pages(args: trope_lib::TropePipelineAllPages) -> Result<(), Box<dyn std::error::Error>> {

  let (unencrypted, force) = (args.unencrypted, args.force);

  let all_pagelists_args = trope_lib::TropePipelineAllPagelists{
    unencrypted,
    force,
  };
  all_pagelists(all_pagelists_args)?;

  for ns in trope_lib::ALL_NAMESPACES {
    println!("\n\n=====");
    println!("Namespace {}", &ns);
    let ns_tl_args = trope_lib::TropePipelineNamespacePages{
      namespace: ns.to_string(),
      unencrypted,
      force,
    };
    namespace_pages(ns_tl_args)?;
  }

  Ok(())

}