use std::{path::Path, str::FromStr};
use dfdx::{prelude::*, gradients::Gradients};

use trope_lib::{EntityType, NamedLink, PageId, PageIdLookup, TropeTeachCategorize};

use crate::TeachError;


pub fn categorize(args: TropeTeachCategorize) -> Result<(), TeachError> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let page = args.pagename;
  log::info!("Categorizing page {}...", page);
  let sc_page_dir = trope_lib::sc_page_dir(&ns).join(&page);

  let trope_pageid_path = trope_lib::sc_pageid_path(&EntityType::Trope);
  let media_pageid_path = trope_lib::sc_pageid_path(&EntityType::Media);
  let trope_lookup = PageIdLookup::from_path(&trope_pageid_path)?;
  let media_lookup = PageIdLookup::from_path(&media_pageid_path)?;

  log::info!("Assembling tropes...");
  let mentioned_tropes_path = sc_page_dir.join("mentioned_tropes.csv");
  let (
    mentioned_trope_pageids, _missing_tropes
  ) = assemble_pageids(&mentioned_tropes_path, &trope_lookup)?;

  log::info!("Assembling media...");
  let mentioned_media_path = sc_page_dir.join("mentioned_media.csv");
  let (
    mentioned_media_pageids, _missing_media
  ) = assemble_pageids(&mentioned_media_path, &media_lookup)?;

  // Input to ML is the list of tropes and/or media
  // Output is namespace

  log::trace!(
    "{} total trope pageids, {} total media pageids",
    trope_lookup.len(), media_lookup.len()
  );
  log::trace!(
    "{} mentioned tropes, {} mentioned media",
    mentioned_trope_pageids.len(), mentioned_media_pageids.len()
  );

  Ok(())

}


fn assemble_pageids(p: &Path, page_lookup: &PageIdLookup)
-> Result<(Vec<PageId>, Vec<String>), csv::Error> {

  let mentioned_pages = path_to_page_names(&p)?;

  let (found_pages, missing_pages): (Vec<_>, Vec<_>) = mentioned_pages.into_iter().partition(
    |name| page_lookup.contains_page(&name)
  );
  let ment_page_pageids: Vec<_> = found_pages.into_iter().map(
    |name| page_lookup.pageid_from_page(&name).unwrap()
  ).collect();

  log::trace!("Found pageids:");
  for t_id in &ment_page_pageids {
    log::trace!("{}", t_id);
  }

  log::trace!("Missing pages:");
  for missing_page in &missing_pages {
    log::trace!("{}", missing_page);
  }

  Ok((ment_page_pageids, missing_pages))

}


fn path_to_page_names(p: &Path) -> Result<Vec<String>, csv::Error> {
  // Note: url is the source of truth in these mentions; short name is different page-to-page
  let mentions = csv::Reader::from_path(p)?.into_deserialize::<NamedLink>();
  mentions.map(|m_result| m_result.map(|m| m.url_page_name().to_string())).collect()
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
