mod cmd;

use trope_lib::{TropePipelineArgs, TropePipelineMethod};
use cmd::{
  namespace_tropelist::namespace_tropelist,
};

pub fn run(args: TropePipelineArgs) {
  match args.method {
    TropePipelineMethod::NamespaceTropelist(method_args) => {
      namespace_tropelist(method_args).expect("Unhandled namespace_tropelist error");
    },
  }
}
