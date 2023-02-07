// Allow dead code so we can try different compiled model structures easily
use std::path::Path;
use dfdx::{nn, numpy};


// See https://github.com/coreylowman/dfdx/blob/main/examples/07-custom-module.rs


// TODO: Figure out how to handle these types (probably should be the same)
pub type InModel = WrapperModel::<InArray>;
pub type OutModel = WrapperModel::<OutArray>;


const L0_SIZE: usize = 10;
const L1_SIZE: usize = 10000;
const L2_SIZE: usize = 30;

#[allow(dead_code)]
pub type TwoLayerReLU = (
  nn::Linear<L0_SIZE, L1_SIZE>,
  nn::ReLU,
  nn::Linear<L1_SIZE, L2_SIZE>,
);


type InArray = [f64; 3];
type OutArray = [f64; 3];

#[allow(dead_code)]
#[derive(Default)]
pub struct WrapperModel<T> {
  pub inner: T
}

impl<T> WrapperModel<T>
where T: Default + dfdx::numpy::NumpyDtype + dfdx::numpy::NumpyShape
+ dfdx::numpy::ReadNumbers + dfdx::numpy::WriteNumbers {

  pub fn from_path(p: &Path) -> Result<Self, numpy::NpyError> {
    let mut data = T::default();
    numpy::load(p, &mut data)?;
    Ok(WrapperModel {
      inner: data
    })
  }

  pub fn init_random() -> Self {
    let m = Self::default();
    // Randomize m.inner;
    m
  }

  pub fn save(&self, p: &Path) -> Result<(), numpy::NpyError> {
    numpy::save(p, &self.inner)?;
    Ok(())
  }

}
