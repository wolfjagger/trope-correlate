use dfdx::{prelude::*, gradients::Gradients};

use trope_lib::{
  EntityType, PageIdLookup, TropeTeachCategorize,
  sc_pagelist_dir, ALL_NAMESPACES
};

use crate::{InModel, OutModel, TeachError, TeachModel, TrainParams};


pub fn categorize(args: TropeTeachCategorize) -> Result<(), TeachError> {

  let TropeTeachCategorize {
    in_model: in_model_file,
    out_model: out_model_file,
    train_params: train_params_file,
    force: _force
  } = args;

  let _in_model = in_model_file.as_ref().map(
    |f| InModel::from_path(f)
  ).transpose()?;
  let out_model = OutModel::init_random();
  let _train_params = TrainParams::from_path(&train_params_file)?;

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  log::trace!(
    "{} total trope pageids, {} total media pageids",
    trope_lookup.len(), media_lookup.len()
  );

  log::info!("Assembling global trope and media pagelists...");
  let mut global_trope_pageids = vec![];
  let mut global_media_pageids = vec![];
  for ns in ALL_NAMESPACES {
    let pagelist_path = sc_pagelist_dir(&ns).join("links.csv");
    match ns.entity_type() {
      EntityType::Trope => {
        let (found_pageids, _missing) = trope_lookup.pageids_from_path(&pagelist_path)?;
        global_trope_pageids.push((ns.clone(), found_pageids));
      },
      EntityType::Media => {
        let (found_pageids, _missing) = media_lookup.pageids_from_path(&pagelist_path)?;
        global_media_pageids.push((ns.clone(), found_pageids));
      },
      _ => {}
    }
  }

  log::trace!("{:?}", global_trope_pageids);
  log::trace!("{:?}", global_media_pageids);

  log::info!("Assembling tropes...");
  // TODO: Assemble trope pageid mentions lists from tropes and media
  // let mentioned_tropes_path = sc_page_dir.join("mentioned_tropes.csv");
  // let (
  //   mentioned_trope_pageids, _missing_tropes
  // ) = assemble_pageids(&mentioned_tropes_path, &trope_lookup)?;

  log::info!("Assembling media...");
  // TODO: Assemble media pageid mentions lists from tropes and media
  // let mentioned_media_path = sc_page_dir.join("mentioned_media.csv");
  // let (
  //   mentioned_media_pageids, _missing_media
  // ) = assemble_pageids(&mentioned_media_path, &media_lookup)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  let res: Result<(), NpzError> = out_model.save(&out_model_file).map_err(NpzError::from);
  res?;

  Ok(())

}


fn _do_tensor_propagation() {

  // Look in tutorial.rs for how to do model & tensor stuff, patchy example below.
  // Assemble data files, pageid lists here and do optimization update loop.
  // Input is input data selected to train and ml training params.
  // Output is a model, in some state of training.
  // Allow feeding in new or existing models

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
