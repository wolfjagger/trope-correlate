use std::path;


/// Download all the pages
pub fn scrape_trope_html_to_path(
  in_dir: &path::Path, out_dir: &path::Path, name: &str,
  encrypted: bool
) -> Result<(), Box<dyn std::error::Error>> {

  let _in_file_name = in_dir.clone().join(
    if encrypted {
      format!("{}.html.br", &name)
    } else {
      format!("{}.html", &name)
    }
  );

  let _out_file_name = out_dir.clone().join(format!("{}.csv", &name));

  // TODO: scrape trope

  Ok(())

}