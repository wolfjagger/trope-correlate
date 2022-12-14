use trope_lib;
use crate::scrape::scrape_trope;


/// Download a trope page from args
pub fn scrape_trope_page(args: trope_lib::TropeScrapeTropePage) -> Result<(), Box<dyn std::error::Error>> {

  let name = args.name;

  // Set up input html
  let in_dir = trope_lib::dl_trope_dir();
  let in_path = in_dir.join(
    if !args.unencrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );

  // Set up output file in same parent dir
  let out_dir = trope_lib::sc_trope_dir().join(&name);

  scrape_trope(&name, in_path, &out_dir, !args.unencrypted, args.force)?;

  Ok(())

}
