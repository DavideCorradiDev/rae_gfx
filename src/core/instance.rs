use crate::hal;

pub struct Instance
{
  instance: hal::Instance,
}

use super::InstanceCreationError;

impl Instance
{
  const ENGINE_NAME: &'static str = "Red Ape Engine";
  const ENGINE_VERSION: u32 = 1;

  pub fn create() -> Result<Self, InstanceCreationError>
  {
    let instance = Self::create_instance()?;
    Ok(Self { instance })
  }

  fn create_instance() -> Result<hal::Instance, InstanceCreationError>
  {
    let instance =
      hal::Instance::create(Self::ENGINE_NAME, Self::ENGINE_VERSION)?;
    Ok(instance)
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
