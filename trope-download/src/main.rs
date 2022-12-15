mod arg;
mod download_pagelist;
mod header;

use crate::{arg::Args, download_pagelist::save_pagelist};

// Download TvTropes pages
fn main() {
  let args = Args::parse_args();
  // TODO: Translate Args to more specific PagelistArgs based on "method" of download
  save_pagelist(args).expect("Unhandled download_pagelist error");
}
