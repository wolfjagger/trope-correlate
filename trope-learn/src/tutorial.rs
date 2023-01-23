use dfdx::{prelude::*, gradients::Gradients};
use rand::{rngs::SmallRng, SeedableRng};

use trope_lib::TropeLearnTutorial;


pub fn tutorial(_args: TropeLearnTutorial) -> Result<(), Box<dyn std::error::Error>> {
  tutorial1();
  tutorial2();
  tutorial3();
  tutorial4();
  tutorial5();
  tutorial6();
  tutorial7();
  Ok(())
}

fn tutorial1() {
  log::info!("tutorial1");

  // 1. crate::tensor::Tensors can be created with normal rust arrays. See crate::tensor.

  let x = tensor([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
  let y: Tensor2D<2, 3> = TensorCreator::ones();
  log::info!("x: {:?}, y: {:?}", x, y);

}

fn tutorial2() {
  log::info!("tutorial2");

  // 2. Neural networks are built with types. Tuples are sequential models. See crate::nn.

  type _Mlp = (
    Linear<5, 3>,
    ReLU,
    Linear<3, 2>,
  );

}

fn tutorial3() {
  log::info!("tutorial3");

  // 3. Instantiate models with Default, and randomize with crate::nn::ResetParams

  let seed = 4321;
  let mut rng = SmallRng::seed_from_u64(seed);

  let mut mlp: Linear<5, 2> = Default::default();
  mlp.reset_params(&mut rng);

  log::info!("mlp: {:?}", mlp);

}

fn tutorial4() {
  log::info!("tutorial4");

  // 4. Pass data through networks with crate::nn::Module

  let mlp: Linear<5, 2> = Default::default();
  let x = Tensor1D::zeros(); // compiler knows that x is a `Tensor1D<5>`
  log::info!("x: {:?}", x);
  let y = mlp.forward(x); // compiler knows that `y` must be `Tensor1D<2>`
  log::info!("y: {:?}", y);

}

fn tutorial5() {
  log::info!("tutorial5");

  // 5. Trace gradients using crate::tensor::trace()

  type MLP = (
      (Linear<10, 32>, ReLU),
      (Linear<32, 32>, ReLU),
      (Linear<32, 5>, Tanh),
  );
  let mlp: MLP = Default::default();

  // tensors default to not having a tape
  let x: Tensor1D<10, NoneTape> = TensorCreator::zeros();
  log::info!("x: {:?}", x);

  // `.trace()` clones `x` and inserts a gradient tape.
  let x_t: Tensor1D<10, OwnedTape> = x.trace();
  log::info!("x_t: {:?}", x_t);

  // The tape from the input is moved through the network during .forward().
  let y: Tensor1D<5, NoneTape> = mlp.forward(x);
  log::info!("y: {:?}", y);
  let y_t: Tensor1D<5, OwnedTape> = mlp.forward(x_t);
  log::info!("y_t: {:?}", y_t);

}

fn tutorial6() {
  log::info!("tutorial6");

  // 6. Compute gradients with crate::tensor_ops::backward(). See crate::tensor_ops.

  type MLP = (
      (Linear<10, 32>, ReLU),
      (Linear<32, 32>, ReLU),
      (Linear<32, 5>, Tanh),
  );
  let mlp: MLP = Default::default();

  // tensors default to not having a tape
  let x: Tensor1D<10, NoneTape> = TensorCreator::zeros();
  let x_t = x.traced();

  // The tape from the input is moved through the network during .forward().
  let y: Tensor1D<5, OwnedTape> = mlp.forward(x_t);
  let y_true: Tensor1D<5, NoneTape> = Default::default();

  // compute cross entropy loss
  let loss: Tensor0D<OwnedTape> = cross_entropy_with_logits_loss(y, y_true);

  // call `backward()` to compute gradients. The tensor *must* have `OwnedTape`!
  let gradients: Gradients = loss.backward();

  log::info!("gradients: {:?}", gradients);

}

fn tutorial7() {
  log::info!("tutorial7");

  // 7. Use an optimizer from crate::optim to optimize your network!

  type MLP = (
      (Linear<10, 32>, ReLU),
      (Linear<32, 32>, ReLU),
      (Linear<32, 5>, Tanh),
  );
  let mut mlp: MLP = Default::default();

  // tensors default to not having a tape
  let x: Tensor1D<10, NoneTape> = TensorCreator::zeros();
  log::info!("x: {:?}", x);
  let x_t = x.traced();
  log::info!("x_t: {:?}", x_t);

  // The tape from the input is moved through the network during .forward().
  let y: Tensor1D<5, OwnedTape> = mlp.forward(x_t);
  log::info!("y: {:?}", y);
  let y_true: Tensor1D<5, NoneTape> = Default::default();
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
