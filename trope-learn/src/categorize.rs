use std::{path::Path, str::FromStr};
use dfdx::{prelude::*, gradients::Gradients};
use serde::de::DeserializeOwned;

use trope_lib::{EntityType, NamedLink, PageId, PageIdLookup, TropeLearnCategorize};

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
  let trope_lookup = PageIdLookup::new(trope_pageids);
  let media_pageids = path_to_records::<PageId>(&media_pageid_path)?;
  let media_lookup = PageIdLookup::new(media_pageids);

  let mentioned_tropes = path_to_page_names(&mentioned_tropes_path)?;
  let mentioned_media = path_to_page_names(&mentioned_media_path)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  log::trace!(
    "{} trope pageids, {} media pageids",
    trope_lookup.len(), media_lookup.len()
  );
  log::trace!(
    "{} mentioned tropes, {} mentioned media",
    mentioned_tropes.len(), mentioned_media.len()
  );

  let (found_tropes, missing_tropes): (Vec<_>, Vec<_>) = mentioned_tropes.into_iter().partition(
    |name| trope_lookup.contains_page(&name)
  );
  let ment_trope_pageids: Vec<_> = found_tropes.into_iter().map(
    |name| trope_lookup.pageid_from_page(&name).unwrap()
  ).collect();

  log::info!("Found tropes:");
  for t_id in ment_trope_pageids {
    log::info!("{}", t_id);
  }

  log::info!("Missing tropes:");
  for missing_trope in missing_tropes {
    log::info!("{}", missing_trope);
  }

  let (found_media, missing_media): (Vec<_>, Vec<_>) = mentioned_media.into_iter().partition(
    |name| media_lookup.contains_page(&name)
  );
  let ment_media_pageids: Vec<_> = found_media.into_iter().map(
    |name| media_lookup.pageid_from_page(&name).unwrap()
  ).collect();

  log::info!("Found media:");
  for t_id in ment_media_pageids {
    log::info!("{}", t_id);
  }

  log::info!("Missing media:");
  for missing_media in missing_media {
    log::info!("{}", missing_media);
  }

  Ok(())

}


fn path_to_page_names(p: &Path) -> Result<Vec<String>, csv::Error> {
  // Note: url is the source of truth in these mentions; short name is different page-to-page
  let mentions = csv::Reader::from_path(p)?.into_deserialize::<NamedLink>();
  mentions.map(|m_result| m_result.map(|m| m.url_page_name().to_string())).collect()
}

fn path_to_records<Record>(p: &Path) -> Result<Vec<Record>, csv::Error>
where Record: DeserializeOwned {
  println!("Turning {} to records", p.display());
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
