use std::{fs, io::{prelude::*, BufWriter}, path};
use brotli::BrotliDecompress;
use scraper::Html;


pub fn read_html_file(file_name: path::PathBuf, encrypted: bool) -> Result<Html, Box<dyn std::error::Error>> {

  // Input file
  let html_content: String = if encrypted {

    let mut enc_content = fs::File::open(&file_name).expect(&format!("Cannot open {}", &file_name.display()));

    // Cannot write directly to string; use bytes and then from_utf8
    let mut html_writer = BufWriter::new(Vec::new());

    // Decode using brotli decompression
    BrotliDecompress(&mut enc_content, &mut html_writer).expect(&"Error during Brotli decompression");

    let html_bytes = html_writer.into_inner().expect("Into inner error");
    String::from_utf8(html_bytes).expect("Error writing html string from bytes")

  } else {

    let mut html_content = String::new();
    fs::File::open(file_name).and_then(|mut fi| fi.read_to_string(&mut html_content)).expect("Error writing html string from file");
    html_content

  };

  Ok(Html::parse_document(&html_content))

}
