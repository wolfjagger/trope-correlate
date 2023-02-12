use std::{fs::read_dir, mem::size_of_val};
use dfdx::{prelude::*, gradients::Gradients};
use rand::{rngs::StdRng, SeedableRng, seq::SliceRandom};

use trope_lib::{
  EntityType, PageIdLookup, TropeTeachCategorize,
  ALL_NAMESPACES, Namespace, PageId, sc_page_dir
};

use crate::{InModel, OutModel, TeachError, TeachModel, TrainParams};


pub fn categorize(args: TropeTeachCategorize) -> Result<(), TeachError> {

  let TropeTeachCategorize {
    in_model: in_model_file,
    out_model: out_model_file,
    train_params: train_params_file,
    force: _force,
    random_seed,
  } = args;

  let in_model = in_model_file.as_ref().map(
    |f| InModel::from_path(f)
  ).transpose()?;
  let out_model = match in_model {
    Some(model) => model,
    None => OutModel::init_random()
  };
  let _train_params = TrainParams::from_path(&train_params_file)?;

  let mut rng = StdRng::seed_from_u64(random_seed);

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  log::trace!(
    "{} total trope pageids, {} total media pageids",
    trope_lookup.len(), media_lookup.len()
  );
  log::debug!("Size of trope_lookup: {}", trope_lookup.byte_size());
  log::debug!("Size of media_lookup: {}", media_lookup.byte_size());


  // // NOTE: We might not even want these! Maybe just rely on lookups for global info?
  // log::info!("Assembling global trope and media pagelists...");
  // let mut global_trope_pageids = vec![];
  // let mut global_media_pageids = vec![];
  // for ns in ALL_NAMESPACES {
  //   let pagelist_path = sc_pagelist_dir(&ns).join("links.csv");
  //   match ns.entity_type() {
  //     EntityType::Trope => {
  //       let (found_pageids, _missing) = trope_lookup.pageids_from_path(&pagelist_path)?;
  //       global_trope_pageids.push((ns.clone(), found_pageids));
  //     },
  //     EntityType::Media => {
  //       let (found_pageids, _missing) = media_lookup.pageids_from_path(&pagelist_path)?;
  //       global_media_pageids.push((ns.clone(), found_pageids));
  //     },
  //     _ => {}
  //   }
  // }

  // log::trace!("{:?}", global_trope_pageids);
  // log::trace!("{:?}", global_media_pageids);

  // log::debug!(
  //   "Size of global_trope_pageids: {}",
  //   pageids_collection_byte_size(global_trope_pageids)
  // );
  // log::debug!(
  //   "Size of global_media_pageids: {}",
  //   pageids_collection_byte_size(global_media_pageids)
  // );


  log::info!("Assembling tropes and media mention paths...");

  let sc_page_dirs: Result<Vec<_>, _> = ALL_NAMESPACES.iter().filter(
    |ns| [EntityType::Trope, EntityType::Media].contains(&ns.entity_type())
  ).map(
    |ns| read_dir(sc_page_dir(&ns)).map(|dirs| (ns.clone(), dirs))
  ).collect();
  let sc_page_dirs = sc_page_dirs.expect("Error reading some directory of page dirs");

  let mut trope_mentions = sc_page_dirs.into_iter().map(|(ns, page_dirs)|
    (
      ns,
      page_dirs.into_iter().filter_map(
        |dir| dir.map(|d| d.path()).ok()
      ).map(
        |page_dir| {
          let name = page_dir.file_name().map(
            |n| n.to_string_lossy().to_string()
          ).unwrap_or(String::new());
          (
            name,
            page_dir.join("mentioned_trope_pageid.csv"),
            page_dir.join("mentioned_media_pageid.csv")
          )
        }
      ).collect::<Vec<_>>()
    )
  ).collect::<Vec<_>>();
  trope_mentions.shuffle(&mut rng);


  let num_trope_mentions = trope_mentions.len();
  let mut train_mentions = trope_mentions;
  let test_mentions = train_mentions.split_off(
    (0.8 * num_trope_mentions as f32).floor() as usize
  );


  // Input to ML is the list of tropes and/or media
  // Output is namespace

  for train_time in 0..100 {
    println!("Train time {}", train_time);

    for (ns, ns_train_mentions) in train_mentions.iter() {
      for train_mention in ns_train_mentions {

        let (name, m_trope_path, m_media_path) = train_mention;
        log::trace!("Feeding {} into model", name);

        let mut reader = csv::Reader::from_path(m_trope_path)?;
        let mentioned_tropes: Result<Vec<trope_lib::PageId>, _> = reader.deserialize::<trope_lib::PageId>().collect();
        let mentioned_tropes = match mentioned_tropes {
          Ok(mt) => mt,
          Err(err) => {
            log::error!("Error while parsing mentioned tropes: {}", err);
            continue
          }
        };
        let mut reader = csv::Reader::from_path(m_media_path)?;
        let mentioned_media: Result<Vec<trope_lib::PageId>, _> = reader.deserialize::<trope_lib::PageId>().collect();
        let mentioned_media = match mentioned_media {
          Ok(mm) => mm,
          Err(err) => {
            log::error!("Error while parsing mentioned media: {}", err);
            continue
          }
        };

        let ml_input = (
          mentioned_tropes.iter().map(|pageid| pageid.id),
          mentioned_media.iter().map(|pageid| pageid.id)
        );

      }
    }

    // How to input the variable-length arrays to the model?
    // Sparse tensor of size 100K?

  }


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


#[allow(dead_code)]
fn pageids_collection_byte_size(collection: Vec<(Namespace, Vec<PageId>)>) -> usize {
  collection.iter().map(
    |(ns, pid_list)| size_of_val(&ns) + pid_list.iter().map(
      |pid| size_of_val(&pid.id) + size_of_val(&*pid.page)
    ).sum::<usize>()
  ).sum::<usize>()
}
