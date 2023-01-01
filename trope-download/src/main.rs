mod arg;
mod download_pagelist;
mod header;

// Download TvTropes pages
fn main() {
  download_pagelist::save_pagelist().unwrap();
}
