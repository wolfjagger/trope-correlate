mod cmd;
mod error;

use trope_lib::{TropeTeachArgs, TropeTeachMethod};

use cmd::{
  categorize::categorize,
  tutorial::tutorial
};
use error::TeachError;

pub fn run(args: TropeTeachArgs) {
  match args.method {
    TropeTeachMethod::Categorize(method_args) => {
      categorize(method_args).expect("Unhandled categorize error");
    },
    TropeTeachMethod::Tutorial(method_args) => {
      tutorial(method_args).expect("Unhandled tutorial error");
    },
  }
}
