mod cmd;
mod error;

use trope_lib::{TropeLensArgs, TropeLensMethod};

use cmd::{
  categorize::categorize
};
use error::LensError;

pub fn run(args: TropeLensArgs) {
  match args.method {
    TropeLensMethod::Categorize(method_args) => {
      categorize(method_args).expect("Unhandled categorize error");
    },
  }
}
