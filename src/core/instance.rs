use crate::hal;

pub struct Instance
{
  instance: hal::Instance,
}

impl Instance
{
  const ENGINE_NAME: &'static str = "Red Ape Engine";
  const ENGINE_VERSION: u32 = 1;

  pub fn create() -> Self
  {
    let instance = Self::create_instance();
    Self { instance }
  }

  fn create_instance() -> hal::Instance
  {
    hal::Instance::create(Self::ENGINE_NAME, Self::ENGINE_VERSION).unwrap()
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
