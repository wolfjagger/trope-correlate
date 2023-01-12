use trope_lib::TropeLearnArgs;
use trope_learn::run;


fn main() {
  let args = TropeLearnArgs::parse_args();
  run(args);
}
