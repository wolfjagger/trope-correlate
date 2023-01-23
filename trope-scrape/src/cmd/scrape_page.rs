use std::str::FromStr;
use trope_lib;
use crate::scrape;


/// Download a trope page from args
pub fn scrape_page(args: trope_lib::TropeScrapePage) -> Result<(), Box<dyn std::error::Error>> {

  let name = args.pagename;
  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  // Set up input html
  let in_dir = trope_lib::dl_page_dir(&ns);
  let in_path = in_dir.join(
    if !args.unencrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );

  // Set up output file in same parent dir
  let out_dir = trope_lib::sc_page_dir(&ns).join(&name);

  scrape::scrape_page(&name, in_path, &out_dir, !args.unencrypted, args.force)?;

  Ok(())

}
