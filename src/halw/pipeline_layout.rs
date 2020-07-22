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

pub struct PipelineLayout
{
  value: ManuallyDrop<<Backend as hal::Backend>::PipelineLayout>,
  gpu: Rc<RefCell<Gpu>>,
}

impl PipelineLayout
{
  pub fn create<IS, IR>(
    gpu: Rc<RefCell<Gpu>>,
    set_layouts: IS,
    push_constants: IR,
  ) -> Result<Self, hal::device::OutOfMemory>
  where
    IS: std::iter::IntoIterator,
    IS::Item:
      core::borrow::Borrow<<Backend as hal::Backend>::DescriptorSetLayout>,
    IR: std::iter::IntoIterator,
    IR::Item:
      core::borrow::Borrow<(hal::pso::ShaderStageFlags, core::ops::Range<u32>)>,
  {
    let pipeline_layout = unsafe {
      gpu
        .borrow()
        .device
        .create_pipeline_layout(set_layouts, push_constants)
    }?;
    Ok(Self {
      value: ManuallyDrop::new(pipeline_layout),
      gpu,
    })
  }
}

impl Drop for PipelineLayout
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_pipeline_layout(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for PipelineLayout
{
  type Target = <Backend as hal::Backend>::PipelineLayout;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for PipelineLayout
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for PipelineLayout
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "PipelineLayout {{ value: {:?} }}", self.value)
  }
}
