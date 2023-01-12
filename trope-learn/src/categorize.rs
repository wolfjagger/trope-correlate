// use dfdx::{prelude::*, gradients::Gradients};
// use rand::{rngs::SmallRng, SeedableRng};

use trope_lib::TropeLearnCategorize;


pub fn categorize(args: TropeLearnCategorize) -> Result<(), Box<dyn std::error::Error>> {
  println!("{:?}", args);
  Ok(())
}