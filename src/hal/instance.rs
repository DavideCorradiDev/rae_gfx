extern crate gfx_hal as hal;

use super::{Backend, Error};

use std::ops;

#[derive(Debug)]
pub struct Instance
{
  value: <Backend as hal::Backend>::Instance,
}

impl Instance
{
  pub const ENGINE_VERSION: u32 = 1;
  pub const ENGINE_NAME: &'static str = "Red Ape Engine";

  pub fn new() -> Result<Self, Error>
  {
    use hal::Instance;
    match <Backend as hal::Backend>::Instance::create(
      Self::ENGINE_NAME,
      Self::ENGINE_VERSION,
    )
    {
      Ok(value) => Ok(Self { value }),
      Err(_) => Err(Error::UnsupportedBackend),
    }
  }
}

impl ops::Deref for Instance
{
  type Target = <Backend as hal::Backend>::Instance;

  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl ops::DerefMut for Instance
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn instance_creation()
  {
    let _instance = Instance::new();
  }
}
