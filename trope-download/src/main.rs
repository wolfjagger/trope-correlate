mod download_pagelist;
mod header;

use clap::Parser;

use trope_lib::{TropeDownloadArgs, TropeDownloadMethod};
use crate::{
  download_pagelist::save_pagelist
};

// Download TvTropes pages
fn main() {
  let args = TropeDownloadArgs::parse();
  match args.method {
    TropeDownloadMethod::Pagelist(method_args) => {
      save_pagelist(method_args).expect("Unhandled download_pagelist error");
    },
  }
}
