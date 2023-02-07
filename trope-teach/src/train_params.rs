use std::{
  path::Path,
  fs::read_to_string,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::TeachError;


#[derive(Deserialize, Serialize)]
pub struct TrainParams {

}

impl TrainParams {
  pub fn from_path(p: &Path) -> Result<Self, TeachError> {
    let json_str = read_to_string(p)?;
    let params = from_str(&json_str)?;
    Ok(params)
  }
}
