mod cmd;
mod error;
mod model;
mod train_params;

use trope_lib::{TropeTeachArgs, TropeTeachMethod};

use cmd::{
  categorize::categorize,
  tutorial::tutorial
};
use error::TeachError;
use train_params::TrainParams;

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
