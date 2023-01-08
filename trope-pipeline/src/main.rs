use trope_lib::TropePipelineArgs;
use trope_pipeline::run;

fn main() {
  let args = TropePipelineArgs::parse_args();
  run(args);
}
