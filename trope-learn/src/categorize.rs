use std::{path::Path, str::FromStr};
use dfdx::{prelude::*, gradients::Gradients};
use serde::de::DeserializeOwned;

use trope_lib::{EntityType, NamedLink, PageId, TropeLearnCategorize};

use crate::LearnError;


pub fn categorize(args: TropeLearnCategorize) -> Result<(), LearnError> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let page = args.pagename;
  log::info!("Categorizing page {}...", page);
  let sc_page_dir = trope_lib::sc_page_dir(&ns).join(&page);

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let mentioned_tropes_path = sc_page_dir.join("mentioned_tropes.csv");
  let mentioned_media_path = sc_page_dir.join("mentioned_media.csv");

  let trope_pageids = path_to_records::<PageId>(&trope_pageid_path)?;
  let media_pageids = path_to_records::<PageId>(&media_pageid_path)?;
  let mentioned_tropes = path_to_records::<NamedLink>(&mentioned_tropes_path)?;
  let mentioned_media = path_to_records::<NamedLink>(&mentioned_media_path)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  println!(
    "{} trope pageids, {} media pageids",
    trope_pageids.len(), media_pageids.len()
  );
  println!(
    "{} mentioned tropes, {} mentioned media",
    mentioned_tropes.len(), mentioned_media.len()
  );

  Ok(())

}


fn path_to_records<Record>(p: &Path) -> Result<Vec<Record>, csv::Error>
where Record: DeserializeOwned {
  println!("{}", p.display());
  csv::Reader::from_path(p)?.into_deserialize().collect::<Result<Vec<_>, _>>()
}


fn _do_tensor_propagation() {

  type MLP = (
    (Linear<10, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    (Linear<32, 5>, Tanh),
  );

  // 7. Use an optimizer from crate::optim to optimize your network!

  let mut mlp: MLP = Default::default();

  // tensors default to not having a tape
  let x: Tensor1D<10, NoneTape> = TensorCreator::zeros();
  let x = x.traced();
  log::info!("x: {:?}", x);

  // The tape from the input is moved through the network during .forward().
  let y: Tensor1D<5, OwnedTape> = mlp.forward(x);
  log::info!("y: {:?}", y);
  let y_true = tensor([1.0, 2.0, 3.0, 4.0, 5.0]);
  log::info!("y: {:?}", y_true);

  // compute cross entropy loss
  let loss: Tensor0D<OwnedTape> = cross_entropy_with_logits_loss(y, y_true);
  log::info!("loss: {:?}", loss);

  // call `backward()` to compute gradients. The tensor *must* have `OwnedTape`!
  let gradients: Gradients = loss.backward();
  log::info!("gradients: {:?}", gradients);

  // Use stochastic gradient descent (Sgd), with a learning rate of 1e-2, and 0.9 momentum.
  let mut opt = Sgd::new(SgdConfig {
    lr: 1e-2,
    momentum: Some(Momentum::Classic(0.9)),
    weight_decay: None,
  });

  // pass the gradients & the model into the optimizer's update method
  opt.update(&mut mlp, gradients).unwrap();
  log::info!("opt: {:?}", opt);

}
