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

pub struct Sampler
{
  value: ManuallyDrop<<Backend as hal::Backend>::Sampler>,
  gpu: Rc<RefCell<Gpu>>,
}

impl Sampler
{
  pub fn create(
    gpu: Rc<RefCell<Gpu>>,
    desc: &hal::image::SamplerDesc,
  ) -> Result<Self, hal::device::AllocationError>
  {
    let sampler = unsafe { gpu.borrow().device.create_sampler(desc) }?;
    Ok(Self {
      value: ManuallyDrop::new(sampler),
      gpu,
    })
  }
}

impl Drop for Sampler
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_sampler(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for Sampler
{
  type Target = <Backend as hal::Backend>::Sampler;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for Sampler
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for Sampler
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "Sampler {{ value: {:?} }}", self.value)
  }
}
