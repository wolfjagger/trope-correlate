use std::fs;
use reqwest;


/// Get header from local file and parse for headers (can panic!)
pub fn get_header_map() -> reqwest::header::HeaderMap {

  let header_str = fs::read_to_string(".header").expect("You need to set the .header file; see readme");
  if header_str.is_empty() {
    panic!("Your .header file is empty; see readme");
  }

  let mut map = reqwest::header::HeaderMap::new();

  for header_line in header_str.lines() {
    let (key, val) = header_line.split_once(": ").expect("A header line is improperly formatted");
    let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes()).expect("Incorrect header name in header file");
    let header_value = val.parse().expect("Header value could not be parsed");
    map.append(header_name, header_value);
  }

  // Force brotli compression so we can decompress uniformly
  map.insert("Accept-Encoding", "br".parse().expect("Encoding header value could not be parsed"));

  map

}
