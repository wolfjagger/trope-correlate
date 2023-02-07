mod categorize;
mod error;
mod tutorial;

use trope_lib::{TropeLensArgs, TropeLensMethod};

use categorize::categorize;
use error::LensError;
use tutorial::tutorial;

pub fn run(args: TropeLensArgs) {
  match args.method {
    TropeLensMethod::Categorize(method_args) => {
      categorize(method_args).expect("Unhandled categorize error");
    },
    TropeLensMethod::Tutorial(method_args) => {
      tutorial(method_args).expect("Unhandled tutorial error");
    },
  }
}
