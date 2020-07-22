extern crate gfx_hal as hal;
extern crate raw_window_handle;

use hal::Instance as HalInstance;
use std::{
  cell::RefCell,
  fmt::{Debug, Formatter},
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use super::{Backend, Instance};

pub struct Surface {
  value: ManuallyDrop<<Backend as hal::Backend>::Surface>,
  instance: Rc<RefCell<Instance>>,
}

impl Surface {
  pub fn create(
    instance: Rc<RefCell<Instance>>,
    handle: &impl raw_window_handle::HasRawWindowHandle,
  ) -> Result<Self, hal::window::InitError> {
    let surface = unsafe { instance.borrow().create_surface(handle) }?;
    Ok(Self {
      value: ManuallyDrop::new(surface),
      instance,
    })
  }
}

impl Drop for Surface {
  fn drop(&mut self) {
    unsafe {
      self
        .instance
        .borrow()
        .destroy_surface(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for Surface {
  type Target = <Backend as hal::Backend>::Surface;
  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl DerefMut for Surface {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}

impl Debug for Surface {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "Surface {{ value: {:?} }}", self.value)
  }
}
