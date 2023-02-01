mod categorize;
mod error;
mod tutorial;

use trope_lib::{TropeLearnArgs, TropeLearnMethod};

use categorize::categorize;
use error::LearnError;
use tutorial::tutorial;

pub fn run(args: TropeLearnArgs) {
  match args.method {
    TropeLearnMethod::Categorize(method_args) => {
      categorize(method_args).expect("Unhandled categorize error");
    },
    TropeLearnMethod::Tutorial(method_args) => {
      tutorial(method_args).expect("Unhandled tutorial error");
    },
  }
}
