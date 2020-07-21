extern crate gfx_hal as hal;

use std::{
  cell::RefCell,
  fmt::{Debug, Formatter},
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use super::{Backend, Gpu};

pub struct Memory
{
  value: ManuallyDrop<<Backend as hal::Backend>::Memory>,
  gpu: Rc<RefCell<Gpu>>,
}

impl Memory
{
  pub fn allocate(
    gpu: Rc<RefCell<Gpu>>,
    memory_type: hal::MemoryTypeId,
    size: u64,
  ) -> Result<Memory, hal::device::AllocationError>
  {
    use hal::device::Device;

    let memory =
      unsafe { gpu.borrow().device.allocate_memory(memory_type, size) }?;
    Ok(Self {
      value: ManuallyDrop::new(memory),
      gpu,
    })
  }
}

impl Drop for Memory
{
  fn drop(&mut self)
  {
    use hal::device::Device;

    unsafe {
      self
        .gpu
        .borrow()
        .device
        .free_memory(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for Memory
{
  type Target = <Backend as hal::Backend>::Memory;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for Memory
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for Memory
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "Memory {{ value: {:?} }}", self.value)
  }
}
