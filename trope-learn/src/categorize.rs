use std::str::FromStr;
use dfdx::{prelude::*, gradients::Gradients};

use trope_lib::{TropeLearnCategorize, NamedLink};

use crate::LearnError;


pub fn categorize(args: TropeLearnCategorize) -> Result<(), LearnError> {

  let ns = trope_lib::Namespace::from_str(&args.namespace)?;

  let sc_dir = trope_lib::sc_page_dir(&ns).join(&args.pagename);

  let page = 1;
  let page_str = page.to_string();

  log::info!("Categorizing page {}...", page_str);

  let mentioned_tropes_path = sc_dir.join("mentioned_tropes.csv");
  let mentioned_media_path = sc_dir.join("mentioned_media.csv");
  let mentioned_tropes = csv::Reader::from_path(&mentioned_tropes_path)?.into_deserialize();
  let mentioned_media = csv::Reader::from_path(&mentioned_media_path)?.into_deserialize();

  let tropes = match mentioned_tropes.collect::<Result<Vec<NamedLink>, _>>() {
    Ok(t) => t,
    Err(err) => {
      log::error!("Cannot parse trope, {}", err);
      return Err(err.into());
    }
  };
  let medias = match mentioned_media.collect::<Result<Vec<NamedLink>, _>>() {
    Ok(m) => m,
    Err(err) => {
      log::error!("Cannot parse media, {}", err);
      return Err(err.into());
    }
  };

  println!(
    "{} mentioned tropes, {} mentioned media",
    medias.len(), tropes.len()
  );

  Ok(())

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
