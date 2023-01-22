mod cmd;
mod download;
mod header;

use trope_lib::{TropeDownloadArgs, TropeDownloadMethod};
pub use cmd::{
  download_namespace::save_namespace,
  download_page::save_page,
  download_pagelist::save_pagelist,
};


// Download TvTropes pages
pub fn run(args: TropeDownloadArgs) {
  match args.method {
    TropeDownloadMethod::Namespace(method_args) => {
      save_namespace(method_args).expect("Unhandled download_pagelist error");
    },
    TropeDownloadMethod::Page(method_args) => {
      save_page(method_args).expect("Unhandled save_page error");
    },
    TropeDownloadMethod::Pagelist(method_args) => {
      save_pagelist(method_args).expect("Unhandled download_pagelist error");
    },
  }
}
