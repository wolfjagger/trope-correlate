mod cmd;

use trope_lib::{TropePipelineArgs, TropePipelineMethod};
pub use cmd::{
  namespace_tropelist::namespace_tropelist,
  all_tropelists::all_tropelists,
};

pub fn run(args: TropePipelineArgs) {
  match args.method {
    TropePipelineMethod::NamespaceTropelist(method_args) => {
      namespace_tropelist(method_args).expect("Unhandled namespace_tropelist error");
    },
    TropePipelineMethod::AllTropelists(method_args) => {
      all_tropelists(method_args).expect("Unhandled namespace_tropelist error");
    },
  }
}
