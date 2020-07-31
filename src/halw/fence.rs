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

pub struct Fence
{
  value: ManuallyDrop<<Backend as hal::Backend>::Fence>,
  gpu: Rc<RefCell<Gpu>>,
}

impl Fence
{
  pub fn create(
    gpu: Rc<RefCell<Gpu>>,
    signaled: bool,
  ) -> Result<Self, hal::device::OutOfMemory>
  {
    let fence = gpu.borrow().device.create_fence(signaled)?;
    Ok(Self {
      value: ManuallyDrop::new(fence),
      gpu,
    })
  }

  pub fn wait(
    &self,
    timeout_ns: u64,
  ) -> Result<bool, hal::device::OomOrDeviceLost>
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .wait_for_fence(&self.value, timeout_ns)
    }
  }

  pub fn reset(&self) -> Result<(), hal::device::OutOfMemory>
  {
    unsafe { self.gpu.borrow().device.reset_fence(&self.value) }
  }
}

impl Drop for Fence
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_fence(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for Fence
{
  type Target = <Backend as hal::Backend>::Fence;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for Fence
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for Fence
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "Fence {{ value: {:?} }}", self.value)
  }
}
