use crate::hal;

pub struct Instance
{
  instance: hal::Instance,
}

use super::{Error, InstanceCreationError};

impl Instance
{
  const ENGINE_NAME: &'static str = "Red Ape Engine";
  const ENGINE_VERSION: u32 = 1;

  pub fn create() -> Result<Self, Error>
  {
    let instance = Self::create_instance()?;
    Ok(Self { instance })
  }

  fn create_instance() -> Result<hal::Instance, Error>
  {
    hal::Instance::create(Self::ENGINE_NAME, Self::ENGINE_VERSION).map_err(
      |_| {
        Error::InstanceCreationFailed(InstanceCreationError::UnsupportedBackend)
      },
    )
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn instance_creation()
  {
    let _instance = Instance::create();
  }
}
