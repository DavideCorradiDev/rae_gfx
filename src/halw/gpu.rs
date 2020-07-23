extern crate gfx_hal as hal;

use hal::adapter::PhysicalDevice as HalPhysicalDevice;
use std::{
  fmt::{Debug, Formatter},
  ops::{Deref, DerefMut},
};

use super::{Adapter, Backend, QueueFamily};

pub struct Gpu
{
  value: hal::adapter::Gpu<Backend>,
}

impl Gpu
{
  unsafe fn open(
    adapter: &Adapter,
    families: &[(&QueueFamily, &[hal::queue::QueuePriority])],
    requested_features: hal::Features,
  ) -> Result<Self, hal::device::CreationError>
  {
    let gpu = adapter.physical_device.open(families, requested_features)?;
    Ok(Self { value: gpu })
  }
}

impl Deref for Gpu
{
  type Target = hal::adapter::Gpu<Backend>;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for Gpu
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for Gpu
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "Gpu {{ value: {:?} }}", self.value)
  }
}