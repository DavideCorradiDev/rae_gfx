extern crate gfx_hal as hal;

use super::Backend;

#[derive(Debug)]
pub struct Instance
{
  value: <Backend as hal::Backend>::Instance,
}

impl Instance
{
  pub const ENGINE_VERSION: u32 = 1;
  pub const ENGINE_NAME: &'static str = "Red Ape Engine";

  pub fn new() -> Self
  {
    use hal::Instance;
    let value = <Backend as hal::Backend>::Instance::create(
      Self::ENGINE_NAME,
      Self::ENGINE_VERSION,
    )
    .unwrap();
    Self { value }
  }
}

impl std::ops::Deref for Instance
{
  type Target = <Backend as hal::Backend>::Instance;

  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl std::ops::DerefMut for Instance
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
