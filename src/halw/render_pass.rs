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

pub struct RenderPass
{
  value: ManuallyDrop<<Backend as hal::Backend>::RenderPass>,
  gpu: Rc<RefCell<Gpu>>,
}

impl RenderPass
{
  pub fn create(
    gpu: Rc<RefCell<Gpu>>,
    attachments: &[hal::pass::Attachment],
    subpasses: &[hal::pass::SubpassDesc],
    subpass_dependencies: &[hal::pass::SubpassDependency],
  ) -> Result<Self, hal::device::OutOfMemory>
  {
    let render_pass = unsafe {
      gpu.borrow().device.create_render_pass(
        attachments,
        subpasses,
        subpass_dependencies,
      )
    }?;
    Ok(Self {
      value: ManuallyDrop::new(render_pass),
      gpu,
    })
  }
}

impl Drop for RenderPass
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_render_pass(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for RenderPass
{
  type Target = <Backend as hal::Backend>::RenderPass;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for RenderPass
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for RenderPass
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "RenderPass {{ value: {:?} }}", self.value)
  }
}
