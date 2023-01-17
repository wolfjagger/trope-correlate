mod cmd;

use trope_lib::{TropePipelineArgs, TropePipelineMethod};
pub use cmd::{
  namespace_pagelist::namespace_pagelist,
  all_pagelists::all_pagelists,
  namespace_pages::namespace_pages,
};

pub fn run(args: TropePipelineArgs) {
  match args.method {
    TropePipelineMethod::NamespacePagelist(method_args) => {
      namespace_pagelist(method_args).expect("Unhandled namespace_pagelist error");
    },
    TropePipelineMethod::AllPagelists(method_args) => {
      all_pagelists(method_args).expect("Unhandled all_pagelists error");
    },
    TropePipelineMethod::NamespacePages(method_args) => {
      namespace_pages(method_args).expect("Unhandled namespace_pages error");
    }
  }
}
