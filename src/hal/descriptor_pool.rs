extern crate gfx_hal as hal;

use hal::device::Device as HalDevice;
use std::{
  cell::RefCell,
  fmt::{Debug, Formatter},
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use super::{Backend, Gpu};

pub struct DescriptorPool
{
  value: ManuallyDrop<<Backend as hal::Backend>::DescriptorPool>,
  gpu: Rc<RefCell<Gpu>>,
}

impl DescriptorPool
{
  pub fn new<I>(
    gpu: Rc<RefCell<Gpu>>,
    max_sets: usize,
    descriptor_ranges: I,
    flags: hal::pso::DescriptorPoolCreateFlags,
  ) -> Result<Self, hal::device::OutOfMemory>
  where
    I: std::iter::IntoIterator,
    I::Item: std::borrow::Borrow<hal::pso::DescriptorRangeDesc>,
  {
    let dsl = unsafe {
      gpu.borrow().device.create_descriptor_pool(
        max_sets,
        descriptor_ranges,
        flags,
      )
    }?;
    Ok(Self {
      value: ManuallyDrop::new(dsl),
      gpu,
    })
  }
}

impl Drop for DescriptorPool
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_descriptor_pool(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for DescriptorPool
{
  type Target = <Backend as hal::Backend>::DescriptorPool;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for DescriptorPool
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for DescriptorPool
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "DescriptorPool {{ value: {:?} }}", self.value)
  }
}
