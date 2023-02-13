// Allow dead code so we can try different compiled model structures easily
#![allow(dead_code)]

use std::path::Path;
use dfdx::{nn, nn::LoadFromNpz, numpy};
use trope_lib::ALL_NAMESPACES;

use crate::TeachError;


// See https://github.com/coreylowman/dfdx/blob/main/examples/07-custom-module.rs


pub trait TeachModel {
  fn from_path(p: &Path) -> Result<Self, TeachError> where Self: Sized;
  fn init_random() -> Self;
}


pub type InModel = TwoLayerReLU;
pub type OutModel = TwoLayerReLU;


pub const MAX_NUM_TROPE: usize = 100000;
pub const MAX_NUM_MEDIA: usize = 200000;
pub const MAX_NUM_OTHER: usize = 100000;

pub const MODEL_INPUT_SIZE: usize = MAX_NUM_TROPE + MAX_NUM_MEDIA + MAX_NUM_OTHER;
pub const MODEL_OUTPUT_SIZE: usize = ALL_NAMESPACES.len();


const L0_SIZE: usize = MODEL_INPUT_SIZE;
const L1_SIZE: usize = 10000;
const L2_SIZE: usize = MODEL_OUTPUT_SIZE;

pub type TwoLayerReLU = (
  nn::Linear<L0_SIZE, L1_SIZE>,
  nn::ReLU,
  nn::Linear<L1_SIZE, L2_SIZE>,
);

impl TeachModel for TwoLayerReLU {

  fn from_path(p: &Path) -> Result<Self, TeachError> {
    let mut data = TwoLayerReLU::default();
    data.load(p)?;
    Ok(data)
  }

  fn init_random() -> Self {
    let m = Self::default();
    // Randomize m.inner;
    m
  }

}


type InArray = [f64; 3];
type OutArray = [f64; 3];

#[derive(Default)]
pub struct WrapperModel<T> {
  pub inner: T
}

impl<T> TeachModel for WrapperModel<T>
where T: Default + dfdx::numpy::NumpyDtype + dfdx::numpy::NumpyShape
+ dfdx::numpy::ReadNumbers {

  fn from_path(p: &Path) -> Result<Self, TeachError> {
    let mut data = T::default();
    numpy::load(p, &mut data).map_err(nn::NpzError::from)?;
    Ok(WrapperModel {
      inner: data
    })
  }

  fn init_random() -> Self {
    let m = Self::default();
    // Randomize m.inner;
    m
  }

}

impl<T> WrapperModel<T>
where T: dfdx::numpy::NumpyDtype + dfdx::numpy::NumpyShape + dfdx::numpy::WriteNumbers {
  pub fn save(&self, p: &Path) -> Result<(), numpy::NpyError> {
    numpy::save(p, &self.inner)?;
    Ok(())
  }
}
