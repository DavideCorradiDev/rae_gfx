extern crate gfx_hal as hal;

use hal::device::Device as HalDevice;
use std::{
  cell::RefCell,
  fmt::{Debug, Formatter},
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use super::{Backend, Gpu, GraphicsPipelineDesc, PipelineCache};

pub struct GraphicsPipeline {
  value: ManuallyDrop<<Backend as hal::Backend>::GraphicsPipeline>,
  gpu: Rc<RefCell<Gpu>>,
}

impl GraphicsPipeline {
  pub fn create<'a>(
    gpu: Rc<RefCell<Gpu>>,
    desc: &GraphicsPipelineDesc<'a>,
    cache: Option<&PipelineCache>,
  ) -> Result<Self, hal::pso::CreationError> {
    let graphics_pipeline = unsafe {
      gpu
        .borrow()
        .device
        .create_graphics_pipeline(desc, cache.map(|x| x.deref()))
    }?;
    Ok(Self {
      value: ManuallyDrop::new(graphics_pipeline),
      gpu,
    })
  }
}

impl Drop for GraphicsPipeline {
  fn drop(&mut self) {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_graphics_pipeline(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for GraphicsPipeline {
  type Target = <Backend as hal::Backend>::GraphicsPipeline;
  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl DerefMut for GraphicsPipeline {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}

impl Debug for GraphicsPipeline {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "GraphicsPipeline {{ value: {:?} }}", self.value)
  }
}
