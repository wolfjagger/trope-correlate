mod tutorial;

use trope_lib::{TropeLearnArgs, TropeLearnMethod};
use tutorial::tutorial;

pub fn run(args: TropeLearnArgs) {
  match args.method {
    TropeLearnMethod::Tutorial(method_args) => {
      tutorial(method_args).expect("Unhandled tutorial error");
    },
  }
}
