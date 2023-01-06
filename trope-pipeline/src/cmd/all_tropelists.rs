use trope_lib;

use crate::namespace_tropelist;


/// Download a trope page
pub fn all_tropelists(args: trope_lib::TropePipelineAllTropelists) -> Result<(), Box<dyn std::error::Error>> {

  let (unencrypted, force) = (args.unencrypted, args.force);

  for ns in trope_lib::ALL_NAMESPACES {
    println!("\n\n=====");
    println!("Namespace {}", &ns);
    let ns_tl_args = trope_lib::TropePipelineNamespaceTropelist{
      namespace: ns.to_string(),
      unencrypted,
      force,
    };
    namespace_tropelist(ns_tl_args)?;
  }

  Ok(())

}
