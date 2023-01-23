use simple_logger::SimpleLogger;

use trope_lib::TropePipelineArgs;
use trope_pipeline::run;

fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);
  let logger = logger.with_module_level("trope_download", log::LevelFilter::Warn);
  let logger = logger.with_module_level("trope_scrape", log::LevelFilter::Warn);
  logger.env().init().unwrap();

  let args = TropePipelineArgs::parse_args();
  run(args);

}
