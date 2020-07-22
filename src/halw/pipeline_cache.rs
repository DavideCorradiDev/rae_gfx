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

pub struct PipelineCache
{
  value: ManuallyDrop<<Backend as hal::Backend>::PipelineCache>,
  gpu: Rc<RefCell<Gpu>>,
}

impl PipelineCache
{
  pub fn create(
    gpu: Rc<RefCell<Gpu>>,
    data: Option<&[u8]>,
  ) -> Result<Self, hal::device::OutOfMemory>
  {
    let pipeline_cache =
      unsafe { gpu.borrow().device.create_pipeline_cache(data) }?;
    Ok(Self {
      value: ManuallyDrop::new(pipeline_cache),
      gpu,
    })
  }
}

impl Drop for PipelineCache
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_pipeline_cache(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for PipelineCache
{
  type Target = <Backend as hal::Backend>::PipelineCache;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for PipelineCache
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for PipelineCache
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "PipelineCache {{ value: {:?} }}", self.value)
  }
}
