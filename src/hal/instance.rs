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
  pub fn create(name: &str, version: u32) -> Result<Self, Error>
  {
    use hal::Instance;
    match <Backend as hal::Backend>::Instance::create(name, version)
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
    let _instance = Instance::create("Name", 42);
  }
}
