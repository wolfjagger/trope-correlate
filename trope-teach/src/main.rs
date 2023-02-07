use simple_logger::SimpleLogger;

use trope_lib::TropeTeachArgs;
use trope_teach::run;


fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);
  logger.env().init().unwrap();

  let args = TropeTeachArgs::parse_args();
  run(args);

}
