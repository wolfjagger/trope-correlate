use std::path::Path;
use dfdx::numpy;


// See https://github.com/coreylowman/dfdx/blob/main/examples/07-custom-module.rs


pub type Model<T> = WrapperModel<T>;


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
