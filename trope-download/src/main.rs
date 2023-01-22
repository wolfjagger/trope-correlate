use simple_logger::SimpleLogger;

use trope_lib::TropeDownloadArgs;
use trope_download::run;


fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info).env();
  logger.init().unwrap();

  let args = TropeDownloadArgs::parse_args();
  run(args);

}
