mod categorize;
mod error;
mod tutorial;

use trope_lib::{TropeTeachArgs, TropeTeachMethod};

use categorize::categorize;
use error::TeachError;
use tutorial::tutorial;

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
