use trope_lib::TropeDownloadArgs;
use trope_download::run;


fn main() {
  let args = TropeDownloadArgs::parse_args();
  run(args);
}
