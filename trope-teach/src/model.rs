use std::path::Path;
use dfdx::numpy;


// See https://github.com/coreylowman/dfdx/blob/main/examples/07-custom-module.rs


pub struct Model<T> {
  pub some_data: T
}

impl<T: Default + dfdx::numpy::NumpyDtype + dfdx::numpy::NumpyShape + dfdx::numpy::ReadNumbers>
Model<T> {

  pub fn from_path(p: &Path) -> Result<Self, numpy::NpyError> {
    let mut data = T::default();
    numpy::load(p, &mut data)?;
    Ok(Model {
      some_data: data
    })
  }

}
