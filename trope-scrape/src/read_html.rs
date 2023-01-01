use std::{fs, io::{prelude::*, BufWriter}, path};
use brotli::BrotliDecompress;
use scraper::Html;


pub fn read_html_file(file_name: path::PathBuf, encrypted: bool) -> Html {

  let mut fi = fs::File::open(&file_name).expect(&format!("Cannot open {}", &file_name.display()));

  // Cannot write directly to string; use bytes and convert from latin1
  let html_bytes = if encrypted {

    let mut html_writer = BufWriter::new(Vec::new());

    // Decode using brotli decompression
    BrotliDecompress(&mut fi, &mut html_writer).expect("Error during Brotli decompression");

    html_writer.into_inner().expect("Into inner error")

  } else {

    let mut html_bytes = vec![];
    fi.read_to_end(&mut html_bytes).expect("Error reading html file");
    html_bytes

  };

  let html_content = latin1_to_string(&html_bytes);

  Html::parse_document(&html_content)

}

// The tvtropes html is stored as iso-8859-1, so map to utf-8
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}
