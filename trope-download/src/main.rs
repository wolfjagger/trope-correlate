mod arg;
mod download_pagelist;
mod header;

use clap::Parser;

use crate::{
  arg::{Args, DownloadMethodArgs},
  download_pagelist::save_pagelist
};

// Download TvTropes pages
fn main() {
  let args = Args::parse();
  match args.method {
    DownloadMethodArgs::Pagelist(method_args) => {
      save_pagelist(method_args).expect("Unhandled download_pagelist error");
    },
  }
}
