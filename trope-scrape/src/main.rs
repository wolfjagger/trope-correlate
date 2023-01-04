use trope_lib::TropeScrapeArgs;
use trope_scrape::run;

fn main() {
  let args = TropeScrapeArgs::parse_args();
  run(args);
}
