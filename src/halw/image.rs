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

pub struct Image {
    value: ManuallyDrop<<Backend as hal::Backend>::Image>,
    gpu: Rc<RefCell<Gpu>>,
}

impl Image {
    pub fn create(
        gpu: Rc<RefCell<Gpu>>,
        kind: hal::image::Kind,
        mip_levels: hal::image::Level,
        format: hal::format::Format,
        tiling: hal::image::Tiling,
        usage: hal::image::Usage,
        view_caps: hal::image::ViewCapabilities,
    ) -> Result<Self, hal::image::CreationError> {
        let image = unsafe {
            gpu.borrow()
                .device
                .create_image(kind, mip_levels, format, tiling, usage, view_caps)
        }?;
        Ok(Self {
            value: ManuallyDrop::new(image),
            gpu,
        })
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_image(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for Image {
    type Target = <Backend as hal::Backend>::Image;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Image {{ value: {:?} }}", self.value)
    }
}
