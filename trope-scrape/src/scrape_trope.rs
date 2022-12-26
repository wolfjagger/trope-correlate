use std::path;

use trope_lib;
use crate::{
  read_html::read_html_file,
  scrape::scrape_trope
};


/// Download a trope page from args
pub fn scrape_trope_page(args: trope_lib::TropeScrapeTropePage) -> Result<(), Box<dyn std::error::Error>> {

  let name = args.name;

  // Set up input html
  let in_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("trope_page");
  let in_path = in_dir.join(
    if args.encrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );
  let in_html = read_html_file(in_path, args.encrypted).expect("Error reading html file");

  // Set up output file in same parent dir
  let out_dir = path::PathBuf::from("..")
    .join(trope_lib::DATA_DIR)
    .join("tropes")
    .join(&name);

  scrape_trope(&name, &in_html, &out_dir, args.force)?;

  Ok(())

}
