extern crate gfx_hal as hal;

use hal::device::Device as HalDevice;
use std::{
  cell::RefCell,
  fmt::{Debug, Formatter},
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use super::{Backend, Gpu, Image};

pub struct ImageView
{
  value: ManuallyDrop<<Backend as hal::Backend>::ImageView>,
  gpu: Rc<RefCell<Gpu>>,
}

impl ImageView
{
  pub fn create(
    gpu: Rc<RefCell<Gpu>>,
    image: &Image,
    view_kind: hal::image::ViewKind,
    format: hal::format::Format,
    swizzle: hal::format::Swizzle,
    range: hal::image::SubresourceRange,
  ) -> Result<Self, hal::image::ViewCreationError>
  {
    let image_view = unsafe {
      gpu
        .borrow()
        .device
        .create_image_view(image, view_kind, format, swizzle, range)
    }?;
    Ok(Self {
      value: ManuallyDrop::new(image_view),
      gpu,
    })
  }
}

impl Drop for ImageView
{
  fn drop(&mut self)
  {
    unsafe {
      self
        .gpu
        .borrow()
        .device
        .destroy_image_view(ManuallyDrop::take(&mut self.value));
    }
  }
}

impl Deref for ImageView
{
  type Target = <Backend as hal::Backend>::ImageView;
  fn deref(&self) -> &Self::Target
  {
    &self.value
  }
}

impl DerefMut for ImageView
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.value
  }
}

impl Debug for ImageView
{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
  {
    write!(f, "ImageView {{ value: {:?} }}", self.value)
  }
}
