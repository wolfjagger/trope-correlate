use simple_logger::SimpleLogger;

use trope_lib::TropeScrapeArgs;
use trope_scrape::run;

fn main() {

  let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);
  let logger = logger.with_module_level("selectors", log::LevelFilter::Info);
  let logger = logger.with_module_level("html5ever", log::LevelFilter::Info);
  logger.env().init().unwrap();

  let args = TropeScrapeArgs::parse_args();
  run(args);

}
