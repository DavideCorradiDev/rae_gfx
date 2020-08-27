extern crate gfx_hal as hal;

use hal::device::Device as HalDevice;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Backend, Gpu, RenderPass};

pub struct Framebuffer {
    value: ManuallyDrop<<Backend as hal::Backend>::Framebuffer>,
    gpu: Rc<RefCell<Gpu>>,
}

impl Framebuffer {
    pub fn create<I>(
        gpu: Rc<RefCell<Gpu>>,
        pass: &RenderPass,
        attachments: I,
        extent: hal::image::Extent,
    ) -> Result<Self, hal::device::OutOfMemory>
    where
        I: std::iter::IntoIterator,
        I::Item: std::borrow::Borrow<<Backend as hal::Backend>::ImageView>,
    {
        let framebuffer = unsafe {
            gpu.borrow()
                .device
                .create_framebuffer(pass, attachments, extent)
        }?;
        Ok(Self {
            value: ManuallyDrop::new(framebuffer),
            gpu,
        })
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.gpu.borrow().wait_idle().unwrap();
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_framebuffer(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for Framebuffer {
    type Target = <Backend as hal::Backend>::Framebuffer;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Framebuffer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Framebuffer {{ value: {:?} }}", self.value)
    }
}
