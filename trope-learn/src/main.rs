use simple_logger::SimpleLogger;

use trope_lib::TropeLearnArgs;
use trope_learn::run;


fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);
  logger.env().init().unwrap();

  let args = TropeLearnArgs::parse_args();
  run(args);

}
