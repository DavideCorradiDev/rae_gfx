extern crate gfx_hal as hal;

use super::Backend;

#[derive(Debug)]
pub struct Instance
{
  value: <Backend as hal::Backend>::Instance,
}

impl Instance
{
  pub fn new() -> Self
  {
    use hal::Instance;

    const VERSION: u32 = 1;
    const NAME: &'static str = "Red Ape Engine";
    let value =
      <Backend as hal::Backend>::Instance::create(NAME, VERSION).unwrap();
    Self { value }
  }
}
