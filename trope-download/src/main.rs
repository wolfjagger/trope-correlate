mod download_pagelist;
mod download_trope;
mod header;

use trope_lib::{TropeDownloadArgs, TropeDownloadMethod};
use crate::{
  download_pagelist::save_pagelist,
  download_trope::save_trope_page,
};


// Download TvTropes pages
fn main() {
  let args = TropeDownloadArgs::parse_args();
  match args.method {
    TropeDownloadMethod::Pagelist(method_args) => {
      save_pagelist(method_args).expect("Unhandled download_pagelist error");
    },
    TropeDownloadMethod::TropePage(method_args) => {
      save_trope_page(method_args).expect("Unhandled download_pagelist error");
    },
  }
}
