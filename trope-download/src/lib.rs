mod cmd;
mod download;
mod header;

use trope_lib::{TropeDownloadArgs, TropeDownloadMethod};
pub use cmd::{
  download_namespace::save_namespace,
  download_pagelist::save_pagelist,
  download_trope::save_trope_page,
  download_tropelist::save_tropelist,
};


// Download TvTropes pages
pub fn run(args: TropeDownloadArgs) {
  match args.method {
    TropeDownloadMethod::Namespace(method_args) => {
      save_namespace(method_args).expect("Unhandled download_pagelist error");
    },
    TropeDownloadMethod::Pagelist(method_args) => {
      save_pagelist(method_args).expect("Unhandled download_pagelist error");
    },
    TropeDownloadMethod::TropePage(method_args) => {
      save_trope_page(method_args).expect("Unhandled save_trope_page error");
    },
    TropeDownloadMethod::Tropelist(method_args) => {
      save_tropelist(method_args).expect("Unhandled download_pagelist error");
    },
  }
}
