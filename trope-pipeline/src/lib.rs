mod cmd;

use trope_lib::{TropePipelineArgs, TropePipelineMethod};
pub use cmd::{
  namespace_pagelist::namespace_pagelist,
  all_pagelists::all_pagelists,
};

pub fn run(args: TropePipelineArgs) {
  match args.method {
    TropePipelineMethod::NamespacePagelist(method_args) => {
      namespace_pagelist(method_args).expect("Unhandled namespace_pagelist error");
    },
    TropePipelineMethod::AllPagelists(method_args) => {
      all_pagelists(method_args).expect("Unhandled all_pagelists error");
    },
  }
}
