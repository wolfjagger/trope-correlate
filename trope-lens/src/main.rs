use simple_logger::SimpleLogger;

use trope_lib::TropeLensArgs;
use trope_lens::run;


fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);
  logger.env().init().unwrap();

  let args = TropeLensArgs::parse_args();
  run(args);

}
